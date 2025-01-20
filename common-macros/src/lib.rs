use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(EnumConversions)]
pub fn derive_enum_conversions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("EnumConversions can only be derived for enums"),
    };

    let mut current_value = 1;
    let mut variant_values = Vec::new();

    // 计算每个变体的值
    for variant in variants {
        if let Some((_, expr)) = &variant.discriminant {
            // 如果显式指定了值
            let value: i32 = syn::parse2::<syn::Expr>(expr.to_token_stream())
                .and_then(|expr| eval_const_expr(&expr))
                .expect("Failed to evaluate constant expression");
            current_value = value + 1;
            variant_values.push(value);
        } else {
            // 如果没有显式指定值，使用当前值并递增
            variant_values.push(current_value);
            current_value += 1;
        }
    }

    let variant_names: Vec<_> = variants.iter().map(|v| &v.ident).collect();
    let variant_strings: Vec<_> = variant_names.iter().map(|n| n.to_string()).collect();

    let expanded = quote! {
        impl #name {
            pub fn variant_names() -> &'static [&'static str] {
                &[#(#variant_strings),*]
            }

            pub fn to_string(&self) -> String {
                match self {
                    #(Self::#variant_names => #variant_strings.to_string()),*
                }
            }

            pub fn to_number(&self) -> i32 {
                match self {
                    #(Self::#variant_names => #variant_values),*
                }
            }
        }

        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_uppercase().as_str() {
                    #(s if s == #variant_strings.to_uppercase() => Ok(Self::#variant_names),)*
                    _ => Err(format!("Unknown variant: {}", s))
                }
            }
        }

        impl From<i32> for #name {
            fn from(value: i32) -> Self {
                match value {
                    #(#variant_values => Self::#variant_names,)*
                    _ => panic!("Invalid value: {}", value)
                }
            }
        }
    };

    TokenStream::from(expanded)
}

// 辅助函数：评估常量表达式
fn eval_const_expr(expr: &syn::Expr) -> syn::Result<i32> {
    match expr {
        syn::Expr::Lit(expr_lit) => {
            match &expr_lit.lit {
                syn::Lit::Int(lit_int) => lit_int.base10_parse(),
                _ => Err(syn::Error::new_spanned(expr, "Expected integer literal")),
            }
        }
        _ => Err(syn::Error::new_spanned(expr, "Expected constant expression")),
    }
}
