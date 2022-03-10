#![feature(proc_macro_span)]
#![feature(extend_one)]
#![feature(proc_macro_span_shrink)]
#![feature(proc_macro_diagnostic)]

use std::collections::HashMap;

use proc_macro::{TokenStream, TokenTree, LineColumn};
use quote::quote;

struct SourceCode {
    source: String,
    line: usize,
    col: usize,
    captures: Vec::<String>
}

impl SourceCode {
    fn new() -> Self {
        Self { source: "".into(), line: 1, col: 0, captures: vec![] }
    }

    // stealing stuff from mara here
    fn add_str(&mut self, s: &str) {
        // Let's assume for now s contains no newlines.
        self.source += s;
        self.col += s.len();
    }

    fn add_whitespace(&mut self, loc: LineColumn) {
        while self.line < loc.line {
            self.source.push('\n');
            self.line += 1;
            self.col = 0;
        }
        while self.col < loc.column {
            self.source.push(' ');
            self.col += 1;
        }
    }

    fn reconstruct_c(&mut self, strm: TokenStream) {
        let mut tokens = strm.clone().into_iter();
    
        while let Some(x) = tokens.next() {
            match x {
                TokenTree::Group(y) => {
                    let s = y.to_string();
                    self.add_whitespace(y.span_open().start());
                    self.add_str(&s[..1]); // the '[', '{' or '('.
                    self.reconstruct_c(y.stream());
                    self.add_whitespace(y.span_close().start());
                    self.add_str(&s[s.len() - 1..]); // the ']', '}' or ')'.
                },
                y @ _ => {
                    if let TokenTree::Punct(v) = y {
                        if v.as_char() == '\'' {
                            let next_tok = tokens.next().unwrap();
                            if let TokenTree::Ident(id) = next_tok {
                                self.add_whitespace(v.span().start());
                                let name = id.to_string();
                                self.add_str(&format!("((void(*)()):ONE_LBRACE::ONE_RBRACE:)"));
                                self.captures.push(name);
                            } else {
                                panic!("expected identifier after '");
                            }
                        } else {
                            self.add_whitespace(v.span().start());
                            self.add_str(&v.to_string());
                        }
                    } else {
                        self.add_whitespace(y.span().start());
                        self.add_str(&y.to_string());
                    }
                }
            }
        }
    }
}


#[proc_macro]
pub fn c(_strm: TokenStream) -> TokenStream {
    //let mut strm: TokenStream = _strm.clone().into();

    let _captures: HashMap<String, u64> = HashMap::new();

    let mut src = SourceCode::new();

    src.reconstruct_c(_strm);
    /*
    let mut tok = strm.into_iter();
    let mut span = tok.next().unwrap().span();
    while let Some(t) = tok.next() {
        span = span.join(t.span()).unwrap();
    }
    */

    let src_text = src.source
        .replace("{", "{{")
        .replace("}", "}}")
        .replace(":ONE_LBRACE:", "{")
        .replace(":ONE_RBRACE:", "}");
    
    let capts = src.captures;

    let mut proper_capts: Vec<proc_macro2::TokenStream> = vec![];

    for i in capts {
        let proper_i = proc_macro2::Ident::new(&i, proc_macro2::Span::call_site());
        proper_capts.push(quote!(((#proper_i as *const()) as u64)));
    }

    let tk = quote!(
        {
            let (a, b) = runtime_c::compile_c(&format!(#src_text,#(#proper_capts),*));
            runtime_c::do_horrible_crimes::<()>(a, b);
        }
    ).into();

    tk
}