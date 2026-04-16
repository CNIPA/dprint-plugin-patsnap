use std::rc::Rc;

use dprint_core::formatting::*;

use crate::configuration::types::*;
use crate::parser::ast::*;
use crate::parser::token::Span;

use super::context::Context;

/// Generate PrintItems IR from a parsed File AST.
pub fn generate(file: &File, ctx: &Context) -> PrintItems {
    let mut items = PrintItems::new();

    for stmt in file.statements.iter() {
        match stmt {
            Statement::Query(expr) => {
                items.extend(gen_query(expr, ctx));
                items.push_signal(Signal::NewLine);
            }
            Statement::Comment(comment) => {
                items.extend(gen_comment(comment));
                items.push_signal(Signal::NewLine);
            }
            Statement::BlankLine => {
                items.push_signal(Signal::NewLine);
            }
        }
    }

    items
}

// ── Top-level query ──

/// Generate IR for a top-level query expression.
/// Binary chains are always broken into multiple lines.
fn gen_query(expr: &QueryExpr, ctx: &Context) -> PrintItems {
    match expr {
        QueryExpr::Binary(_) => gen_top_level_binary(expr, ctx),
        QueryExpr::Not(n) => {
            let mut items = PrintItems::new();
            items.push_string(format_not_op(&n.op_span, ctx));
            items.push_string(" ".into());
            items.extend(gen_expr(&n.operand, ctx));
            items
        }
        _ => gen_expr(expr, ctx),
    }
}

// ── Flat part for binary chain flattening ──

struct FlatPart<'a> {
    /// Operator text (None for the first operand).
    op: Option<String>,
    /// The operand expression.
    expr: &'a QueryExpr,
}

/// Recursively flatten a binary expression tree into a linear list of
/// (operator, operand) pairs for formatting.
fn flatten_binary_chain<'a>(
    expr: &'a QueryExpr,
    parts: &mut Vec<FlatPart<'a>>,
    ctx: &Context,
) {
    match expr {
        QueryExpr::Binary(b) => {
            // Flatten left subtree first
            flatten_binary_chain(&b.left, parts, ctx);

            // Handle implicit AND + NOT → display as "not"
            if b.op == BoolOp::And && b.op_span.start == b.op_span.end {
                if let QueryExpr::Not(not_expr) = b.right.as_ref() {
                    parts.push(FlatPart {
                        op: Some(format_not_op(&not_expr.op_span, ctx)),
                        expr: &not_expr.operand,
                    });
                    return;
                }
            }

            let op_text = format_bool_op(b.op, &b.op_span, ctx);

            // Flatten right subtree and attach our operator to its first element
            let right_start = parts.len();
            flatten_binary_chain(&b.right, parts, ctx);
            if right_start < parts.len() && parts[right_start].op.is_none() {
                parts[right_start].op = Some(op_text);
            }
        }
        QueryExpr::Not(n) => {
            // Standalone NOT at the beginning of a chain
            parts.push(FlatPart {
                op: Some(format_not_op(&n.op_span, ctx)),
                expr: &n.operand,
            });
        }
        other => {
            parts.push(FlatPart {
                op: None,
                expr: other,
            });
        }
    }
}

/// Generate IR for a top-level binary chain with forced line breaks
/// and operator-aligned formatting.
///
/// Output style:
///     first_operand
/// and second_operand
///  or third_operand
///
/// All operands are left-aligned; operators are right-justified within
/// `align_width` characters (= max operator length + 1).
fn gen_top_level_binary(expr: &QueryExpr, ctx: &Context) -> PrintItems {
    let mut parts = Vec::new();
    flatten_binary_chain(expr, &mut parts, ctx);

    if parts.is_empty() {
        return PrintItems::new();
    }
    if parts.len() == 1 {
        return gen_single_part(&parts[0], ctx);
    }

    let align_width = calc_align_width(&parts);
    let mut items = PrintItems::new();

    // First operand (with right-justified op if present, or leading padding)
    emit_aligned_part(&parts[0], align_width, &mut items, ctx);

    // Continuation lines with forced breaks
    for part in &parts[1..] {
        items.push_signal(Signal::NewLine);
        emit_aligned_part(part, align_width, &mut items, ctx);
    }

    items
}

