use proc_macro2::Span;
use std::fmt::{self, Display};
use syn::parse::{Error, Result};

#[derive(Clone)]
pub struct UeAttr {
    text: String,
}

impl UeAttr {
    pub fn parse(text: &str, span: Span) -> Result<Self> {
        match syn::parse_str::<syn::Lit>(text) {
            Ok(literal) => {
                let text = match literal {
                    syn::Lit::Str(s) => s.value(),
                    _ => todo!(),
                };
                Ok(UeAttr { text })
            }
            Err(err) => Err(Error::new(span, err)),
        }
    }
}

impl Display for UeAttr {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}

impl PartialEq<str> for UeAttr {
    fn eq(&self, rhs: &str) -> bool {
        self.text == rhs
    }
}
