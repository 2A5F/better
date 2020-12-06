use crate::*;

pub fn map_expr(expr: &Expr) -> TokenStream2 {
    match expr {
        Expr::Array(_) => {}
        Expr::Assign(_) => {}
        Expr::AssignOp(_) => {}
        Expr::Async(_) => {} // todo check block
        Expr::Await(_) => {}
        Expr::Binary(_) => {}
        Expr::Block(block) => return map_expr_block(block),
        Expr::Box(_) => {}
        Expr::Break(_) => {}
        Expr::Call(_) => {} // todo check call
        Expr::Cast(_) => {}
        Expr::Closure(_) => {} // todo check block
        Expr::Continue(_) => {}
        Expr::Field(_) => {}
        Expr::ForLoop(_) => {} // todo check call
        Expr::Group(_) => {}   // todo check sub
        Expr::If(_) => {}      // todo check sub
        Expr::Index(_) => {}   // todo check sub
        Expr::Let(_) => {}     // todo check sub
        Expr::Lit(_) => {}
        Expr::Loop(_) => {} // todo check block
        Expr::Macro(_) => {}
        Expr::Match(_) => {}      // todo check sub
        Expr::MethodCall(_) => {} // todo check call
        Expr::Paren(_) => {}      // todo check sub
        Expr::Path(_) => {}
        Expr::Range(_) => {}     // todo check sub
        Expr::Reference(_) => {} // todo check sub
        Expr::Repeat(_) => {}    // todo check sub
        Expr::Return(_) => {}
        Expr::Struct(_) => {}   // todo check sub
        Expr::Try(_) => {}      // todo check sub
        Expr::TryBlock(_) => {} // todo check sub
        Expr::Tuple(_) => {}    // todo check sub
        Expr::Type(_) => {}
        Expr::Unary(unary) => return map_unary(unary),
        Expr::Unsafe(_) => {} // todo check block
        Expr::Verbatim(_) => {}
        Expr::While(_) => {} // todo check sub
        Expr::Yield(_) => {} // todo check sub
        Expr::__Nonexhaustive => {}
    }
    quote! { #expr }
}

pub fn map_expr_block(block: &ExprBlock) -> TokenStream2 {
    let ExprBlock { attrs, label, block } = block;
    let stmts = block.stmts.iter().map(map_stmt);
    quote! { #(#attrs)* #label { #(#stmts)* } }
}

pub fn map_unary(unary: &ExprUnary) -> TokenStream2 {
    let ExprUnary { op, expr, .. } = unary;
    if let UnOp::Deref(_) = op {
        if let Expr::Unary(ExprUnary { op: UnOp::Deref(_), expr, .. }) = expr.deref() {
            if let Expr::Lit(lit) = expr.deref() {
                if let Lit::Str(s) = &lit.lit {
                    return build_template_string(s);
                }
            }
        }
    }
    quote! { #unary }
}
