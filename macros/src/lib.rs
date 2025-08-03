extern crate proc_macro;
use proc_macro::{TokenStream};
use proc_macro2::{Span};
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

    match attr_str {
        "r8" => {
            item_fun.block.stmts.insert(0, syn::parse(quote! { let r8_val: u8; }.into()).unwrap());
            item_fun.block.stmts.insert(1, syn::parse(quote! { let r8_reg: &Value = &console.registers[RegSize::Byte(r8)]; }.into()).unwrap());
            item_fun.block.stmts.insert(2, syn::parse(quote! { match_value!(r8_reg, Value::Byte(r) => { r8_val = **r; }); }.into()).unwrap());
        },
        "r16" => {
            item_fun.block.stmts.insert(0, syn::parse(quote! { let r16_val: u16; }.into()).unwrap());
            item_fun.block.stmts.insert(1, syn::parse(quote! { let r16_reg: &Value = &console.registers[RegSize::Word(r16)]; }.into()).unwrap());
            item_fun.block.stmts.insert(2, syn::parse(quote! { match_value!(r16_reg, Value::Word(r) => { r16_val = **r; }); }.into()).unwrap());
        },
        "a" => {
            item_fun.block.stmts.insert(0, syn::parse(quote! { let a_val: u8; }.into()).unwrap());
            item_fun.block.stmts.insert(1, syn::parse(quote! { let a_reg: &Value = &console.registers[RegSize::Byte(A)]; }.into()).unwrap());
            item_fun.block.stmts.insert(2, syn::parse(quote! { match_value!(a_reg, Value::Byte(r) => { a_val = **r; }); }.into()).unwrap());
        },
        "c" => {
            item_fun.block.stmts.insert(0, syn::parse(quote! { let c_val: u8; }.into()).unwrap());
            item_fun.block.stmts.insert(1, syn::parse(quote! { let c_reg: &Value = &console.registers[RegSize::Byte(C)]; }.into()).unwrap());
            item_fun.block.stmts.insert(2, syn::parse(quote! { match_value!(c_reg, Value::Byte(r) => { c_val = **r; }); }.into()).unwrap());
        },
        "hl" => {
            item_fun.block.stmts.insert(0, syn::parse(quote! { let hl_val: u16; }.into()).unwrap());
            item_fun.block.stmts.insert(1, syn::parse(quote! { let hl_reg: &Value = &console.registers[RegSize::Word(HL)]; }.into()).unwrap());
            item_fun.block.stmts.insert(2, syn::parse(quote! { match_value!(hl_reg, Value::Word(r) => { hl_val = **r; }); }.into()).unwrap());    
        },
        _ => panic!("Invalid register parameter. Possible args are: r8; r16; a; c; hl"),
    };

    item_fun.into_token_stream().into()
}
