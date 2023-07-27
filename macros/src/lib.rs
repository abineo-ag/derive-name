use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Name)]
pub fn derive(input: TokenStream) -> TokenStream {
	let ast: syn::DeriveInput = syn::parse_macro_input!(input);
    let name = &ast.ident;
	let gen = quote! {
		impl derive_name::Name for #name {
			fn name() -> &'static str {
				stringify!(#name)
			}
		}
	};
	gen.into()
}
