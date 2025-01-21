// Initial version copied from sp1-derive under MIT license
extern crate alloc;
extern crate proc_macro;

use hints::create_new_struct_and_impl_hintable;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Data, DeriveInput, Expr, Fields, GenericParam, Generics, ItemStruct, Token, TypeParamBound,
};

mod hints;

/// Returns true if the generic parameter C: Config exists.
pub(crate) fn has_config_generic(generics: &Generics) -> bool {
    generics.params.iter().any(|param| match param {
        GenericParam::Type(ty) => {
            ty.ident == "C"
                && ty.bounds.iter().any(|b| match b {
                    TypeParamBound::Trait(tr) => tr.path.segments.last().unwrap().ident == "Config",
                    _ => false,
                })
        }
        _ => false,
    })
}

#[proc_macro_derive(DslVariable)]
pub fn derive_variable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident; // Struct name
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    assert!(
        has_config_generic(&input.generics),
        "DslVariable requires a generic parameter C: Config"
    );

    let gen = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let fields_init = fields.named.iter().map(|f| {
                    let fname = &f.ident;
                    let ftype = &f.ty;
                    let ftype_str = quote! { #ftype }.to_string();
                    if ftype_str.contains("Array") {
                        quote! {
                            #fname: Array::Dyn(builder.uninit(), builder.uninit()),
                        }
                    } else {
                        quote! {
                            #fname: <#ftype as Variable<C>>::uninit(builder),
                        }
                    }
                });

                let fields_assign = fields.named.iter().map(|f| {
                    let fname = &f.ident;
                    quote! {
                        self.#fname.assign(src.#fname.into(), builder);
                    }
                });

                let fields_assert_eq = fields.named.iter().map(|f| {
                    let fname = &f.ident;
                    let ftype = &f.ty;
                    quote! {
                        <#ftype as Variable<C>>::assert_eq(lhs.#fname, rhs.#fname, builder);
                    }
                });

                let field_sizes = fields.named.iter().map(|f| {
                    let ftype = &f.ty;
                    quote! {
                        <#ftype as MemVariable<C>>::size_of()
                    }
                });

                let field_loads = fields.named.iter().map(|f| {
                    let fname = &f.ident;
                    let ftype = &f.ty;
                    quote! {
                        {
                            // let address = builder.eval(ptr + Usize::Const(offset));
                            self.#fname.load(ptr, index, builder);
                            index.offset += <#ftype as MemVariable<C>>::size_of();
                        }
                    }
                });

                let field_stores = fields.named.iter().map(|f| {
                    let fname = &f.ident;
                    let ftype = &f.ty;
                    quote! {
                        {
                            // let address = builder.eval(ptr + Usize::Const(offset));
                            self.#fname.store(ptr, index, builder);
                            index.offset += <#ftype as MemVariable<C>>::size_of();
                        }
                    }
                });

                quote! {
                    impl #impl_generics Variable<C> for #name #ty_generics #where_clause {
                        type Expression = Self;

                        fn uninit(builder: &mut Builder<C>) -> Self {
                            Self {
                                #(#fields_init)*
                            }
                        }

                        fn assign(&self, src: Self::Expression, builder: &mut Builder<C>) {
                            #(#fields_assign)*
                        }

                        fn assert_eq(
                            lhs: impl Into<Self::Expression>,
                            rhs: impl Into<Self::Expression>,
                            builder: &mut Builder<C>,
                        ) {
                            let lhs = lhs.into();
                            let rhs = rhs.into();
                            #(#fields_assert_eq)*
                        }
                    }

                    impl #impl_generics MemVariable<C> for #name #ty_generics #where_clause {
                        fn size_of() -> usize {
                            let mut size = 0;
                            #(size += #field_sizes;)*
                            size
                        }

                        fn load(&self, ptr: Ptr<<C as Config>::N>,
                            index: MemIndex<<C as Config>::N>,
                            builder: &mut Builder<C>) {
                            let mut index = index;
                            #(#field_loads)*
                        }

                        fn store(&self, ptr: Ptr<<C as Config>::N>,
                                 index: MemIndex<<C as Config>::N>,
                                builder: &mut Builder<C>) {
                            let mut index = index;
                            #(#field_stores)*
                        }
                    }
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    gen.into()
}

#[proc_macro_derive(Hintable)]
pub fn hintable_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);

    let new_struct = create_new_struct_and_impl_hintable(&ast);
    match new_struct {
        Ok(new_struct) => new_struct.into(),
        Err(err) => err.into(),
    }
}

struct IterZipArgs {
    builder: Expr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for IterZipArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let builder = input.parse()?;
        let _: Token![,] = input.parse()?;
        let args = Punctuated::parse_terminated(input)?;

        Ok(IterZipArgs { builder, args })
    }
}

#[proc_macro]
pub fn iter_zip(input: TokenStream) -> TokenStream {
    let IterZipArgs { builder, args } = parse_macro_input!(input as IterZipArgs);
    let array_elements = args.iter().map(|arg| {
        quote! {
            Box::new(#arg.clone()) as Box<dyn ArrayLike<_>>
        }
    });

    let expanded = quote! {
        #builder.zip(&[
            #(#array_elements),*
        ])
    };

    expanded.into()
}
