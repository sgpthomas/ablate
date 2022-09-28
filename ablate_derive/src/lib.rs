#[proc_macro_derive(Ablate)]
pub fn ablate_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // construct a rust ast from the token stream
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // build the trait implementation
    impl_ablate(&input).into()
}

fn impl_ablate(input: &syn::DeriveInput) -> proc_macro::TokenStream {
    todo!()
}
