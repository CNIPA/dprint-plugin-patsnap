use super::ast::*;
use super::lexer::Lexer;
use super::token::{Span, Token, TokenKind};

/// Parse error with location information.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Parse result for a .patsnap file.
pub struct ParseResult {
    pub file: File,
    pub errors: Vec<ParseError>,
}

/// Parser for Patsnap search query syntax.
/// Uses a Pratt parser (precedence climbing) approach.
pub struct Parser<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<ParseError>,
}

/// Operator precedence levels (higher = binds tighter).
const PREC_OR: u8 = 1;
const PREC_AND: u8 = 2;
const PREC_PROXIMITY: u8 = 4;

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        Self {
            source,
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    /// Parse the entire file into a File AST node.
    pub fn parse(mut self) -> ParseResult {
        let mut statements = Vec::new();

        self.consume_newlines();

        while !self.at_eof() {
            // Skip newlines and detect blank lines
            if self.peek_kind() == TokenKind::Newline {
                let newline_count = self.consume_newlines();
                if newline_count >= 2 {
                    statements.push(Statement::BlankLine);
                }
                continue;
            }

            // Check for standalone comment
            if self.peek_kind() == TokenKind::LineComment {
                let tok = self.advance();
                statements.push(Statement::Comment(Comment {
                    text: tok.text(self.source).to_string(),
                    span: tok.span,
                }));
                continue;
            }

            // Parse a query expression
            match self.parse_query_expr(0) {
                Some(expr) => {
                    statements.push(Statement::Query(expr));
                }
                None => {
                    // Skip to next line on error
                    self.skip_to_next_line();
                }
            }
        }

        ParseResult {
            file: File { statements },
            errors: self.errors,
        }
    }

    // ── Pratt parser core ──

    /// Parse an expression with the given minimum precedence.
    fn parse_query_expr(&mut self, min_prec: u8) -> Option<QueryExpr> {
        let mut left = self.parse_prefix()?;

        loop {
            // Skip newlines between operands in a query (not blank lines)
            let saved = self.pos;
            self.skip_non_blank_newlines();

            if let Some((op, prec, op_span)) = self.peek_infix_op() {
                if prec < min_prec {
                    self.pos = saved;
                    break;
                }
                self.advance(); // consume the operator token

                // Skip newlines after operator
                self.skip_non_blank_newlines();

                let right = match self.parse_query_expr(prec + 1) {
                    Some(r) => r,
                    None => {
                        self.error_at_current("expected expression after operator");
                        break;
                    }
                };

                left = match op {
                    InfixOp::Bool(bool_op) => QueryExpr::Binary(BinaryExpr {
                        left: Box::new(left),
                        op: bool_op,
                        op_span,
                        right: Box::new(right),
                    }),
                    InfixOp::Proximity(op_str) => QueryExpr::Proximity(ProximityExpr {
                        left: Box::new(left),
                        op: op_str,
                        op_span,
                        right: Box::new(right),
                    }),
                };
            } else {
                // Check for implicit AND (two atoms next to each other without operator)
                // This handles: 太阳能 电池 (space means AND)
                // Also handles: TTL:a NOT ABST:b (NOT as prefix in implicit AND)
                if (self.is_atom_start() || self.peek_kind() == TokenKind::Not) && min_prec <= PREC_AND {
                    let right = match self.parse_query_expr(PREC_AND + 1) {
                        Some(r) => r,
                        None => {
                            self.pos = saved;
                            break;
                        }
                    };
                    let implicit_span = Span::new(
                        left.span().end,
                        right.span().start,
                    );
                    left = QueryExpr::Binary(BinaryExpr {
                        left: Box::new(left),
                        op: BoolOp::And,
                        op_span: implicit_span,
                        right: Box::new(right),
                    });
                } else {
                    self.pos = saved;
                    break;
                }
            }
        }

        // Check for frequency operator (postfix)
        if self.peek_kind() == TokenKind::FrequencyOp {
            let tok = self.advance();
            left = QueryExpr::Frequency(FrequencyExpr {
                operand: Box::new(left),
                op: tok.text(self.source).to_string(),
                op_span: tok.span,
            });
        }

        Some(left)
    }

    /// Parse a prefix expression (atom or unary NOT).
    fn parse_prefix(&mut self) -> Option<QueryExpr> {
        match self.peek_kind() {
            TokenKind::Not => {
                let tok = self.advance();
                self.skip_non_blank_newlines();
                let operand = self.parse_query_expr(PREC_AND + 1)?;
                Some(QueryExpr::Not(NotExpr {
                    op_span: tok.span,
                    operand: Box::new(operand),
                }))
            }
            _ => self.parse_atom(),
        }
    }

    /// Parse an atomic expression.
    fn parse_atom(&mut self) -> Option<QueryExpr> {
        match self.peek_kind() {
            TokenKind::FieldCode => self.parse_field_expr(),
            TokenKind::LParen => self.parse_group_expr(),
            TokenKind::LBracket => self.parse_range_expr(),
            TokenKind::QuotedString => self.parse_quoted_term(),
            TokenKind::Keyword => self.parse_keyword_term(),
            TokenKind::TreeAt => self.parse_tree_at_expr(),
            _ => {
                self.error_at_current("unexpected token");
                None
            }
        }
    }

    fn parse_field_expr(&mut self) -> Option<QueryExpr> {
        let field_tok = self.advance(); // FieldCode
        let field_name = field_tok.text(self.source).to_string();
        let field_span = field_tok.span;

        if self.peek_kind() != TokenKind::Colon {
            self.error_at_current("expected ':' after field code");
            return None;
        }
        let colon_tok = self.advance(); // Colon

        let body = if self.peek_kind() == TokenKind::LParen {
            let lparen = self.advance();
            self.skip_non_blank_newlines();
            let inner = self.parse_query_expr(0).unwrap_or(QueryExpr::Error(ErrorNode {
                raw_text: String::new(),
                span: Span::new(lparen.span.end, lparen.span.end),
            }));
            self.skip_non_blank_newlines();
            if self.peek_kind() != TokenKind::RParen {
                self.error_at_current("expected ')' to close field expression");
                // Try to recover
            }
            let rparen_span = if self.peek_kind() == TokenKind::RParen {
                self.advance().span
            } else {
                Span::new(self.current_byte_pos(), self.current_byte_pos())
            };
            FieldBody::Parenthesized {
                lparen_span: lparen.span,
                inner: Box::new(inner),
                rparen_span,
            }
        } else if self.peek_kind() == TokenKind::LBracket {
            // Range expression: FIELD:[from TO to]
            let range = self.parse_range_expr()?;
            FieldBody::Simple(Box::new(range))
        } else {
            // Simple value: FIELD:value or FIELD:"quoted"
            let value = self.parse_atom()?;
            FieldBody::Simple(Box::new(value))
        };

        Some(QueryExpr::Field(FieldExpr {
            field_name,
            field_span,
            colon_span: colon_tok.span,
            body,
        }))
    }

    fn parse_group_expr(&mut self) -> Option<QueryExpr> {
        let lparen = self.advance(); // LParen
        self.skip_non_blank_newlines();
        let inner = self.parse_query_expr(0).unwrap_or(QueryExpr::Error(ErrorNode {
            raw_text: String::new(),
            span: Span::new(lparen.span.end, lparen.span.end),
        }));
        self.skip_non_blank_newlines();
        if self.peek_kind() != TokenKind::RParen {
            self.error_at_current("expected ')'");
        }
        let rparen_span = if self.peek_kind() == TokenKind::RParen {
            self.advance().span
        } else {
            Span::new(self.current_byte_pos(), self.current_byte_pos())
        };
        Some(QueryExpr::Group(GroupExpr {
            lparen_span: lparen.span,
            inner: Box::new(inner),
            rparen_span,
        }))
    }

    fn parse_range_expr(&mut self) -> Option<QueryExpr> {
        let lbracket = self.advance(); // LBracket
        let from_tok = self.expect_keyword_or_wildcard("expected range start value")?;
        let from = from_tok.text(self.source).to_string();
        let from_span = from_tok.span;

        if self.peek_kind() != TokenKind::To {
            self.error_at_current("expected 'TO' in range expression");
            return None;
        }
        let to_kw = self.advance(); // TO

        let to_tok = self.expect_keyword_or_wildcard("expected range end value")?;
        let to = to_tok.text(self.source).to_string();
        let to_span = to_tok.span;

        if self.peek_kind() != TokenKind::RBracket {
            self.error_at_current("expected ']' to close range");
        }
        let rbracket_span = if self.peek_kind() == TokenKind::RBracket {
            self.advance().span
        } else {
            Span::new(self.current_byte_pos(), self.current_byte_pos())
        };

        Some(QueryExpr::Range(RangeExpr {
            lbracket_span: lbracket.span,
            from,
            from_span,
            to_keyword_span: to_kw.span,
            to,
            to_span,
            rbracket_span,
        }))
    }

    fn parse_quoted_term(&mut self) -> Option<QueryExpr> {
        let tok = self.advance();
        let raw = tok.text(self.source);
        // Extract inner value without quotes
        let quote_char = raw.chars().next().unwrap_or('"');
        let value = if raw.len() >= 2 {
            raw[1..raw.len() - 1].to_string()
        } else {
            raw.to_string()
        };
        Some(QueryExpr::Quoted(QuotedTerm {
            value,
            quote_char,
            span: tok.span,
        }))
    }

    fn parse_keyword_term(&mut self) -> Option<QueryExpr> {
        let tok = self.advance();
        Some(QueryExpr::Keyword(KeywordTerm {
            value: tok.text(self.source).to_string(),
            span: tok.span,
        }))
    }

    fn parse_tree_at_expr(&mut self) -> Option<QueryExpr> {
        let tree_at_tok = self.advance(); // TREE@
        let operand = self.parse_atom()?;
        Some(QueryExpr::TreeAt(TreeAtExpr {
            tree_at_span: tree_at_tok.span,
            operand: Box::new(operand),
        }))
    }

    // ── Helper methods ──

    fn peek_kind(&self) -> TokenKind {
        self.tokens
            .get(self.pos)
            .map(|t| t.kind.clone())
            .unwrap_or(TokenKind::Eof)
    }

    fn peek_token(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn at_eof(&self) -> bool {
        self.peek_kind() == TokenKind::Eof
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn current_byte_pos(&self) -> usize {
        self.peek_token().span.start
    }

    fn skip_newline(&mut self) {
        if self.peek_kind() == TokenKind::Newline {
            self.advance();
        }
    }

    fn skip_non_blank_newlines(&mut self) {
        // Skip single newlines but stop at blank lines (two consecutive newlines).
        while self.peek_kind() == TokenKind::Newline {
            // Check if next is also a newline (blank line)
            let next_pos = self.pos + 1;
            if next_pos < self.tokens.len() && self.tokens[next_pos].kind == TokenKind::Newline {
                break; // blank line — don't consume
            }
            self.advance();
        }
    }

    /// Consume all consecutive newlines, return how many were consumed.
    fn consume_newlines(&mut self) -> usize {
        let mut count = 0;
        while self.peek_kind() == TokenKind::Newline {
            self.advance();
            count += 1;
        }
        count
    }

    fn skip_to_next_line(&mut self) {
        while !self.at_eof() && self.peek_kind() != TokenKind::Newline {
            self.advance();
        }
        self.skip_newline();
    }

    fn is_atom_start(&self) -> bool {
        matches!(
            self.peek_kind(),
            TokenKind::FieldCode
                | TokenKind::Keyword
                | TokenKind::QuotedString
                | TokenKind::LParen
                | TokenKind::LBracket
                | TokenKind::TreeAt
        )
    }

    fn peek_infix_op(&self) -> Option<(InfixOp, u8, Span)> {
        let tok = self.peek_token();
        match &tok.kind {
            TokenKind::And => Some((InfixOp::Bool(BoolOp::And), PREC_AND, tok.span)),
            TokenKind::Or => Some((InfixOp::Bool(BoolOp::Or), PREC_OR, tok.span)),
            TokenKind::Gand => Some((InfixOp::Bool(BoolOp::Gand), PREC_AND, tok.span)),
            TokenKind::ProximityOp => {
                let op_str = tok.text(self.source).to_string();
                Some((InfixOp::Proximity(op_str), PREC_PROXIMITY, tok.span))
            }
            _ => None,
        }
    }

    fn expect_keyword_or_wildcard(&mut self, msg: &str) -> Option<Token> {
        match self.peek_kind() {
            TokenKind::Keyword | TokenKind::QuotedString => Some(self.advance()),
            _ => {
                self.error_at_current(msg);
                None
            }
        }
    }

    fn error_at_current(&mut self, message: &str) {
        let span = self.peek_token().span;
        self.errors.push(ParseError {
            message: message.to_string(),
            span,
        });
    }
}

enum InfixOp {
    Bool(BoolOp),
    Proximity(String),
}

// ── Span helpers for QueryExpr ──

impl QueryExpr {
    pub fn span(&self) -> Span {
        match self {
            QueryExpr::Binary(e) => Span::new(e.left.span().start, e.right.span().end),
            QueryExpr::Not(e) => Span::new(e.op_span.start, e.operand.span().end),
            QueryExpr::Field(e) => {
                let end = match &e.body {
                    FieldBody::Simple(inner) => inner.span().end,
                    FieldBody::Parenthesized { rparen_span, .. } => rparen_span.end,
                };
                Span::new(e.field_span.start, end)
            }
            QueryExpr::Group(e) => Span::new(e.lparen_span.start, e.rparen_span.end),
            QueryExpr::Keyword(e) => e.span,
            QueryExpr::Quoted(e) => e.span,
            QueryExpr::Range(e) => Span::new(e.lbracket_span.start, e.rbracket_span.end),
            QueryExpr::Proximity(e) => Span::new(e.left.span().start, e.right.span().end),
            QueryExpr::Frequency(e) => Span::new(e.operand.span().start, e.op_span.end),
            QueryExpr::TreeAt(e) => Span::new(e.tree_at_span.start, e.operand.span().end),
            QueryExpr::Error(e) => e.span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> ParseResult {
        Parser::new(input).parse()
    }

    fn assert_no_errors(result: &ParseResult) {
        if !result.errors.is_empty() {
            panic!(
                "expected no parse errors, got: {:?}",
                result.errors
            );
        }
    }

    #[test]
    fn simple_keyword() {
        let result = parse("汽车");
        assert_no_errors(&result);
        assert_eq!(result.file.statements.len(), 1);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Keyword(k)) => {
                assert_eq!(k.value, "汽车");
            }
            other => panic!("expected keyword, got {:?}", other),
        }
    }

    #[test]
    fn field_with_simple_value() {
        let result = parse("TTL:汽车");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Field(f)) => {
                assert_eq!(f.field_name, "TTL");
                match &f.body {
                    FieldBody::Simple(inner) => match inner.as_ref() {
                        QueryExpr::Keyword(k) => assert_eq!(k.value, "汽车"),
                        other => panic!("expected keyword, got {:?}", other),
                    },
                    _ => panic!("expected simple body"),
                }
            }
            other => panic!("expected field expr, got {:?}", other),
        }
    }

    #[test]
    fn field_with_parenthesized_or() {
        let result = parse("tac:(空调 or 蒸发器)");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Field(f)) => {
                assert_eq!(f.field_name, "tac");
                match &f.body {
                    FieldBody::Parenthesized { inner, .. } => match inner.as_ref() {
                        QueryExpr::Binary(b) => {
                            assert_eq!(b.op, BoolOp::Or);
                        }
                        other => panic!("expected binary, got {:?}", other),
                    },
                    _ => panic!("expected parenthesized body"),
                }
            }
            other => panic!("expected field expr, got {:?}", other),
        }
    }

    #[test]
    fn binary_and_or() {
        let result = parse("TTL:a and ABST:b or AN:c");
        assert_no_errors(&result);
        // Should parse as: (TTL:a AND ABST:b) OR AN:c  (AND binds tighter than OR)
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Binary(outer)) => {
                assert_eq!(outer.op, BoolOp::Or);
                match outer.left.as_ref() {
                    QueryExpr::Binary(inner) => {
                        assert_eq!(inner.op, BoolOp::And);
                    }
                    other => panic!("expected inner AND, got {:?}", other),
                }
            }
            other => panic!("expected binary, got {:?}", other),
        }
    }

    #[test]
    fn not_operator() {
        let result = parse("TTL:a not ABST:b");
        assert_no_errors(&result);
        // NOT has higher precedence than AND, so: TTL:a AND (NOT ABST:b)
        // Actually with implicit AND and NOT as prefix...
        // "TTL:a not ABST:b" = TTL:a AND (NOT ABST:b) implicit AND
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Binary(b)) => {
                assert_eq!(b.op, BoolOp::And);
                match b.right.as_ref() {
                    QueryExpr::Not(_) => {}
                    other => panic!("expected NOT, got {:?}", other),
                }
            }
            other => panic!("expected binary with NOT, got {:?}", other),
        }
    }

    #[test]
    fn range_expression() {
        let result = parse("APD:[20200101 TO 20241231]");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Field(f)) => {
                assert_eq!(f.field_name, "APD");
                match &f.body {
                    FieldBody::Simple(inner) => match inner.as_ref() {
                        QueryExpr::Range(r) => {
                            assert_eq!(r.from, "20200101");
                            assert_eq!(r.to, "20241231");
                        }
                        other => panic!("expected range, got {:?}", other),
                    },
                    _ => panic!("expected simple body"),
                }
            }
            other => panic!("expected field, got {:?}", other),
        }
    }

    #[test]
    fn quoted_phrase() {
        let result = parse("TTL:\"air condition\"");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Field(f)) => match &f.body {
                FieldBody::Simple(inner) => match inner.as_ref() {
                    QueryExpr::Quoted(q) => {
                        assert_eq!(q.value, "air condition");
                        assert_eq!(q.quote_char, '"');
                    }
                    other => panic!("expected quoted, got {:?}", other),
                },
                _ => panic!("expected simple body"),
            },
            other => panic!("expected field, got {:?}", other),
        }
    }

    #[test]
    fn proximity_operator() {
        let result = parse("data $W2 line");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Proximity(p)) => {
                assert_eq!(p.op, "$W2");
            }
            other => panic!("expected proximity, got {:?}", other),
        }
    }

    #[test]
    fn tree_at_expression() {
        let result = parse("ANCS:(TREE@\"拜耳股份公司\")");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Field(f)) => {
                assert_eq!(f.field_name, "ANCS");
                match &f.body {
                    FieldBody::Parenthesized { inner, .. } => match inner.as_ref() {
                        QueryExpr::TreeAt(t) => match t.operand.as_ref() {
                            QueryExpr::Quoted(q) => {
                                assert_eq!(q.value, "拜耳股份公司");
                            }
                            other => panic!("expected quoted, got {:?}", other),
                        },
                        other => panic!("expected tree_at, got {:?}", other),
                    },
                    _ => panic!("expected parenthesized body"),
                }
            }
            other => panic!("expected field, got {:?}", other),
        }
    }

    #[test]
    fn comment_standalone() {
        let result = parse("# this is a comment\nTTL:test");
        assert_no_errors(&result);
        assert_eq!(result.file.statements.len(), 2);
        match &result.file.statements[0] {
            Statement::Comment(c) => {
                assert_eq!(c.text, "# this is a comment");
            }
            other => panic!("expected comment, got {:?}", other),
        }
    }

    #[test]
    fn multiple_queries_with_blank_line() {
        let result = parse("TTL:a\n\nTTL:b");
        assert_no_errors(&result);
        // Should have: Query, BlankLine, Query
        assert_eq!(result.file.statements.len(), 3);
        assert!(matches!(result.file.statements[0], Statement::Query(_)));
        assert!(matches!(result.file.statements[1], Statement::BlankLine));
        assert!(matches!(result.file.statements[2], Statement::Query(_)));
    }

    #[test]
    fn complex_real_world_query() {
        let result = parse(
            "ttl:(空调 or \"air condition\" or 空气调节) and tac:(蒸发器 or evaporator)",
        );
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Binary(b)) => {
                assert_eq!(b.op, BoolOp::And);
                // Left should be field TTL with OR chain
                match b.left.as_ref() {
                    QueryExpr::Field(f) => assert_eq!(f.field_name, "ttl"),
                    other => panic!("expected field, got {:?}", other),
                }
                // Right should be field TAC with OR chain
                match b.right.as_ref() {
                    QueryExpr::Field(f) => assert_eq!(f.field_name, "tac"),
                    other => panic!("expected field, got {:?}", other),
                }
            }
            other => panic!("expected binary AND, got {:?}", other),
        }
    }

    #[test]
    fn nested_group_in_field() {
        let result = parse("tac:(空调 and (蒸发器 or evaporator))");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Field(f)) => {
                assert_eq!(f.field_name, "tac");
                match &f.body {
                    FieldBody::Parenthesized { inner, .. } => match inner.as_ref() {
                        QueryExpr::Binary(b) => {
                            assert_eq!(b.op, BoolOp::And);
                            match b.right.as_ref() {
                                QueryExpr::Group(g) => match g.inner.as_ref() {
                                    QueryExpr::Binary(inner_b) => {
                                        assert_eq!(inner_b.op, BoolOp::Or);
                                    }
                                    other => panic!("expected binary, got {:?}", other),
                                },
                                other => panic!("expected group, got {:?}", other),
                            }
                        }
                        other => panic!("expected binary, got {:?}", other),
                    },
                    _ => panic!("expected parenthesized"),
                }
            }
            other => panic!("expected field, got {:?}", other),
        }
    }

    #[test]
    fn frequency_operator() {
        let result = parse("TTL:(汽车 $FREQ2)");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Field(f)) => match &f.body {
                FieldBody::Parenthesized { inner, .. } => match inner.as_ref() {
                    QueryExpr::Frequency(freq) => {
                        assert_eq!(freq.op, "$FREQ2");
                    }
                    other => panic!("expected frequency, got {:?}", other),
                },
                _ => panic!("expected parenthesized"),
            },
            other => panic!("expected field, got {:?}", other),
        }
    }

    #[test]
    fn gand_operator() {
        let result = parse("EPDS:DE GAND EPDS_SLS:1");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Binary(b)) => {
                assert_eq!(b.op, BoolOp::Gand);
            }
            other => panic!("expected binary GAND, got {:?}", other),
        }
    }

    #[test]
    fn multiline_query() {
        let result = parse("TTL:a\nand ABST:b");
        assert_no_errors(&result);
        match &result.file.statements[0] {
            Statement::Query(QueryExpr::Binary(b)) => {
                assert_eq!(b.op, BoolOp::And);
            }
            other => panic!("expected binary AND for multiline, got {:?}", other),
        }
    }
}
