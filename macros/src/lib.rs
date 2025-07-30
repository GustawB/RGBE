extern crate proc_macro;
use proc_macro::{TokenStream};
use proc_macro2::Span;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Block, ExprParen, Ident, ItemFn, Path, Result, Token};
use quote::{quote, ToTokens};

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

#[proc_macro_attribute]
pub fn arg_register(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str: &str = &attr.to_string();
    let mut item_fun: ItemFn = parse_macro_input!(item as ItemFn);
    let permitted: Vec<&str>= vec!["r8", "r16"];

    if !permitted.contains(&attr_str) {
        panic!("Invalid register parameter. Possible args are: r8; r16")
    }
    
    let val_ident = Ident::new(&format!("{}_val", attr_str), Span::call_site());
    let reg_ident = Ident::new(&format!("{}_reg", attr_str), Span::call_site());
    let reg_param_ident = Ident::new(attr_str, Span::call_site());
    item_fun.block.stmts.insert(0, syn::parse(quote! { let #val_ident: u8; }.into()).unwrap());
    item_fun.block.stmts.insert(1, syn::parse(quote! { let #reg_ident: &Value = &console.registers[RegSize::Byte(#reg_param_ident)]; }.into()).unwrap());
    item_fun.block.stmts.insert(2, syn::parse(quote! { match_value!(#reg_ident, Value::Byte(r) => { #val_ident = **r; }); }.into()).unwrap());

    item_fun.into_token_stream().into()
}
