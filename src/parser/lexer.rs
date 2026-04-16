use super::fields::is_field_code;
use super::token::{Span, Token, TokenKind};

/// Lexer for Patsnap search query syntax.
pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<(usize, char)>,
    pos: usize, // index into chars
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars: Vec<(usize, char)> = source.char_indices().collect();
        Self {
            source,
            chars,
            pos: 0,
        }
    }

    /// Tokenize the entire input into a Vec of tokens.
    /// Whitespace tokens are excluded; Newline tokens are preserved.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok.kind == TokenKind::Whitespace {
                continue;
            }
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        if self.pos >= self.chars.len() {
            let byte_pos = self.source.len();
            return Token::new(TokenKind::Eof, Span::new(byte_pos, byte_pos));
        }

        let (byte_start, ch) = self.chars[self.pos];

        match ch {
            // Newlines
            '\n' => {
                self.pos += 1;
                Token::new(TokenKind::Newline, Span::new(byte_start, byte_start + 1))
            }
            '\r' => {
                self.pos += 1;
                let end = if self.peek_char() == Some('\n') {
                    self.pos += 1;
                    byte_start + 2
                } else {
                    byte_start + 1
                };
                Token::new(TokenKind::Newline, Span::new(byte_start, end))
            }

            // Whitespace (not newlines)
            ' ' | '\t' => self.read_whitespace(byte_start),

            // Comment: # ...
            '#' => self.read_line_comment(byte_start),

            // Structural
            ':' => {
                self.pos += 1;
                Token::new(TokenKind::Colon, Span::new(byte_start, byte_start + 1))
            }
            '(' => {
                self.pos += 1;
                Token::new(TokenKind::LParen, Span::new(byte_start, byte_start + 1))
            }
            ')' => {
                self.pos += 1;
                Token::new(TokenKind::RParen, Span::new(byte_start, byte_start + 1))
            }
            '[' => {
                self.pos += 1;
                Token::new(TokenKind::LBracket, Span::new(byte_start, byte_start + 1))
            }
            ']' => {
                self.pos += 1;
                Token::new(TokenKind::RBracket, Span::new(byte_start, byte_start + 1))
            }

            // Quoted string
            '"' => self.read_quoted_string(byte_start, '"'),
            '\'' => self.read_quoted_string(byte_start, '\''),

            // Dollar-sign operators: $Wn, $PREn, $WS, $SEN, $PARA, $FREQn
            '$' => self.read_dollar_op(byte_start),

            // Word: field code, boolean operator, keyword, or TREE@
            _ if is_word_start(ch) => self.read_word(byte_start),

            // Any other character — treat as keyword character
            _ => self.read_keyword_other(byte_start),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).map(|&(_, c)| c)
    }

    fn byte_pos(&self) -> usize {
        self.chars
            .get(self.pos)
            .map(|&(b, _)| b)
            .unwrap_or(self.source.len())
    }

    fn read_whitespace(&mut self, byte_start: usize) -> Token {
        self.pos += 1;
        while let Some(&(_, ch)) = self.chars.get(self.pos) {
            if ch == ' ' || ch == '\t' {
                self.pos += 1;
            } else {
                break;
            }
        }
        Token::new(TokenKind::Whitespace, Span::new(byte_start, self.byte_pos()))
    }

    fn read_line_comment(&mut self, byte_start: usize) -> Token {
        // Skip everything until newline (but don't consume the newline).
        self.pos += 1; // skip '#'
        while let Some(&(_, ch)) = self.chars.get(self.pos) {
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.pos += 1;
        }
        Token::new(TokenKind::LineComment, Span::new(byte_start, self.byte_pos()))
    }

    fn read_quoted_string(&mut self, byte_start: usize, quote: char) -> Token {
        self.pos += 1; // skip opening quote
        while let Some(&(_, ch)) = self.chars.get(self.pos) {
            self.pos += 1;
            if ch == quote {
                return Token::new(
                    TokenKind::QuotedString,
                    Span::new(byte_start, self.byte_pos()),
                );
            }
            if ch == '\\' {
                // skip escaped character
                if self.pos < self.chars.len() {
                    self.pos += 1;
                }
            }
        }
        // Unterminated quote — return what we have as an error
        Token::new(TokenKind::Error, Span::new(byte_start, self.byte_pos()))
    }

    fn read_dollar_op(&mut self, byte_start: usize) -> Token {
        self.pos += 1; // skip '$'
        // Collect alphanumeric chars after '$'
        let word_start = self.pos;
        while let Some(&(_, ch)) = self.chars.get(self.pos) {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let byte_end = self.byte_pos();
        let word = if word_start < self.chars.len() {
            let ws = self.chars[word_start].0;
            &self.source[ws..byte_end]
        } else {
            ""
        };
        let upper = word.to_ascii_uppercase();

        // Determine the operator kind
        let kind = if upper.starts_with("W") && !upper.starts_with("WS") && word.len() > 1 {
            // $W5, $W10, etc.
            if upper[1..].chars().all(|c| c.is_ascii_digit()) {
                TokenKind::ProximityOp
            } else {
                TokenKind::Keyword
            }
        } else if upper == "WS" || upper == "SEN" || upper == "PARA" {
            TokenKind::ProximityOp
        } else if upper.starts_with("PRE") {
            // $PRE3, $PRE10, etc.
            if upper.len() > 3 && upper[3..].chars().all(|c| c.is_ascii_digit()) {
                TokenKind::ProximityOp
            } else if upper == "PRE" {
                // bare $PRE without number — still treat as proximity
                TokenKind::ProximityOp
            } else {
                TokenKind::Keyword
            }
        } else if upper.starts_with("FREQ") {
            // $FREQ5, etc.
            if upper.len() > 4 && upper[4..].chars().all(|c| c.is_ascii_digit()) {
                TokenKind::FrequencyOp
            } else if upper == "FREQ" {
                TokenKind::FrequencyOp
            } else {
                TokenKind::Keyword
            }
        } else {
            // Unknown $ operator — treat as keyword
            TokenKind::Keyword
        };

        Token::new(kind, Span::new(byte_start, byte_end))
    }

    fn read_word(&mut self, byte_start: usize) -> Token {
        // Collect word characters: alphanumeric, underscore, wildcards (*, ?, #) embedded in word,
        // and special chars like @, /, -, . for classification codes and TREE@.
        self.pos += 1;

        while let Some(&(_, ch)) = self.chars.get(self.pos) {
            if is_word_continue(ch) {
                self.pos += 1;
            } else {
                break;
            }
        }

        let byte_end = self.byte_pos();
        let word = &self.source[byte_start..byte_end];
        let upper = word.to_ascii_uppercase();

        // Check for TREE@ — the word "TREE" followed by '@'
        if upper == "TREE" && self.peek_char() == Some('@') {
            self.pos += 1; // consume '@'
            let byte_end = self.byte_pos();
            return Token::new(TokenKind::TreeAt, Span::new(byte_start, byte_end));
        }

        // Check if this is a field code (word followed by ':')
        if self.peek_char() == Some(':') && is_field_code(word) {
            return Token::new(TokenKind::FieldCode, Span::new(byte_start, byte_end));
        }

        // Check for boolean operators (case-insensitive)
        match upper.as_str() {
            "AND" => Token::new(TokenKind::And, Span::new(byte_start, byte_end)),
            "OR" => Token::new(TokenKind::Or, Span::new(byte_start, byte_end)),
            "NOT" => Token::new(TokenKind::Not, Span::new(byte_start, byte_end)),
            "GAND" => Token::new(TokenKind::Gand, Span::new(byte_start, byte_end)),
            "TO" => Token::new(TokenKind::To, Span::new(byte_start, byte_end)),
            _ => Token::new(TokenKind::Keyword, Span::new(byte_start, byte_end)),
        }
    }

    /// Read characters that don't start a word but may be part of a keyword
    /// (CJK characters, special symbols, wildcards, etc.)
    fn read_keyword_other(&mut self, byte_start: usize) -> Token {
        self.pos += 1;
        // Continue reading keyword-compatible characters
        while let Some(&(_, ch)) = self.chars.get(self.pos) {
            if is_keyword_continue(ch) {
                self.pos += 1;
            } else {
                break;
            }
        }
        Token::new(TokenKind::Keyword, Span::new(byte_start, self.byte_pos()))
    }
}

