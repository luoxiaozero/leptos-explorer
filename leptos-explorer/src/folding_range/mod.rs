mod view_macro;

use crop::Rope;
use rstml::node::Node;
use syn::{
    spanned::Spanned,
    visit::{self, Visit},
    File, Macro,
};
use view_macro::{ParentIndent, ViewMacro};

pub fn folding_range(path: String) -> Vec<(usize, usize)> {
    let source = std::fs::read_to_string(path).unwrap();
    folding_range_with_source(source)
}

fn folding_range_with_source(source: String) -> Vec<(usize, usize)> {
    let Ok(ast) = syn::parse_file(&source) else {
        return vec![];
    };

    let macro_names = vec!["leptos::view".to_string(), "view".to_string()];
    let rope = Rope::from(source);
    let (_, macros) = collect_macros_in_file(&ast, rope, &macro_names);

    macros
        .into_iter()
        .map(|mac| {
            mac.nodes
                .into_iter()
                .map(|node| match node {
                    Node::Element(el) => {
                        if let Some(close_tag) = el.close_tag {
                            let open_span = el.open_tag.span();
                            let close_span = close_tag.span();
                            Some((open_span.start().line, close_span.start().line))
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .filter_map(|range| range)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn collect_macros_in_file<'a>(
    file: &'a File,
    source: Rope,
    macro_names: &'a Vec<String>,
) -> (Rope, Vec<ViewMacro<'a>>) {
    let mut visitor = ViewMacroVisitor {
        source,
        macros: Vec::new(),
        macro_names,
    };

    visitor.visit_file(file);
    (visitor.source, visitor.macros)
}

struct ViewMacroVisitor<'a> {
    macros: Vec<ViewMacro<'a>>,
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
            let span_line = node.span().start().line;
            let line = self.source.line(span_line - 1);

            let indent_chars: Vec<_> = line
                .chars()
                .take_while(|&c| c == ' ' || c == '\t')
                .collect();

            let tabs = indent_chars.iter().filter(|&&c| c == '\t').count();
            let spaces = indent_chars.iter().filter(|&&c| c == ' ').count();

            if let Some(view_mac) = ViewMacro::try_parse(ParentIndent { tabs, spaces }, node) {
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

#[test]
fn test_folding_range_with_source() {
    let source = String::from("pub fn Comp() { view! { <div>\n   <span>\"123\"</span>\n</div>} }");
    let ranges = folding_range_with_source(source);
    assert_eq!(ranges, vec![(1, 3)]);
}
