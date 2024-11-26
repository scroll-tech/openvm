#![feature(proc_macro_diagnostic)]

use std::sync::atomic::AtomicUsize;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Token,
};

static IDX: AtomicUsize = AtomicUsize::new(0);

struct DeclareArgs {
    name: syn::Ident,
    num: u32,
}

impl Parse for DeclareArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let num: syn::LitInt = input.parse()?;
        Ok(Self {
            name,
            num: num.base10_parse()?,
        })
    }
}

#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
    let DeclareArgs { name, num } = parse_macro_input!(input);
    let span = proc_macro::Span::call_site();
    let new_name = syn::Ident::new(
        &format!(
            "{}_{}",
            name.to_string().as_str(),
            IDX.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        ),
        span.into(),
    );
    proc_macro::Diagnostic::new(
        proc_macro::Level::Warning,
        format!("Current name: {}", new_name),
    )
    .emit();
    TokenStream::from(quote! {
        pub struct #name;
        impl #name {
            pub const NUM: u32 = #num;
            pub fn print(&self) {
                println!("{}: num = {}", stringify!(#new_name), Self::NUM);
            }
        }
    })
}