/// Calculate the alignment width for a binary chain.
/// align_width = max(op.len() for all ops) + 1 (for trailing space).
fn calc_align_width(parts: &[FlatPart]) -> usize {
    let max_op_len = parts
        .iter()
        .filter_map(|p| p.op.as_ref())
        .map(|op| op.len())
        .max()
        .unwrap_or(0);
    max_op_len + 1
}

/// Emit a single FlatPart (when the chain has only one element).
fn gen_single_part(part: &FlatPart, ctx: &Context) -> PrintItems {
    let mut items = PrintItems::new();
    if let Some(ref op) = part.op {
        items.push_string(op.clone());
        items.push_string(" ".into());
    }
    items.extend(gen_expr(part.expr, ctx));
    items
}

/// Emit a part with right-justified operator alignment.
/// If the part has an op, right-justify it within `align_width` chars.
/// If the part has no op (first operand), emit `align_width` spaces as padding.
fn emit_aligned_part(
    part: &FlatPart,
    align_width: usize,
    items: &mut PrintItems,
    ctx: &Context,
) {
    if let Some(ref op) = part.op {
        let pad = align_width.saturating_sub(op.len() + 1);
        if pad > 0 {
            items.push_string(" ".repeat(pad));
        }
        items.push_string(op.clone());
        items.push_string(" ".into());
    } else {
        items.push_string(" ".repeat(align_width));
    }
    items.extend(gen_expr(part.expr, ctx));
}

// ── Inner expression generation ──

/// Generate IR for any query expression (inner level).
fn gen_expr(expr: &QueryExpr, ctx: &Context) -> PrintItems {
    match expr {
        QueryExpr::Binary(_) => gen_inner_binary_chain(expr, ctx),
        QueryExpr::Not(n) => gen_not(n, ctx),
        QueryExpr::Field(f) => gen_field(f, ctx),
        QueryExpr::Group(g) => gen_group(g, ctx),
        QueryExpr::Keyword(k) => gen_keyword(k),
        QueryExpr::Quoted(q) => gen_quoted(q, ctx),
        QueryExpr::Range(r) => gen_range(r),
        QueryExpr::Proximity(p) => gen_proximity(p, ctx),
        QueryExpr::Frequency(f) => gen_frequency(f, ctx),
        QueryExpr::TreeAt(t) => gen_tree_at(t, ctx),
        QueryExpr::Error(e) => gen_error(e),
    }
}

/// Generate IR for an inner binary chain using adaptive line breaking.
/// When the chain fits on one line: `a or b or c`
/// When it breaks, operators are right-justified at the indent position:
///    a
/// or b
/// or c
fn gen_inner_binary_chain(expr: &QueryExpr, ctx: &Context) -> PrintItems {
    let mut parts = Vec::new();
    flatten_binary_chain(expr, &mut parts, ctx);

    if parts.is_empty() {
        return PrintItems::new();
    }
    if parts.len() == 1 {
        return gen_single_part(&parts[0], ctx);
    }

    let align_width = calc_align_width(&parts);
    let mut items = PrintItems::new();

    // First operand — add alignment padding only when already at start of line
    {
        let pad_str: PrintItems = " ".repeat(align_width).into();
        items.push_condition(conditions::if_true(
            "firstAlignPad",
            condition_resolvers::is_start_of_line(),
            pad_str,
        ));
    }
    if let Some(ref op) = parts[0].op {
        items.push_string(op.clone());
        items.push_string(" ".into());
    }
    items.extend(gen_expr(parts[0].expr, ctx));

    // Continuation parts with adaptive SpaceOrNewLine
    for part in &parts[1..] {
        items.push_signal(Signal::SpaceOrNewLine);
        if let Some(ref op) = part.op {
            let op_pad = align_width.saturating_sub(op.len() + 1);

            // When at start of line (broke): right-justified padding + op + space
            let mut true_path = PrintItems::new();
            if op_pad > 0 {
                true_path.push_string(" ".repeat(op_pad));
            }
            true_path.push_string(op.clone());
            true_path.push_string(" ".into());

            // When inline (no break): just op + space
            let mut false_path = PrintItems::new();
            false_path.push_string(op.clone());
            false_path.push_string(" ".into());

            items.push_condition(Condition::new(
                "opAlign",
                ConditionProperties {
                    condition: Rc::new(|context| {
                        Some(context.writer_info.is_start_of_line())
                    }),
                    true_path: Some(true_path),
                    false_path: Some(false_path),
                },
            ));
        }
        items.extend(gen_expr(part.expr, ctx));
    }

    items
}

