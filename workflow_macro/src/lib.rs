extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse_macro_input};

#[proc_macro_attribute]
pub fn impl_executable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let self_ty = &input.self_ty;
    let mut items = input.items.clone();

    items.push(syn::parse_quote! {
        fn get_base(&self) -> &NodeBase {
            &self.base
        }
    });

    items.push(syn::parse_quote! {
        fn clone_box(&self) -> Box<dyn Executable> {
            Box::new(self.clone())
        }
    });

    // 构建新的 impl 块
    let expanded = quote! {
        #[async_trait::async_trait]
        impl Executable for #self_ty {
            #(#items)*
        }
    };

    TokenStream::from(expanded)
}
