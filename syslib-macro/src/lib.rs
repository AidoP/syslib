use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn;

struct ModeInput {
    user: syn::Ident,
    group: syn::Ident,
    other: syn::Ident
}
impl ModeInput {
    fn mode(&self, compile_errors: &mut Vec<TokenStream>) -> u32 {
        Self::parse_ident(&self.user, "user", 6, 2, 'S', compile_errors) |
        Self::parse_ident(&self.group, "group", 3, 1, 'S', compile_errors) |
        Self::parse_ident(&self.other, "other", 0, 0, 'T', compile_errors)
    }
    fn parse_ident(mode: &syn::Ident, domain: &str, shift: u32, top_shift: u32, top_bit: char, compile_errors: &mut Vec<TokenStream>) -> u32 {
        let mut err = |str: &str| compile_errors.push(quote_spanned!{mode.span()=> ::core::compile_error!(#str)});
        let mode = mode.to_string();
        let mut bits = mode.chars();
        let mut match_bit = |b: char, e: char| if b == e {
            1
        } else if b == '_' {
            0
        } else {
            err(&format!("invalid mode bit {b}"));
            0
        };
        match (bits.next(), bits.next(), bits.next(), bits.next(), bits.next()) {
            (Some(r), Some(w), Some(x), top, None) => {
                ((
                    (match_bit(r, 'r') << 2) |
                    (match_bit(w, 'w') << 1) |
                    match_bit(x, 'x')
                ) << shift) |
                (match_bit(top.unwrap_or('_'), top_bit) << (top_shift + 9))
            },
            (_, _, _, _, Some(_)) => {
                err(&format!("extra {domain} mode bits"));
                0
            }
            _ => { 
                err(&format!("missing {domain} mode bits"));
                0
            }
        }
    }
}
impl syn::parse::Parse for ModeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            user: input.parse()?,
            group: input.parse()?,
            other: input.parse()?
        })
    }
}

#[proc_macro]
pub fn mode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mode = syn::parse_macro_input!(input as ModeInput);
    let mut compile_errors = Vec::new();
    let mode = mode.mode(&mut compile_errors);
    let compile_errors = compile_errors.iter();
    quote!{
        ::syslib::open::Mode(#mode) #(| #compile_errors)*
    }.into()
}