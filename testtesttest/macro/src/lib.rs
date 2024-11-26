#![feature(proc_macro_diagnostic)]

use std::sync::atomic::AtomicUsize;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

static IDX: AtomicUsize = AtomicUsize::new(0);

#[proc_macro]
pub fn declare(input: TokenStream) -> TokenStream {
    let name: syn::Ident = parse_macro_input!(input);
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
    TokenStream::from(quote! { let mut #new_name = 0; })
}
