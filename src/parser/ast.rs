use super::token::Span;

/// A complete `.patsnap` file: a sequence of statements.
#[derive(Debug, Clone)]
pub struct File {
    pub statements: Vec<Statement>,
}

/// A top-level statement in the file.
#[derive(Debug, Clone)]
pub enum Statement {
    /// A search query expression.
    Query(QueryExpr),
    /// A standalone comment line.
    Comment(Comment),
    /// A blank line separator.
    BlankLine,
}

/// A comment node.
#[derive(Debug, Clone)]
pub struct Comment {
    pub text: String,
    pub span: Span,
}

/// A query expression node.
#[derive(Debug, Clone)]
pub enum QueryExpr {
    /// Binary expression: left OP right (AND, OR, GAND).
    Binary(BinaryExpr),
    /// Unary NOT expression.
    Not(NotExpr),
    /// Field expression: FIELD:body or FIELD:(body).
    Field(FieldExpr),
    /// Parenthesized group: (expr).
    Group(GroupExpr),
    /// A bare keyword (may include wildcards).
    Keyword(KeywordTerm),
    /// A quoted phrase: "...".
    Quoted(QuotedTerm),
    /// A range expression: [from TO to].
    Range(RangeExpr),
    /// Proximity expression: left $Wn right.
    Proximity(ProximityExpr),
    /// Frequency expression: term $FREQn.
    Frequency(FrequencyExpr),
    /// TREE@ expression.
    TreeAt(TreeAtExpr),
    /// An error node — unparseable region preserved verbatim.
    Error(ErrorNode),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<QueryExpr>,
    pub op: BoolOp,
    pub op_span: Span,
    pub right: Box<QueryExpr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolOp {
    And,
    Or,
    Gand,
}

#[derive(Debug, Clone)]
pub struct NotExpr {
    pub op_span: Span,
    pub operand: Box<QueryExpr>,
}

#[derive(Debug, Clone)]
pub struct FieldExpr {
    pub field_name: String,
    pub field_span: Span,
    pub colon_span: Span,
    pub body: FieldBody,
}

#[derive(Debug, Clone)]
pub enum FieldBody {
    /// Simple value without parentheses: FIELD:value
    Simple(Box<QueryExpr>),
    /// Parenthesized body: FIELD:(...)
    Parenthesized {
        lparen_span: Span,
        inner: Box<QueryExpr>,
        rparen_span: Span,
    },
}

#[derive(Debug, Clone)]
pub struct GroupExpr {
    pub lparen_span: Span,
    pub inner: Box<QueryExpr>,
    pub rparen_span: Span,
}

#[derive(Debug, Clone)]
pub struct KeywordTerm {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct QuotedTerm {
    pub value: String,
    pub quote_char: char,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct RangeExpr {
    pub lbracket_span: Span,
    pub from: String,
    pub from_span: Span,
    pub to_keyword_span: Span,
    pub to: String,
    pub to_span: Span,
    pub rbracket_span: Span,
}

#[derive(Debug, Clone)]
pub struct ProximityExpr {
    pub left: Box<QueryExpr>,
    pub op: String,
    pub op_span: Span,
    pub right: Box<QueryExpr>,
}

#[derive(Debug, Clone)]
pub struct FrequencyExpr {
    pub operand: Box<QueryExpr>,
    pub op: String,
    pub op_span: Span,
}

#[derive(Debug, Clone)]
pub struct TreeAtExpr {
    pub tree_at_span: Span,
    pub operand: Box<QueryExpr>,
}

#[derive(Debug, Clone)]
pub struct ErrorNode {
    pub raw_text: String,
    pub span: Span,
}
