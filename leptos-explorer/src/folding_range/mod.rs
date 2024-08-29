mod collect_folding_range;
mod collect_macros;
mod view_macro;

use collect_folding_range::{collect_node_folding_range, FoldingRange};
use collect_macros::collect_macros_in_file;
use crop::Rope;

pub fn folding_range(path: String) -> Vec<FoldingRange> {
    let source = std::fs::read_to_string(path).unwrap();
    folding_range_with_source(source)
}

fn folding_range_with_source(source: String) -> Vec<FoldingRange> {
    let Ok(ast) = syn::parse_file(&source) else {
        return vec![];
    };

    let macro_names = vec!["leptos::view".to_string(), "view".to_string()];
    let rope = Rope::from(source);
    let (_, macros) = collect_macros_in_file(&ast, rope, &macro_names);

    macros
        .into_iter()
        .map(|mac| collect_node_folding_range(mac.nodes))
        .flatten()
        .collect()
}

#[test]
fn test_folding_range_with_source() {
    let source = String::from("pub fn Comp() { view! { <div>\n   <span>\"123\"</span>\n</div>} }");
    let ranges = folding_range_with_source(source);
    assert_eq!(ranges, vec![FoldingRange { start: 1, end: 3 }]);
}
