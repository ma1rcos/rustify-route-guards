use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, NestedMeta};

pub fn extract_roles_from_args(args: AttributeArgs) -> Vec<String> {
    args.iter()
        .filter_map(|arg| {
            if let NestedMeta::Lit(syn::Lit::Str(lit)) = arg {
                Some(lit.value())
            } else {
                None
            }
        })
        .collect()
}

pub fn generate_guard_check(roles: &[String]) -> TokenStream {
    if roles.is_empty() {
        return quote! {};
    }
    let role_checks = roles.iter().map(|role| {
        quote! {
            crate::has_role(#role) ||
        }
    });
    quote! {
        if !(#(#role_checks)* false) {
            return Err(crate::GuardError::PermissionDenied.into());
        }
    }
}