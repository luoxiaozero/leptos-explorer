use super::view_macro::ViewMacro;
use crop::Rope;
use syn::{
    visit::{self, Visit},
    File, Macro,
};

pub fn collect_macros_in_file<'a>(
    file: &'a File,
    source: Rope,
    macro_names: &'a Vec<String>,
) -> (Rope, Vec<ViewMacro>) {
    let mut visitor = ViewMacroVisitor {
        source,
        macros: Vec::new(),
        macro_names,
    };

    visitor.visit_file(file);
    (visitor.source, visitor.macros)
}

struct ViewMacroVisitor<'a> {
    macros: Vec<ViewMacro>,
    source: Rope,
    macro_names: &'a Vec<String>,
}

impl<'ast> Visit<'ast> for ViewMacroVisitor<'ast> {
    fn visit_macro(&mut self, node: &'ast Macro) {
        let should_format = self
            .macro_names
            .iter()
            .any(|macro_name| &get_macro_full_path(node) == macro_name);

        if should_format {
            if let Some(view_mac) = ViewMacro::try_parse(node) {
                self.macros.push(view_mac);
            }
        }

        visit::visit_macro(self, node);
    }
}

fn get_macro_full_path(mac: &syn::Macro) -> String {
    mac.path
        .segments
        .iter()
        .map(|path| path.ident.to_string())
        .collect::<Vec<String>>()
        .join("::")
}
