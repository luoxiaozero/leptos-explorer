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

                let open_tag_span = open_tag.span();
                let open_tag_start_span = open_tag_span.start();
                let start = open_tag_start_span.line - 1;

                let end = if let Some(close_tag) = close_tag {
                    // <div>\n</div>
                    let close_tag_span = close_tag.span();
                    let close_tag_start_span = close_tag_span.start();
                    close_tag_start_span.line - 2
                } else if open_tag.is_self_closed() {
                    // <input \n title="" \n />
                    let end_tag_span = open_tag.end_tag.span();
                    let end_tag_end_span = end_tag_span.start();
                    end_tag_end_span.line - 2
                } else {
                    return children_ranges;
                };

                if start < end {
                    let range = FoldingRange { start, end };
                    let mut ranges = vec![range];
                    ranges.extend(children_ranges);
                    ranges
                } else {
                    children_ranges
                }
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