/// Characters that can start a "word" — ASCII letters, digits, or wildcard chars.
fn is_word_start(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '*' || ch == '?'
        || is_cjk_or_extended(ch)
}

/// Characters that can continue a "word".
fn is_word_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || ch == '_'
        || ch == '*'
        || ch == '?'
        || ch == '#' // wildcard: 0 or 1 char
        || ch == '.'
        || ch == '/'
        || ch == '-'
        || is_special_char(ch)
        || is_cjk_or_extended(ch)
}

/// Characters that can continue a keyword (non-word-start chars like CJK etc.)
fn is_keyword_continue(ch: char) -> bool {
    !matches!(
        ch,
        ' ' | '\t' | '\n' | '\r' | '(' | ')' | '[' | ']' | ':' | '"' | '\'' | '$'
    ) && !ch.is_ascii_whitespace()
    && ch != '#' // # at start of a new token is a comment
}

/// Check if a character is CJK or other extended Unicode that can appear in keywords.
fn is_cjk_or_extended(ch: char) -> bool {
    matches!(ch,
        '\u{4E00}'..='\u{9FFF}'   // CJK Unified Ideographs
        | '\u{3400}'..='\u{4DBF}' // CJK Unified Ideographs Extension A
        | '\u{F900}'..='\u{FAFF}' // CJK Compatibility Ideographs
        | '\u{3000}'..='\u{303F}' // CJK Symbols and Punctuation
        | '\u{3040}'..='\u{309F}' // Hiragana
        | '\u{30A0}'..='\u{30FF}' // Katakana
        | '\u{AC00}'..='\u{D7AF}' // Hangul Syllables
    )
}

