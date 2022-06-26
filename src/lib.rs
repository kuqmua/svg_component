use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(SvgComponent)]
pub fn derive_svg_component(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_svg_component syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = quote! {
        impl SvgComponent for #ident {
            fn get_html(&self) -> Html {
                match self {

                }
            };
            fn get_class(&self) -> AttrValue {
                match *self {

                }
            };
        }
    };
    gen.into()
}
