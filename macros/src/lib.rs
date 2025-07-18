extern crate proc_macro;
use proc_macro::{TokenStream};
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Block, ExprParen, Ident, Path, Result, Token};
use quote::quote;

struct MakeAnswerInput {
    ident: Ident,
    _comma: Token![,],
    pat: Path,
    parenIdent: ExprParen,
    _arrow: Token![=>],
    block: Block,
}

impl Parse for MakeAnswerInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MakeAnswerInput {
           ident: input.parse()?,
            _comma: input.parse()?,
            pat: input.parse()?,
            parenIdent: input.parse()?,
            _arrow: input.parse()?,
            block: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn match_value(_item: TokenStream) -> TokenStream {
    let MakeAnswerInput { ident, pat, parenIdent, block, .. } = parse_macro_input!(_item as MakeAnswerInput);
    quote! {
        match #ident {
            #pat #parenIdent => #block,
            _ => panic!("Invalid register size returned"),
        }
    }.into()
}
