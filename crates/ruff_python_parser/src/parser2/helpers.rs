use ruff_python_ast::{Expr, ExprContext};
use ruff_text_size::{TextRange, TextSize};

/// Return the range of the string token without the quotes
pub fn remove_str_quotes(str_range: TextRange, prefix_size: TextSize, is_triple_quote: bool) -> TextRange {
    let quote_size = TextSize::from(if is_triple_quote { 3 } else { 1 });
    str_range.add_start(quote_size + prefix_size).sub_end(quote_size)
}

/// Set the `ctx` for `Expr::Id`, `Expression::Attribute`, `Expression::Subscript`,
/// `Expr::Starred`, `Expression::Tuple` and `Expression::List`. If `expr` is either
/// `Expr::Tuple` or `Expression::List`, recursively sets the `ctx` for their elements.
pub fn set_expr_ctx(expr: &mut Expr, ctx: ExprContext) {
    match expr {
        Expr::Name(ident) => ident.ctx = ctx,
        Expr::Attribute(attrib) => attrib.ctx = ctx,
        Expr::Subscript(subscript) => subscript.ctx = ctx,
        Expr::Starred(starred) => starred.ctx = ctx,
        Expr::List(list) => {
            list.ctx = ctx;
            // list.elements.iter_mut().for_each(|element| set_expr_ctx(element, ctx));
        }
        Expr::Tuple(tuple) => {
            tuple.ctx = ctx;
            // tuple.elements.iter_mut().for_each(|element| set_expr_ctx(element, ctx));
        }
        _ => {}
    }
}

pub fn set_expr_range(expr: &mut Expr, range: TextRange) {
    match expr {
        // Expr::Invalid(r) => *r = range,
        Expr::Name(node) => node.range = range,
        Expr::Set(node) => node.range = range,
        Expr::Call(node) => node.range = range,
        Expr::Dict(node) => node.range = range,
        Expr::List(node) => node.range = range,
        Expr::NamedExpr(node) => node.range = range,
        Expr::Yield(node) => node.range = range,
        Expr::Await(node) => node.range = range,
        Expr::Slice(node) => node.range = range,
        Expr::Tuple(node) => node.range = range,
        Expr::BoolOp(node) => node.range = range,
        Expr::IfExp(node) => node.range = range,
        Expr::Lambda(node) => node.range = range,
        Expr::Compare(node) => node.range = range,
        Expr::UnaryOp(node) => node.range = range,
        Expr::FString(node) => node.range = range,
        // Expr::Literal(node) => node.range = range,
        Expr::SetComp(node) => node.range = range,
        Expr::Starred(node) => node.range = range,
        Expr::BinOp(node) => node.range = range,
        Expr::DictComp(node) => node.range = range,
        Expr::ListComp(node) => node.range = range,
        Expr::Attribute(node) => node.range = range,
        Expr::GeneratorExp(node) => node.range = range,
        Expr::Subscript(node) => node.range = range,
        Expr::YieldFrom(node) => node.range = range,
        Expr::FormattedValue(node) => node.range = range,
        _ => todo!(),
    }
}

pub fn is_valid_assignment_target(expr: &Expr) -> bool {
    match expr {
        Expr::Starred(s) => is_valid_assignment_target(&s.value),
        Expr::List(_) | Expr::Tuple(_) => {
            todo!()
            // elements.iter().all(|element| is_valid_assignment_target(element))
        }
        Expr::Name(_) | Expr::Attribute(_) | Expr::Subscript(_) => true,
        _ => false,
    }
}

pub fn is_valid_aug_assignment_target(expr: &Expr) -> bool {
    matches!(expr, Expr::Name(_) | Expr::Attribute(_) | Expr::Subscript(_))
}