fn gen_not(expr: &NotExpr, ctx: &Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_string(format_not_op(&expr.op_span, ctx));
    items.push_string(" ".into());
    items.extend(gen_expr(&expr.operand, ctx));
    items
}

fn gen_field(expr: &FieldExpr, ctx: &Context) -> PrintItems {
    let mut items = PrintItems::new();

    let field_name = format_case(&expr.field_name, ctx.config.field_case);
    items.push_string(field_name);
    items.push_string(":".into());

    match &expr.body {
        FieldBody::Simple(inner) => {
            items.extend(gen_expr(inner, ctx));
        }
        FieldBody::Parenthesized { inner, .. } => {
            items.push_string("(".into());
            items.extend(gen_expr(inner, ctx));
            items.push_string(")".into());
        }
    }

    items
}

fn gen_group(expr: &GroupExpr, ctx: &Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_string("(".into());
    items.extend(gen_expr(&expr.inner, ctx));
    items.push_string(")".into());
    items
}

fn gen_keyword(term: &KeywordTerm) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_string(term.value.clone());
    items
}

fn gen_quoted(term: &QuotedTerm, ctx: &Context) -> PrintItems {
    let mut items = PrintItems::new();
    let quote = match ctx.config.quote_style {
        QuoteStyle::Double => '"',
        QuoteStyle::Single => '\'',
        QuoteStyle::Preserve => term.quote_char,
    };
    items.push_string(format!("{}{}{}", quote, term.value, quote));
    items
}

fn gen_range(expr: &RangeExpr) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_string(format!("[{} TO {}]", expr.from, expr.to));
    items
}

fn gen_proximity(expr: &ProximityExpr, ctx: &Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.extend(gen_expr(&expr.left, ctx));
    items.push_string(" ".into());
    items.push_string(expr.op.clone());
    items.push_string(" ".into());
    items.extend(gen_expr(&expr.right, ctx));
    items
}

fn gen_frequency(expr: &FrequencyExpr, ctx: &Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.extend(gen_expr(&expr.operand, ctx));
    items.push_string(" ".into());
    items.push_string(expr.op.clone());
    items
}

fn gen_tree_at(expr: &TreeAtExpr, ctx: &Context) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_string("TREE@".into());
    items.extend(gen_expr(&expr.operand, ctx));
    items
}

fn gen_comment(comment: &Comment) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_string(comment.text.clone());
    items
}

fn gen_error(error: &ErrorNode) -> PrintItems {
    let mut items = PrintItems::new();
    items.push_string(error.raw_text.clone());
    items
}

// ── Formatting helpers ──

fn format_bool_op(op: BoolOp, op_span: &Span, ctx: &Context) -> String {
    if ctx.config.boolean_operator_case == CaseStyle::Preserve && op_span.len() > 0 {
        return ctx.source[op_span.start..op_span.end].to_string();
    }
    let text = match op {
        BoolOp::And => "and",
        BoolOp::Or => "or",
        BoolOp::Gand => "gand",
    };
    format_case(text, ctx.config.boolean_operator_case)
}

fn format_not_op(not_span: &Span, ctx: &Context) -> String {
    if ctx.config.boolean_operator_case == CaseStyle::Preserve && not_span.len() > 0 {
        return ctx.source[not_span.start..not_span.end].to_string();
    }
    format_case("not", ctx.config.boolean_operator_case)
}

