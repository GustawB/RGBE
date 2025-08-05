extern crate proc_macro;
use proc_macro::{TokenStream};
use proc_macro2::{Span};
use syn::{parse::{Parse, ParseStream}, parse_macro_input, parse_str, Block, Type, ExprParen, Ident, ItemFn, Path, Result, Token};
use quote::{quote, ToTokens};

struct MatchValueInput {
    ident: Ident,
    _comma: Token![,],
    pat: Path,
    paren_ident: ExprParen,
    _arrow: Token![=>],
    block: Block,
}

impl Parse for MatchValueInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MatchValueInput {
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
    let MatchValueInput { ident, _comma, pat, paren_ident, block, .. } = parse_macro_input!(_item as MatchValueInput);
    quote! {
        match #ident {
            #pat #paren_ident => #block,
            _ => panic!("Invalid register size returned"),
        }
    }.into()
}

fn new_arg_tokens(register: &str) -> (Ident, Type, Ident, Path, Path, ExprParen) {
    let val_ident: Ident = Ident::new(&(register.to_owned() + "_val"), Span::call_site());
    let val_type: Type;
    let reg_ident: Ident = Ident::new(&(register.to_owned() + "_reg"), Span::call_site());
    let regsize_path: Path;
    let valuesize_path: Path;
    let paren_ident: ExprParen;

    match register {
        "r8" => {
            val_type = parse_str("u8").expect("Failed to parse val type");
            regsize_path = parse_str("RegSize::Byte").expect("Failed to parse regsize_path");
            valuesize_path = parse_str("Value::Byte").expect("Failed to parse valuesize_path");
            paren_ident = parse_str("(r8)").expect("Failed to parse paren_ident");
        },
        "r16" => {
            val_type = parse_str("u16").expect("Failed to parse val type");
            regsize_path = parse_str("RegSize::Word").expect("Failed to parse regsize_path");
            valuesize_path = parse_str("Value::Word").expect("Failed to parse valuesize_path");
            paren_ident = parse_str("(r16)").expect("Failed to parse paren_ident");
        },
        "a" => {
            val_type = parse_str("u8").expect("Failed to parse val type");
            regsize_path = parse_str("RegSize::Byte").expect("Failed to parse regsize_path");
            valuesize_path = parse_str("Value::Byte").expect("Failed to parse valuesize_path");
            paren_ident = parse_str("(A)").expect("Failed to parse paren_ident");
        },
        "c" => {
            val_type = parse_str("u8").expect("Failed to parse val type");
            regsize_path = parse_str("RegSize::Byte").expect("Failed to parse regsize_path");
            valuesize_path = parse_str("Value::Byte").expect("Failed to parse valuesize_path");
            paren_ident = parse_str("(C)").expect("Failed to parse paren_ident");
        },
        "hl" => {
            val_type = parse_str("u16").expect("Failed to parse val type");
            regsize_path = parse_str("RegSize::Word").expect("Failed to parse regsize_path");
            valuesize_path = parse_str("Value::Word").expect("Failed to parse valuesize_path");
            paren_ident = parse_str("(HL)").expect("Failed to parse paren_ident");
        },
        _ => panic!("Invalid register parameter. Possible args are: r8; r16; a; c; hl"),
    };

    (val_ident, val_type, reg_ident, regsize_path, valuesize_path, paren_ident)
}

#[proc_macro_attribute]
pub fn arg_register(attr: TokenStream, item: TokenStream) -> TokenStream {
    let register: &str = &attr.to_string();
    let mut item_fun: ItemFn = parse_macro_input!(item as ItemFn);

    let (vi, vt, ri, rp, vp, pi) = new_arg_tokens(register);

    item_fun.block.stmts.insert(0, syn::parse(quote! {
        let #vi: #vt;
    }.into()).expect("Failed to declare #val_ident"));
    item_fun.block.stmts.insert(1, syn::parse(quote! {
        let #ri: &Value = &console.registers[#rp #pi];
    }.into()).expect("Failed to get #reg_ident"));
    item_fun.block.stmts.insert(2, syn::parse(quote! {
        match_value!(#ri, #vp (r) => { #vi = **r;});
    }.into()).expect("Failed to set #val_ident"));

    item_fun.into_token_stream().into()
}
