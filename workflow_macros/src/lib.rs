extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse_macro_input};

/// 在 `impl` 块中插入 `get_base()` 方法
#[proc_macro_attribute]
pub fn impl_executable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let self_ty = &input.self_ty;
    let mut items = input.items.clone();

    // 插入 get_base 方法
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
        impl Executable for #self_ty {
            #(#items)*
        }
    };

    TokenStream::from(expanded)
}
