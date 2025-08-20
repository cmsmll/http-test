#![allow(non_snake_case)]
extern crate proc_macro;

// 在您的宏包（如 my_macros）中实现此代码
use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, LitStr, PatType, parse_macro_input};

#[proc_macro_attribute]
pub fn GET(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析属性参数（字符串字面量）
    let url = parse_macro_input!(args as LitStr);

    // 解析输入函数
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let test_name = format!("{}_http_test", fn_name);
    let test_ident = syn::Ident::new(&test_name, fn_name.span());
    let mut args = Vec::new();
    for arg in &input_fn.sig.inputs {
        match arg {
            FnArg::Typed(PatType { ty, .. }) => args.push(&**ty),
            FnArg::Receiver(_) => panic!("方法不支持作为提取器函数"),
        }
    }

    // 生成测试函数和原始函数
    let expanded = quote! {
        #input_fn

        #[cfg(test)]
        #[tokio::test]
        async fn #test_ident() {
            use http_test_core::{FromResp, Resp, Result};
            async fn inner(resp:reqwest::Response) -> Result<()> {
                let mut resp = Resp(Some(resp));
                #fn_name(#(<#args as FromResp>::from_resp(&mut resp).await?,)*);
                Ok(())
            }

            match reqwest::get(#url).await {
                Ok(resp) => if let Err(err) = inner(resp).await {
                    eprintln!("{err}");
                },
                Err(err) => eprintln!("{err}")
            }
        }
    };

    TokenStream::from(expanded)
}
