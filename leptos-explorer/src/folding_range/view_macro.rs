use proc_macro2::{token_stream, TokenStream, TokenTree};
use rstml::node::Node;
use syn::Macro;

pub struct ViewMacro {
    pub nodes: Vec<Node>,
}

impl ViewMacro {
    pub fn try_parse<'a>(mac: &'a Macro) -> Option<Self> {
        let tokens = mac.tokens.clone().into_iter();
        let (tokens, _) = extract_global_class(tokens)?;
        let nodes = rstml::parse2(tokens).ok()?;
        Some(Self { nodes })
    }
}

fn extract_global_class(
    mut tokens: token_stream::IntoIter,
) -> Option<(TokenStream, Option<TokenTree>)> {
    let first = tokens.next();
    let second = tokens.next();
    let third = tokens.next();
    let fourth = tokens.next();
    let global_class = match (&first, &second) {
        (Some(TokenTree::Ident(first)), Some(TokenTree::Punct(eq)))
            if *first == "class" && eq.as_char() == '=' =>
        {
            match &fourth {
                Some(TokenTree::Punct(comma)) if comma.as_char() == ',' => third.clone(),
                _ => {
                    return None;
                }
            }
        }
        _ => None,
    };

    let tokens = if global_class.is_some() {
        tokens.collect::<proc_macro2::TokenStream>()
    } else {
        [first, second, third, fourth]
            .into_iter()
            .flatten()
            .chain(tokens)
            .collect()
    };

    Some((tokens, global_class))
}
