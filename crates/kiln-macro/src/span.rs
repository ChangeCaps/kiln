use std::collections::BTreeMap;

use proc_macro2::{Delimiter, Spacing, TokenTree};
use proc_macro_error::{Diagnostic, Level};

pub trait SpannedResult {
    type Ok;

    fn spanned_unwrap(self, source: &SpannedSource) -> Self::Ok;
}

impl<T> SpannedResult for Result<T, naga::front::wgsl::ParseError> {
    type Ok = T;

    fn spanned_unwrap(self, source: &SpannedSource) -> <Self as SpannedResult>::Ok {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => {
                let (_line, column) = error.location(&source.source);

                let span = source.get_span(column);

                Diagnostic::spanned(*span, Level::Error, error.to_string()).abort();
            }
        }
    }
}

impl<T> SpannedResult for Result<T, naga::WithSpan<naga::valid::ValidationError>> {
    type Ok = T;

    fn spanned_unwrap(self, source: &SpannedSource) -> <Self as SpannedResult>::Ok {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => {
                for (span, error) in error.spans() {
                    let span = source.get_span(span.to_range().unwrap().start);

                    Diagnostic::spanned(*span, Level::Error, error.clone()).emit();
                }

                Diagnostic::new(Level::Error, error.to_string()).abort();
            }
        }
    }
}

#[derive(Default)]
pub struct SpannedSource {
    pub spans: BTreeMap<usize, proc_macro2::Span>,
    pub source: String,
}

impl SpannedSource {
    pub fn get_span(&self, start: usize) -> &proc_macro2::Span {
        if let Some(span) = self.spans.get(&start) {
            return span;
        }

        for (span_start, span) in self.spans.iter().rev() {
            if start >= *span_start {
                return span;
            }
        }

        proc_macro_error::abort_call_site! {
            "span not found"
        }
    }

    #[inline]
    pub fn new(source: &proc_macro2::TokenStream) -> Self {
        let mut this = Self::default();

        for tree in source.clone() {
            this.add_tree(tree);
        }

        this
    }

    pub fn add_tree(&mut self, tree: TokenTree) {
        match tree {
            TokenTree::Group(group) => {
                match group.delimiter() {
                    Delimiter::Parenthesis => {
                        self.add_string("(", group.span_open());
                    }
                    Delimiter::Brace => {
                        self.add_string("{", group.span_open());
                    }
                    Delimiter::Bracket => {
                        self.add_string("[", group.span_open());
                    }
                    _ => {}
                }

                for tree in group.stream() {
                    self.add_tree(tree);
                }

                match group.delimiter() {
                    Delimiter::Parenthesis => {
                        self.add_string(")", group.span_open());
                    }
                    Delimiter::Brace => {
                        self.add_string("}", group.span_open());
                    }
                    Delimiter::Bracket => {
                        self.add_string("]", group.span_open());
                    }
                    _ => {}
                }
            }
            TokenTree::Ident(ident) => {
                self.add_string(&ident.to_string(), ident.span());
                self.source.push(' ');
            }
            TokenTree::Literal(lit) => {
                self.add_string(&lit.to_string(), lit.span());
                self.source.push(' ');
            }
            TokenTree::Punct(punct) => {
                self.add_string(&punct.to_string(), punct.span());

                match punct.spacing() {
                    Spacing::Alone => self.source.push(' '),
                    Spacing::Joint => {}
                }
            }
        }
    }

    pub fn add_string(&mut self, string: &str, span: proc_macro2::Span) {
        let start = self.source.len();

        self.source += string;

        self.spans.insert(start, span);
    }
}
