use super::Optimizer;
use crate::{compress::optimize::is_left_access_to_arguments, mode::Mode};
use std::iter::repeat_with;
use swc_atoms::js_word;
use swc_common::{util::take::Take, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_utils::{find_ids, ident::IdentLike, private_ident, Id};
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

/// Methods related to the option `arguments`.
impl<M> Optimizer<'_, M>
where
    M: Mode,
{
    ///
    /// - `arguments['foo']` => `arguments.foo`
    pub(super) fn optimize_str_access_to_arguments(&mut self, e: &mut Expr) {
        if !self.options.arguments {
            return;
        }

        match e {
            Expr::Member(MemberExpr { prop, .. }) => {
                if let MemberProp::Computed(c) = prop {
                    if let Expr::Lit(Lit::Str(str)) = &mut *c.expr {
                        if !str.value.starts_with(|c: char| c.is_ascii_alphabetic()) {
                            return;
                        }

                        self.changed = true;
                        tracing::debug!("arguments: Optimizing computed access to arguments");
                        *prop = MemberProp::Ident(Ident {
                            span: str.span,
                            sym: str.take().value,
                            optional: false,
                        })
                    }
                }
            }

            Expr::SuperProp(SuperPropExpr { prop, .. }) => {
                if let SuperProp::Computed(c) = prop {
                    if let Expr::Lit(Lit::Str(str)) = &mut *c.expr {
                        if !str.value.starts_with(|c: char| c.is_ascii_alphabetic()) {
                            return;
                        }

                        self.changed = true;
                        tracing::debug!("arguments: Optimizing computed access to arguments");
                        *prop = SuperProp::Ident(Ident {
                            span: str.span,
                            sym: str.take().value,
                            optional: false,
                        })
                    }
                }
            }

            _ => (),
        };
    }

    pub(super) fn optimize_usage_of_arguments(&mut self, f: &mut Function) {
        if !self.options.arguments {
            return;
        }

        if f.params.iter().any(|param| match &param.pat {
            Pat::Ident(BindingIdent {
                id:
                    Ident {
                        sym: js_word!("arguments"),
                        ..
                    },
                ..
            }) => true,
            Pat::Ident(i) => self
                .data
                .as_ref()
                .and_then(|v| v.vars.get(&i.id.to_id()))
                .map(|v| v.declared_count >= 2)
                .unwrap_or(false),
            _ => true,
        }) {
            return;
        }

        {
            // If a function has a variable named `arguments`, we abort.
            let data: Vec<Id> = find_ids(&f.body);
            for id in &data {
                if id.0 == js_word!("arguments") {
                    return;
                }
            }
        }

        let mut v = ArgReplacer {
            params: &mut f.params,
            changed: false,
            keep_fargs: self.options.keep_fargs,
            prevent: false,
        };

        // We visit body two time, to use simpler logic in `inject_params_if_required`
        f.body.visit_mut_children_with(&mut v);
        f.body.visit_mut_children_with(&mut v);

        self.changed |= v.changed;
    }
}

struct ArgReplacer<'a> {
    params: &'a mut Vec<Param>,
    changed: bool,
    keep_fargs: bool,
    prevent: bool,
}

impl ArgReplacer<'_> {
    fn inject_params_if_required(&mut self, idx: usize) {
        if idx < self.params.len() || self.keep_fargs {
            return;
        }
        let new_args = idx + 1 - self.params.len();

        self.changed = true;
        tracing::debug!("arguments: Injecting {} parameters", new_args);
        let mut start = self.params.len();
        self.params.extend(
            repeat_with(|| {
                let p = Param {
                    span: DUMMY_SP,
                    decorators: Default::default(),
                    pat: Pat::Ident(private_ident!(format!("argument_{}", start)).into()),
                };
                start += 1;
                p
            })
            .take(new_args),
        )
    }
}

impl VisitMut for ArgReplacer<'_> {
    noop_visit_mut_type!();

    /// Noop.
    fn visit_mut_arrow_expr(&mut self, _: &mut ArrowExpr) {}

    fn visit_mut_assign_expr(&mut self, n: &mut AssignExpr) {
        n.visit_mut_children_with(self);

        if is_left_access_to_arguments(&n.left) {
            self.prevent = true;
        }
    }

    fn visit_mut_expr(&mut self, n: &mut Expr) {
        if self.prevent {
            return;
        }

        n.visit_mut_children_with(self);

        if let Expr::Member(MemberExpr {
            obj,
            prop: MemberProp::Computed(c),
            ..
        }) = n
        {
            if let Expr::Ident(Ident {
                sym: js_word!("arguments"),
                ..
            }) = &**obj
            {
                match &*c.expr {
                    Expr::Lit(Lit::Str(Str { value, .. })) => {
                        let idx = value.parse::<usize>();
                        let idx = match idx {
                            Ok(v) => v,
                            _ => return,
                        };

                        self.inject_params_if_required(idx);

                        if let Some(param) = self.params.get(idx) {
                            if let Pat::Ident(i) = &param.pat {
                                self.changed = true;
                                *n = Expr::Ident(i.id.clone());
                            }
                        }
                    }
                    Expr::Lit(Lit::Num(Number { value, .. })) => {
                        if value.fract() != 0.0 {
                            // We ignores non-integer values.
                            return;
                        }

                        let idx = value.round() as i64 as usize;

                        self.inject_params_if_required(idx);

                        //
                        if let Some(param) = self.params.get(idx) {
                            if let Pat::Ident(i) = &param.pat {
                                tracing::debug!(
                                    "arguments: Replacing access to arguments to normal reference",
                                );
                                self.changed = true;
                                *n = Expr::Ident(i.id.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Noop.
    fn visit_mut_function(&mut self, _: &mut Function) {}

    fn visit_mut_member_expr(&mut self, n: &mut MemberExpr) {
        if self.prevent {
            return;
        }

        n.obj.visit_mut_with(self);

        if let MemberProp::Computed(c) = &mut n.prop {
            c.visit_mut_with(self);
        }
    }

    fn visit_mut_super_prop_expr(&mut self, n: &mut SuperPropExpr) {
        if self.prevent {
            return;
        }

        if let SuperProp::Computed(c) = &mut n.prop {
            c.visit_mut_with(self);
        }
    }
}
