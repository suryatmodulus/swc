use crate::perf::Check;
use swc_atoms::JsWord;
use swc_ecma_ast::*;
use swc_ecma_transforms_macros::fast_path;
use swc_ecma_visit::{noop_fold_type, Fold, FoldWith, Node, Visit};

pub fn reserved_words() -> impl 'static + Fold {
    EsReservedWord {}
}

struct EsReservedWord {}

#[fast_path(ShouldWork)]
impl Fold for EsReservedWord {
    noop_fold_type!();

    fn fold_export_specifier(&mut self, n: ExportSpecifier) -> ExportSpecifier {
        n
    }

    fn fold_ident(&mut self, i: Ident) -> Ident {
        let sym = rename_ident(i.sym, true);

        Ident { sym, ..i }
    }

    fn fold_import_named_specifier(&mut self, s: ImportNamedSpecifier) -> ImportNamedSpecifier {
        if s.imported.is_some() {
            ImportNamedSpecifier {
                local: s.local.fold_with(self),
                ..s
            }
        } else {
            ImportNamedSpecifier {
                imported: s.imported.fold_with(self),
                ..s
            }
        }
    }

    fn fold_member_expr(&mut self, e: MemberExpr) -> MemberExpr {
        if e.computed {
            MemberExpr {
                obj: e.obj.fold_with(self),
                prop: e.prop.fold_with(self),
                ..e
            }
        } else {
            MemberExpr {
                obj: e.obj.fold_with(self),
                ..e
            }
        }
    }

    fn fold_prop_name(&mut self, n: PropName) -> PropName {
        n
    }
}

fn is_reserved(sym: &JsWord) -> bool {
    match &**sym {
        "enum" | "implements" | "package" | "protected" | "interface" | "private" | "public"
        | "await" | "break" | "case" | "catch" | "class" | "const" | "continue" | "debugger"
        | "default" | "delete" | "do" | "else" | "export" | "extends" | "finally" | "for"
        | "function" | "if" | "in" | "instanceof" | "new" | "return" | "super" | "switch"
        | "this" | "throw" | "try" | "typeof" | "var" | "void" | "while" | "with" | "yield" => true,

        _ => false,
    }
}

fn rename_ident(sym: JsWord, _strict: bool) -> JsWord {
    // Es
    if is_reserved(&sym) {
        format!("_{}", sym).into()
    } else {
        sym
    }
}
#[derive(Default)]
struct ShouldWork {
    found: bool,
}

impl Visit for ShouldWork {
    fn visit_ident(&mut self, i: &Ident, _: &dyn Node) {
        if is_reserved(&i.sym) {
            self.found = true;
            return;
        }
    }
}

impl Check for ShouldWork {
    fn should_handle(&self) -> bool {
        self.found
    }
}
