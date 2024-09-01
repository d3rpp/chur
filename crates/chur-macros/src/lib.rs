use proc_macro::TokenStream;

mod include_tree;

#[macro_use]
extern crate quote;

#[proc_macro]
pub fn include_tree(input: TokenStream) -> TokenStream {
    include_tree::include_tree_inner(input.into()).into()
}
