mod attributes;

use proc_macro::{self, TokenStream};
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};
use attributes::{OicColumn};

// HelloMacro 定义
#[proc_macro_derive(HelloMacro, attributes(oic_column))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    // ident 当前枚举名称
    let DeriveInput { ident, .. } = input;

    let mut comment_arms = Vec::new();

    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = input.data {
        for variant in variants {
            // 当前枚举项名称如 Alex, Box
            let ident_item = &variant.ident;
            // 根据属性值转为 OicColumn 定义的结构化数据
            if let Ok(column) = OicColumn::from_attributes(&variant.attrs) {
                // 获取属性中的comment信息
                let msg: &syn::Lit = &column.comment.unwrap();

                // 生成 match 匹配项 Robot::Alex => "msg"
                comment_arms.push(quote! ( #ident::#ident_item => #msg ));
            } else {
                comment_arms.push(quote! ( #ident::#ident_item => "" ));
            }
        }
    }

    if comment_arms.is_empty() {
        comment_arms.push(quote! ( _ => "" ));
    }

    // 实现 comment 方法
    let output = quote! {
        impl #ident {
            fn comment(&self) -> &'static str {
                match self {
                    #(#comment_arms),*
                }
            }
        }
    };
    output.into()
}
