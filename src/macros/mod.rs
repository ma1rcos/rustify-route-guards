mod util;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, AttributeArgs, NestedMeta};

use crate::guard::error::GuardError;
use crate::guard::role::has_role;

#[proc_macro_attribute]
pub fn guards(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut input = parse_macro_input!(item as ItemFn);
    
    let roles = util::extract_roles_from_args(args);
    let guard_check = util::generate_guard_check(&roles);
    
    let original_block = &input.block;
    input.block = syn::parse2(quote! {
        {
            #guard_check
            #original_block
        }
    }).unwrap();
    
    TokenStream::from(quote! { #input })
}