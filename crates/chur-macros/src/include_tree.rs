use std::{env, path::Path};

use proc_macro2::{Span, TokenStream};
use syn::{Ident, LitStr};

#[derive(Debug, Default)]
struct Mod(String, Vec<TreeItem>);

#[derive(Debug)]
struct File(String);

#[derive(Debug)]
enum TreeItem {
    Mod(Mod),
    File(File),
}

fn mod_to_token_stream(mod_item: Mod, root_dir: &Path) -> TokenStream {
    let mod_children = mod_item.1.into_iter().map(|item| match item {
        TreeItem::Mod(mod_item) => mod_to_token_stream(mod_item, root_dir),
        TreeItem::File(file_item) => file_to_token_stream(file_item, root_dir),
    });

    let mod_name = syn::parse_str::<Ident>(mod_item.0.as_str()).expect("invalid mod ident");

    quote! {
        pub mod #mod_name {
            #(#mod_children)*
        }
    }
}

fn file_to_token_stream(file_item: File, root_dir: &Path) -> TokenStream {
    let path = root_dir
        .join(format!("{}.rs", file_item.0))
        .display()
        .to_string();
    let path_tok = syn::Lit::Str(LitStr::new(&path, Span::call_site()));

    quote!(include!(#path_tok);)
}

fn insert_into_mod<'a>(
    mod_item: &mut Mod,
    mut path_chunk_iter: impl Iterator<Item = &'a str>,
    original_path_name: String,
) {
    if let Some(mod_name) = path_chunk_iter.next() {
        let child_mod = mod_item.1.iter_mut().find_map(|m| {
            if let TreeItem::Mod(existant_mod) = m {
                if existant_mod.0 == mod_name {
                    Some(existant_mod)
                } else {
                    None
                }
            } else {
                None
            }
        });

        if let Some(child) = child_mod {
            insert_into_mod(child, path_chunk_iter, original_path_name);
        } else {
            let mut new_mod = Mod(mod_name.to_string(), vec![]);

            insert_into_mod(&mut new_mod, path_chunk_iter, original_path_name);
            mod_item.1.push(TreeItem::Mod(new_mod))
        }
    } else {
        mod_item.1.push(TreeItem::File(File(original_path_name)))
    }
}

pub(super) fn include_tree_inner(_input: TokenStream) -> TokenStream {
    let out_dir = env::var("OUT_DIR").unwrap();

    let out_dir_path = Path::new(out_dir.as_str());
    let dir_contents = out_dir_path.read_dir().unwrap().filter_map(|entry| {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                return Some(
                    entry
                        .file_name()
                        .to_string_lossy()
                        .to_string()
                        .trim_end_matches(".rs")
                        .to_string(),
                );
            }
        }

        None
    });

    let mut root_tree = Mod::default();

    dir_contents.for_each(|item| {
        insert_into_mod(&mut root_tree, item.split('.'), item.clone());
    });

    let tree_items = root_tree.1.into_iter().map(|item| match item {
        TreeItem::Mod(mod_item) => mod_to_token_stream(mod_item, out_dir_path),
        TreeItem::File(file_item) => file_to_token_stream(file_item, out_dir_path),
    });

    let call_site = Span::call_site();
    let tree = quote_spanned!(call_site=> #(#tree_items)*);

    tree
}
