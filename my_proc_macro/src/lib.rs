extern crate proc_macro;


use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    
    // parse input fn
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);

    // create string
    let fn_str: String = format!("{}", input_fn.to_token_stream());

    // define new function
    let fn_ident: proc_macro2::Ident = input_fn.sig.ident;
    let fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> = input_fn.sig.inputs;
    let fn_generics: syn::Generics = input_fn.sig.generics;

    // generate output fn
    let output: proc_macro2::TokenStream = quote!{
        pub fn #fn_ident #fn_generics(#fn_inputs) -> &'static str {
            #fn_str
        }
    };
    output.into()
}