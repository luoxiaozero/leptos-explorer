use rstml::node::{Node, NodeElement};
use syn::spanned::Spanned;

pub fn collect_node_folding_range(nodes: Vec<Node>) -> Vec<FoldingRange> {
    nodes
        .into_iter()
        .map(|node| match node {
            Node::Element(el) => {
                let NodeElement {
                    open_tag,
                    close_tag,
                    children,
                } = el;
                let children_ranges = collect_node_folding_range(children);

                let Some(close_tag) = close_tag else {
                    return children_ranges;
                };

                let open_span = open_tag.span();
                let close_span = close_tag.span();

                let start = open_span.start().line - 1;
                let end = if close_span.start().line > open_span.start().line {
                    close_span.start().line - 2
                } else {
                    return children_ranges;
                };
                let range = FoldingRange { start, end };
                let mut ranges = vec![range];
                ranges.extend(children_ranges);
                ranges
            }
            _ => vec![],
        })
        .flatten()
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct FoldingRange {
    /// The zero-based start line of the range to fold. The folded area starts after the line's last character.
    /// To be valid, the end must be zero or larger and smaller than the number of lines in the document.
    pub start: usize,
    /// The zero-based end line of the range to fold. The folded area ends with the line's last character.
    /// To be valid, the end must be zero or larger and smaller than the number of lines in the document.
    pub end: usize,
}
