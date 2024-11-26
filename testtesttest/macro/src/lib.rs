#![feature(proc_macro_diagnostic)]

use std::sync::atomic::AtomicUsize;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Token,
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

struct DefineArgs {
    args: Punctuated<syn::LitInt, Token![,]>,
}

impl Parse for DefineArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            args: input.parse_terminated(syn::LitInt::parse, Token![,])?,
        })
    }
}

#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
    let DeclareArgs { name, num } = parse_macro_input!(input);
    let span = proc_macro::Span::call_site();
    let idx = IDX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let new_name = syn::Ident::new(
        &format!("{}_{}", name.to_string().as_str(), idx),
        span.into(),
    );
    let mod_name = syn::Ident::new(&format!("mod_name_lksjfhlahf_{}", idx), span.into());
    let extern_func = syn::Ident::new(&format!("call_print_num_for_{}", num), span.into());
    proc_macro::Diagnostic::new(
        proc_macro::Level::Warning,
        format!("Current name: {}", new_name),
    )
    .emit();
    TokenStream::from(quote! {
        mod #mod_name {
            extern "C" {
                pub fn #extern_func();
            }
        }
        pub struct #name;
        impl #name {
            pub const NUM: u32 = #num;
            pub fn print_name(&self) {
                println!(stringify!(#new_name));
            }
            pub fn print_num(&self) {
                unsafe { #mod_name::#extern_func() }
            }
        }
    })
}

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
    let DefineArgs { args } = parse_macro_input!(input);
    let span = proc_macro::Span::call_site();
    TokenStream::from_iter(args.into_iter().map(|arg| {
        let extern_func = syn::Ident::new(&format!("call_print_num_for_{}", arg), span.into());
        TokenStream::from(quote! {
            #[no_mangle]
            pub extern "C" fn #extern_func() {
                println!("{}", #arg);
            }
        })
    }))
}
