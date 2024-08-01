use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    pub span: Span,
    pub kind: ErrorKind,
}

pub enum ErrorKind {
    TooManyAttributes,
    OnlyNormalArgumentsAllowed,
}

impl Error {
    pub fn emit(&self) -> TokenStream {
        let message = format!("Error in nasl_function: {}", self.message());
        quote_spanned! {
            self.span =>
            compile_error!(#message);
        }
    }

    pub fn message(&self) -> String {
        match self.kind {
            ErrorKind::OnlyNormalArgumentsAllowed => {
                "Only normal identifier arguments are allowed on the function.".into()
            }
            ErrorKind::TooManyAttributes => {
                "Argument is named more than once in attributes.".into()
            }
        }
    }
}