fn format_case(text: &str, case: CaseStyle) -> String {
    match case {
        CaseStyle::Lowercase => text.to_lowercase(),
        CaseStyle::Uppercase => text.to_uppercase(),
        CaseStyle::Preserve => text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::configuration::builder::ConfigurationBuilder;
    use crate::configuration::types::*;
    use crate::format_text::format_text;

    fn format(input: &str) -> String {
        let config = ConfigurationBuilder::new().build();
        format_text(Path::new("test.patsnap"), input, &config)
            .unwrap()
            .unwrap_or_else(|| input.to_string())
    }

    fn format_with(input: &str, build: impl FnOnce(&mut ConfigurationBuilder)) -> String {
        let mut builder = ConfigurationBuilder::new();
        build(&mut builder);
        let config = builder.build();
        format_text(Path::new("test.patsnap"), input, &config)
            .unwrap()
            .unwrap_or_else(|| input.to_string())
    }

    // ── Basic formatting ──

    #[test]
    fn simple_keyword() {
        assert_eq!(format("汽车"), "汽车\n");
    }

    #[test]
    fn field_lowercase() {
        assert_eq!(format("TTL:汽车"), "ttl:汽车\n");
    }

    #[test]
    fn field_uppercase() {
        let result = format_with("ttl:汽车", |b| {
            b.field_case(CaseStyle::Uppercase);
        });
        assert_eq!(result, "TTL:汽车\n");
    }

    #[test]
    fn field_with_parenthesized_body() {
        assert_eq!(
            format("TTL:(空调 or 蒸发器)"),
            "ttl:(空调 or 蒸发器)\n"
        );
    }

    // ── Top-level binary breaking ──

    #[test]
    fn top_level_and_breaks() {
        assert_eq!(
            format("TTL:空调 and ABST:蒸发器"),
            "    ttl:空调\nand abst:蒸发器\n"
        );
    }

    #[test]
    fn top_level_or_breaks() {
        assert_eq!(
            format("TTL:空调 or ABST:蒸发器"),
            "   ttl:空调\nor abst:蒸发器\n"
        );
    }

    #[test]
    fn top_level_chain_three_parts() {
        assert_eq!(
            format("TTL:a AND ABST:b OR AN:c"),
            "    ttl:a\nand abst:b\n or an:c\n"
        );
    }

    #[test]
    fn top_level_not_in_chain() {
        assert_eq!(
            format("TTL:a not ABST:b"),
            "    ttl:a\nnot abst:b\n"
        );
    }

    // ── Boolean operator case ──

    #[test]
    fn boolean_operator_uppercase() {
        let result = format_with("TTL:空调 and ABST:蒸发器", |b| {
            b.boolean_operator_case(CaseStyle::Uppercase);
        });
        assert_eq!(result, "    ttl:空调\nAND abst:蒸发器\n");
    }

    // ── Inner binary (adaptive breaking) ──

    #[test]
    fn inner_or_chain_fits_single_line() {
        assert_eq!(
            format("ttl:(空调 or 蒸发器)"),
            "ttl:(空调 or 蒸发器)\n"
        );
    }

    // ── Range expression ──

    #[test]
    fn range_expression() {
        assert_eq!(
            format("APD:[20200101 TO 20241231]"),
            "apd:[20200101 TO 20241231]\n"
        );
    }

    // ── Quoted strings ──

    #[test]
    fn quoted_string_preserve() {
        assert_eq!(
            format("TTL:\"air condition\""),
            "ttl:\"air condition\"\n"
        );
    }

    #[test]
    fn quoted_string_single() {
        let result = format_with("TTL:\"air condition\"", |b| {
            b.quote_style(QuoteStyle::Single);
        });
        assert_eq!(result, "ttl:'air condition'\n");
    }

    // ── Proximity & frequency ──

    #[test]
    fn proximity_operator() {
        assert_eq!(format("data $W2 line"), "data $W2 line\n");
    }

    #[test]
    fn frequency_operator() {
        assert_eq!(
            format("TTL:(汽车 $FREQ2)"),
            "ttl:(汽车 $FREQ2)\n"
        );
    }

    // ── TREE@ ──

    #[test]
    fn tree_at_expression() {
        assert_eq!(
            format("ANCS:(TREE@\"拜耳股份公司\")"),
            "ancs:(TREE@\"拜耳股份公司\")\n"
        );
    }

    // ── Comments & blank lines ──

    #[test]
    fn comment_preserved() {
        assert_eq!(
            format("# this is a comment\nTTL:test"),
            "# this is a comment\nttl:test\n"
        );
    }

    #[test]
    fn blank_line_between_queries() {
        assert_eq!(
            format("TTL:a\n\nTTL:b"),
            "ttl:a\n\nttl:b\n"
        );
    }

    // ── Complex real-world query ──

    #[test]
    fn complex_query() {
        let input = "ttl:(空调 or \"air condition\" or 空气调节) and tac:(蒸发器 or evaporator)";
        let result = format(input);
        assert_eq!(
            result,
            "    ttl:(空调 or \"air condition\" or 空气调节)\nand tac:(蒸发器 or evaporator)\n"
        );
    }

    // ── Idempotency ──

    #[test]
    fn already_formatted_returns_none() {
        let formatted = "    ttl:空调\nand abst:蒸发器\n";
        let config = ConfigurationBuilder::new().build();
        let result = format_text(Path::new("test.patsnap"), formatted, &config).unwrap();
        assert!(result.is_none(), "already formatted text should return None");
    }
}