/// Special characters supported by Patsnap as part of keywords.
fn is_special_char(ch: char) -> bool {
    matches!(ch,
        '℃' | '℉' | '%' | '±' | '°' | '™' | '®'
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::token::TokenKind::*;

    fn lex(input: &str) -> Vec<(TokenKind, &str)> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        tokens
            .iter()
            .filter(|t| t.kind != TokenKind::Eof)
            .map(|t| (t.kind.clone(), t.text(input)))
            .collect()
    }

    #[test]
    fn simple_field_expression() {
        let result = lex("TTL:汽车");
        assert_eq!(
            result,
            vec![
                (FieldCode, "TTL"),
                (Colon, ":"),
                (Keyword, "汽车"),
            ]
        );
    }

    #[test]
    fn field_with_parens_and_or() {
        let result = lex("tac:(空调 or evaporator)");
        assert_eq!(
            result,
            vec![
                (FieldCode, "tac"),
                (Colon, ":"),
                (LParen, "("),
                (Keyword, "空调"),
                (Or, "or"),
                (Keyword, "evaporator"),
                (RParen, ")"),
            ]
        );
    }

    #[test]
    fn boolean_operators() {
        let result = lex("a AND b OR c NOT d GAND e");
        assert_eq!(
            result,
            vec![
                (Keyword, "a"),
                (And, "AND"),
                (Keyword, "b"),
                (Or, "OR"),
                (Keyword, "c"),
                (Not, "NOT"),
                (Keyword, "d"),
                (Gand, "GAND"),
                (Keyword, "e"),
            ]
        );
    }

    #[test]
    fn case_insensitive_operators() {
        let result = lex("a and b or c not d");
        assert_eq!(
            result,
            vec![
                (Keyword, "a"),
                (And, "and"),
                (Keyword, "b"),
                (Or, "or"),
                (Keyword, "c"),
                (Not, "not"),
                (Keyword, "d"),
            ]
        );
    }

    #[test]
    fn quoted_string() {
        let result = lex("TTL:\"air condition\"");
        assert_eq!(
            result,
            vec![
                (FieldCode, "TTL"),
                (Colon, ":"),
                (QuotedString, "\"air condition\""),
            ]
        );
    }

    #[test]
    fn range_expression() {
        let result = lex("APD:[20200101 TO 20241231]");
        assert_eq!(
            result,
            vec![
                (FieldCode, "APD"),
                (Colon, ":"),
                (LBracket, "["),
                (Keyword, "20200101"),
                (To, "TO"),
                (Keyword, "20241231"),
                (RBracket, "]"),
            ]
        );
    }

    #[test]
    fn wildcard_keywords() {
        let result = lex("electr* ?otor m#tor");
        assert_eq!(
            result,
            vec![
                (Keyword, "electr*"),
                (Keyword, "?otor"),
                (Keyword, "m#tor"),
            ]
        );
    }

    #[test]
    fn proximity_operators() {
        let result = lex("data $W2 line");
        assert_eq!(
            result,
            vec![
                (Keyword, "data"),
                (ProximityOp, "$W2"),
                (Keyword, "line"),
            ]
        );
    }

    #[test]
    fn proximity_pre_op() {
        let result = lex("data $PRE5 line");
        assert_eq!(
            result,
            vec![
                (Keyword, "data"),
                (ProximityOp, "$PRE5"),
                (Keyword, "line"),
            ]
        );
    }

    #[test]
    fn proximity_ws_sen_para() {
        let result = lex("a $WS b $SEN c $PARA d");
        assert_eq!(
            result,
            vec![
                (Keyword, "a"),
                (ProximityOp, "$WS"),
                (Keyword, "b"),
                (ProximityOp, "$SEN"),
                (Keyword, "c"),
                (ProximityOp, "$PARA"),
                (Keyword, "d"),
            ]
        );
    }

    #[test]
    fn frequency_operator() {
        let result = lex("汽车 $FREQ2");
        assert_eq!(
            result,
            vec![
                (Keyword, "汽车"),
                (FrequencyOp, "$FREQ2"),
            ]
        );
    }

    #[test]
    fn tree_at_operator() {
        let result = lex("ANCS:(TREE@\"拜耳股份公司\")");
        assert_eq!(
            result,
            vec![
                (FieldCode, "ANCS"),
                (Colon, ":"),
                (LParen, "("),
                (TreeAt, "TREE@"),
                (QuotedString, "\"拜耳股份公司\""),
                (RParen, ")"),
            ]
        );
    }

    #[test]
    fn line_comment() {
        let result = lex("# this is a comment\nTTL:test");
        assert_eq!(
            result,
            vec![
                (LineComment, "# this is a comment"),
                (Newline, "\n"),
                (FieldCode, "TTL"),
                (Colon, ":"),
                (Keyword, "test"),
            ]
        );
    }

    #[test]
    fn multiple_queries_with_blank_line() {
        let result = lex("TTL:a\n\nTTL:b");
        assert_eq!(
            result,
            vec![
                (FieldCode, "TTL"),
                (Colon, ":"),
                (Keyword, "a"),
                (Newline, "\n"),
                (Newline, "\n"),
                (FieldCode, "TTL"),
                (Colon, ":"),
                (Keyword, "b"),
            ]
        );
    }

    #[test]
    fn complex_real_world_query() {
        let result = lex("ttl:(空调 or \"air condition\" or 空气调节) and tac:(蒸发器 or evaporator)");
        assert_eq!(result.len(), 17);
        assert_eq!(result[0], (FieldCode, "ttl"));
        assert_eq!(result[1], (Colon, ":"));
        assert_eq!(result[2], (LParen, "("));
        assert_eq!(result[3], (Keyword, "空调"));
        assert_eq!(result[4], (Or, "or"));
        assert_eq!(result[5], (QuotedString, "\"air condition\""));
        assert_eq!(result[6], (Or, "or"));
        assert_eq!(result[7], (Keyword, "空气调节"));
        assert_eq!(result[8], (RParen, ")"));
        assert_eq!(result[9], (And, "and"));
        assert_eq!(result[10], (FieldCode, "tac"));
        assert_eq!(result[11], (Colon, ":"));
        assert_eq!(result[12], (LParen, "("));
        assert_eq!(result[13], (Keyword, "蒸发器"));
        assert_eq!(result[14], (Or, "or"));
        assert_eq!(result[15], (Keyword, "evaporator"));
        assert_eq!(result[16], (RParen, ")"));
    }

    #[test]
    fn ipc_classification_code() {
        let result = lex("IPC:A61K31/04");
        assert_eq!(
            result,
            vec![
                (FieldCode, "IPC"),
                (Colon, ":"),
                (Keyword, "A61K31/04"),
            ]
        );
    }

    #[test]
    fn ccf_custom_field() {
        let result = lex("CCF_PR:\"标引值\"");
        assert_eq!(
            result,
            vec![
                (FieldCode, "CCF_PR"),
                (Colon, ":"),
                (QuotedString, "\"标引值\""),
            ]
        );
    }

    #[test]
    fn workspace_path() {
        let result = lex("MWS:|工作空间A\\文件夹B|");
        // The pipe-delimited path should be parsed as keywords
        assert_eq!(result[0], (FieldCode, "MWS"));
        assert_eq!(result[1], (Colon, ":"));
        // The rest is keyword content with pipes
        assert!(result.len() >= 3);
    }

    #[test]
    fn empty_input() {
        let result = lex("");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn whitespace_only() {
        let result = lex("   \t  ");
        assert_eq!(result, vec![]);
    }

    #[test]
    fn wildcard_at_start() {
        let result = lex("*otor");
        assert_eq!(result, vec![(Keyword, "*otor")]);
    }

    #[test]
    fn special_characters_in_keyword() {
        let result = lex("ABST:(85℃)");
        assert_eq!(result[0], (FieldCode, "ABST"));
        assert_eq!(result[1], (Colon, ":"));
        assert_eq!(result[2], (LParen, "("));
        // 85℃ should be a keyword
        assert_eq!(result[3].0, Keyword);
        assert_eq!(result[4], (RParen, ")"));
    }
}
