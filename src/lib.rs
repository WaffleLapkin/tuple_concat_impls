extern crate proc_macro;
use proc_macro::{TokenStream};

use syn::{parse_macro_input};
use quote::quote;
use syn::parse::{Parse, ParseStream};

struct ConcatImplInfo {
    generics1: Vec<syn::Ident>,
    generics2: Vec<syn::Ident>,
}

impl Parse for ConcatImplInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut generics1 = Vec::new();
        loop {
            generics1.push(input.parse::<syn::Ident>()?);

            if let Err(_) = input.parse::<syn::Token![,]>() {
                input.parse::<syn::Token![;]>()?;
                break;
            }
        }

        let mut generics2 = Vec::new();
        loop {
            generics2.push(input.parse::<syn::Ident>()?);

            if let Err(_) = input.parse::<syn::Token![,]>() {
                input.parse::<syn::Token![;]>()?;
                break;
            }
        }

        Ok(ConcatImplInfo {
            generics1,
            generics2,
        })
    }
}

/// Generate impls of `fntools::tuple::concat::TupleConcat`
#[proc_macro]
pub fn concat_impls(input: TokenStream) -> TokenStream {
    let ConcatImplInfo { generics1, generics2 } = parse_macro_input!(input as ConcatImplInfo);

    let mut streams = Vec::new();

    for i in 0..generics1.len() {
        let first = &generics1[..=i];
        for j in 0..generics2.len() {
            let second = &generics2[..=j];
            if first.len() + second.len() > 12 { break; }

            streams.push(quote! {
                impl<#(#first,)* #(#second,)*> TupleConcat<(#(#second,)*)> for (#(#first,)*) {
                    type Res = (#(#first,)* #(#second,)*);

                    #[inline]
                    #[allow(non_snake_case)]
                    fn concat(self, other: (#(#second,)*)) -> Self::Res {
                        let (#(#first,)*) = self;
                        let (#(#second,)*) = other;
                        (#(#first,)* #(#second,)*)
                    }
                }
            });
        }
    }

    let tokens = quote! {
        #(#streams)*
    };

    tokens.into()
}
