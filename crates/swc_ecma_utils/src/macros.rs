/// Shortcut for `quote_ident!(span.apply_mark(Mark::fresh(Mark::root())), s)`
#[macro_export]
macro_rules! private_ident {
    ($s:expr) => {
        private_ident!(::swc_common::DUMMY_SP, $s)
    };
    ($span:expr, $s:expr) => {{
        use swc_common::Mark;
        let mark = Mark::fresh(Mark::root());
        let span = $span.apply_mark(mark);
        $crate::swc_ecma_ast::Ident::new($s.into(), span)
    }};
}

#[macro_export]
macro_rules! quote_ident {
    ($s:expr) => {
        quote_ident!(::swc_common::DUMMY_SP, $s)
    };
    ($span:expr, $s:expr) => {{
        $crate::swc_ecma_ast::Ident::new($s.into(), $span)
    }};
}

#[macro_export]
macro_rules! quote_str {
    ($s:expr) => {
        quote_str!(::swc_common::DUMMY_SP, $s)
    };
    ($span:expr, $s:expr) => {{
        $crate::swc_ecma_ast::Str {
            span: $span,
            value: $s.into(),
            has_escape: false,
            kind: Default::default(),
        }
    }};
}

#[macro_export]
macro_rules! quote_expr {
    ($span:expr, null) => {{
        use $crate::swc_ecma_ast::*;
        Expr::Lit(Lit::Null(Null { span: $span }))
    }};

    ($span:expr, undefined) => {{
        box Expr::Ident(Ident::new(js_word!("undefined"), $span))
    }};
}

/// Creates a member expression.
///
/// # Usage
/// ```rust,ignore
/// member_expr!(span, Function.bind.apply);
/// ```
///
/// Returns Box<[Expr](swc_ecma_ast::Expr)>.
#[macro_export]
macro_rules! member_expr {
    ($span:expr, $first:ident) => {{
        use $crate::swc_ecma_ast::Expr;
        Box::new(Expr::Ident($crate::quote_ident!($span, stringify!($first))))
    }};

    ($span:expr, $first:ident . $($rest:tt)+) => {{
        let obj = member_expr!($span, $first);

        member_expr!(@EXT, $span, obj, $($rest)* )
    }};

    (@EXT, $span:expr, $obj:expr, $first:ident . $($rest:tt)* ) => {{
        use $crate::swc_ecma_ast::MemberProp;
        let prop = MemberProp::Ident($crate::quote_ident!($span, stringify!($first)));

        member_expr!(@EXT, $span, Box::new(Expr::Member(MemberExpr{
            span: ::swc_common::DUMMY_SP,
            obj: $obj,
            prop,
        })), $($rest)*)
    }};

    (@EXT, $span:expr, $obj:expr,  $first:ident) => {{
        use $crate::swc_ecma_ast::*;
        let prop = MemberProp::Ident($crate::quote_ident!($span, stringify!($first)));

        Box::new(Expr::Member(MemberExpr{
            span: ::swc_common::DUMMY_SP,
            obj: $obj,
            prop,
        }))
    }};
}

#[cfg(test)]
mod tests {
    use crate::drop_span;
    use swc_common::DUMMY_SP as span;
    use swc_ecma_ast::*;

    #[test]
    fn quote_member_expr() {
        let expr: Box<Expr> = drop_span(member_expr!(span, Function.prototype.bind));

        assert_eq!(
            expr,
            Box::new(Expr::Member(MemberExpr {
                span,
                obj: Box::new(Expr::Member(MemberExpr {
                    span,
                    obj: member_expr!(span, Function),
                    prop: MemberProp::Ident(quote_ident!(span, "prototype")),
                })),
                prop: MemberProp::Ident(quote_ident!(span, "bind")),
            }))
        );
    }
}
