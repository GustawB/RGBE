extern crate proc_macro;
use proc_macro::{TokenStream};
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Block, ExprParen, Ident, Path, Result, Token};
use quote::quote;

struct MakeAnswerInput {
    ident: Ident,
    _comma: Token![,],
    pat: Path,
    paren_ident: ExprParen,
    _arrow: Token![=>],
    block: Block,
}

impl Parse for MakeAnswerInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MakeAnswerInput {
           ident: input.parse()?,
            _comma: input.parse()?,
            pat: input.parse()?,
            paren_ident: input.parse()?,
            _arrow: input.parse()?,
            block: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn match_value(_item: TokenStream) -> TokenStream {
    let MakeAnswerInput { ident, _comma, pat, paren_ident, block, .. } = parse_macro_input!(_item as MakeAnswerInput);
    quote! {
        match #ident {
            #pat #paren_ident => #block,
            _ => panic!("Invalid register size returned"),
        }
    }.into()
}
