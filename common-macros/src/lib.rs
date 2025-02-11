mod enum_conversions;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumConversions)]
pub fn derive_enum_conversions(input: TokenStream) -> TokenStream {
    enum_conversions::derive_enum_conversions(input)
}
