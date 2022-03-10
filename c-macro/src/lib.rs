#![feature(proc_macro_span)]

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn c(_strm: TokenStream) -> TokenStream {
    let strm: TokenStream = _strm.clone().into();

    let _captures: HashMap<String, u64> = HashMap::new();

    let cap_vec: Vec<u64> = vec![];



    let mut tok = strm.into_iter();
    let mut span = tok.next().unwrap().span();
    while let Some(t) = tok.next() {
        span = span.join(t.span()).unwrap();
    }

    let src = span.source_text().unwrap()
        .replace("{", "{{")
        .replace("}", "}}");

    let tk = quote!(
        {
            let (a, b) = runtime_c::compile_c(&format!(#src, #(#cap_vec)*));
            runtime_c::do_horrible_crimes::<()>(a, b);
        }
    ).into();

    eprintln!("{:#}", tk);

    tk
}