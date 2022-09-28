use proc_macro_error::{abort_call_site, proc_macro_error};

#[proc_macro_derive(Ablate)]
#[proc_macro_error]
pub fn ablate_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // construct a rust ast from the token stream
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // build the trait implementation
    impl_ablate(&input).into()
}

fn impl_ablate(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    // name of the struct or enum that we have
    let name = &ast.ident;

    match &ast.data {
        syn::Data::Struct(data) => impl_struct(name, data),
        syn::Data::Enum(data) => impl_enum(name, data),
        syn::Data::Union(_) => abort_call_site!("Only works on structs or enums."),
    }
}

fn impl_enum(name: &proc_macro2::Ident, ast: &syn::DataEnum) -> proc_macro2::TokenStream {
    let arms: Vec<_> = ast
        .variants
        .iter()
        .enumerate()
        .map(|(i, var)| {
            let var_name = &var.ident;

            if var.fields.len() == 0 {
                quote::quote! {
                    #i => #name::#var_name,
                }
            } else {
                todo!("Don't support this case yet")
            }
        })
        .collect();

    // first order approximation
    let size = ast.variants.len();

    #[rustfmt::skip]
    quote::quote! {
	impl ::ablate::Ablate for #name {
            fn nth(n: usize) -> Self {
		match n {
		    #(#arms)*
		    _ => unreachable!()
		}
            }

            fn size() -> usize {
		#size
            }
	}
    }
}

fn impl_struct(name: &proc_macro2::Ident, ast: &syn::DataStruct) -> proc_macro2::TokenStream {
    let types: Vec<_> = ast.fields.iter().map(|field| &field.ty).collect();

    let index_id = quote::format_ident!("__indices");

    let field_assignments: Vec<_> = ast
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let field_name = &field.ident;
            let field_ty = &field.ty;
            quote::quote! {
                let #field_name = #field_ty::nth(#index_id[#i]);
            }
        })
        .collect();
    let field_names: Vec<_> = ast.fields.iter().map(|field| &field.ident).collect();

    #[rustfmt::skip]
    quote::quote! {
	impl ::ablate::Ablate for #name {
	    fn nth(n: usize) -> Self {
		let #index_id = ::ablate::digits([#(#types::size()),*], n);
		#(#field_assignments)*
		#name {
		    #(#field_names),*
		}
	    }

	    fn size() -> usize {
		#(#types::size() * )* 1usize
	    }
	}
    }
}
