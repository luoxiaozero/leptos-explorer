use proc_macro2::{token_stream, Span, TokenStream, TokenTree};
use rstml::node::Node;
use syn::{spanned::Spanned, Macro};

pub struct ViewMacro<'a> {
    pub parent_indent: ParentIndent,
    pub cx: Option<TokenTree>,
    pub global_class: Option<TokenTree>,
    pub nodes: Vec<Node>,
    pub span: Span,
    pub mac: &'a Macro,
    pub comma: Option<TokenTree>,
}

impl<'a> ViewMacro<'a> {
    pub fn try_parse(parent_indent: ParentIndent, mac: &'a Macro) -> Option<Self> {
        let mut tokens = mac.tokens.clone().into_iter();
        let (cx, comma) = (tokens.next(), tokens.next());

        let mut no_explicit_scope = true;

        // If the second token is not a comma, then leptos 0.5+ is being used, where reactive scope does not have to be manually specified.
        if let Some(TokenTree::Punct(punct)) = &comma {
            if punct.as_char() == ',' {
                no_explicit_scope = false;
            }
        };

        let (cx, comma) = if no_explicit_scope {
            tokens = [cx, comma]
                .into_iter()
                .flatten()
                .chain(tokens)
                .collect::<TokenStream>()
                .into_iter();
            (None, None)
        } else {
            (cx, comma)
        };

        let (tokens, global_class) = extract_global_class(tokens)?;

        let span = mac.span();
        let nodes = rstml::parse2(tokens).ok()?;

        Some(Self {
            parent_indent,
            global_class,
            nodes,
            span,
            mac,
            cx,
            comma,
        })
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

#[derive(Default)]
pub struct ParentIndent {
    pub tabs: usize,
    pub spaces: usize,
}
