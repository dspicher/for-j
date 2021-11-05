#[proc_macro_derive(EnumVariantCount)]
pub fn derive_enum_variant_count(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();
    let len = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("EnumVariantCount only works on Enums"),
    };
    let expanded = quote::quote! {
    const LENGTH: usize = #len;
        };
    expanded.into()
}
