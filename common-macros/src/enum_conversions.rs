use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Attribute};

// 支持的表示类型
#[derive(Default)]
enum ReprType {
    #[default]
    I32,
    U8,
    U16,
    U32,
    U64,
}

impl ReprType {
    fn parse_from_attrs(attrs: &[Attribute]) -> Self {
        for attr in attrs {
            if attr.path().is_ident("repr") {
                if let Ok(repr) = attr.parse_args::<syn::Ident>() {
                    return match repr.to_string().as_str() {
                        "u8" => ReprType::U8,
                        "u16" => ReprType::U16,
                        "u32" => ReprType::U32,
                        "u64" => ReprType::U64,
                        _ => ReprType::I32,
                    };
                }
            }
        }
        ReprType::I32
    }

    fn as_type_token(&self) -> proc_macro2::TokenStream {
        match self {
            ReprType::I32 => quote!(i32),
            ReprType::U8 => quote!(u8),
            ReprType::U16 => quote!(u16),
            ReprType::U32 => quote!(u32),
            ReprType::U64 => quote!(u64),
        }
    }
}

pub(crate) fn derive_enum_conversions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // 解析repr属性
    let repr_type = ReprType::parse_from_attrs(&input.attrs);
    let number_type = repr_type.as_type_token();

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("EnumConversions can only be derived for enums"),
    };

    let mut current_value = 0;
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

            pub fn to_number(&self) -> #number_type {
                match self {
                    #(Self::#variant_names => #variant_values as #number_type),*
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

        impl From<#number_type> for #name {
            fn from(value: #number_type) -> Self {
                match value as i32 {
                    #(#variant_values => Self::#variant_names,)*
                    _ => panic!("Invalid value: {}", value)
                }
            }
        }

        impl From<#name> for #number_type {
            fn from(value: #name) -> Self {
                value.to_number()
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