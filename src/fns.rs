use crate::*;

pub fn map_fn(f: &ItemFn) -> TokenStream2 {
    let ItemFn { attrs, vis, sig, block } = f;

    let b = map_block(block);

    let tks = quote! { #(#attrs)* #vis #sig #b };
    //return syn::Error::new(Span::call_site(), tks).to_compile_error();
    tks
}

pub fn map_block(block: &Block) -> TokenStream2 {
    let stmts = block.stmts.iter().map(map_stmt);
    quote! { { #(#stmts)* } }
}

pub fn map_stmt(stmt: &Stmt) -> TokenStream2 {
    match stmt {
        Stmt::Local(local) => map_local(local),
        Stmt::Item(item) => map_item(item),
        Stmt::Expr(expr) => map_expr(expr),
        Stmt::Semi(expr, semi) => {
            let e = map_expr(expr);
            quote! { #e #semi }
        }
    }
}

pub fn map_local(local: &Local) -> TokenStream2 {
    let Local {
        attrs,
        let_token,
        pat,
        init,
        semi_token,
    } = local;
    let init = init.as_ref().map(|(eq, expr)| {
        let e = map_expr(expr);
        quote! { #eq #e }
    });
    quote! {
        #(#attrs)* #let_token #pat #init #semi_token
    }
}
