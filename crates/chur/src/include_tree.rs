use std::{
    env,
    path::{Path, PathBuf},
};

use proc_macro2::{Span, TokenStream};
use syn::{Ident, LitStr};

use quote::{quote, quote_spanned};

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

    let mod_name = syn::parse_str::<Ident>(mod_item.0.as_str());

    let mod_name_unwrapped = match mod_name {
        Ok(mod_name) => mod_name,
        Err(e) => {
            panic!(
                "invalid mod ident \"{}\" - {}",
                mod_item.0.as_str(),
                e
            );
        }
    };

    quote! {
        pub mod #mod_name_unwrapped {
            #(#mod_children)*
        }
    }
}

fn file_to_token_stream(file_item: File, root_dir: &Path) -> TokenStream {
    let path = root_dir
        .join(format!("{}.rs", file_item.0))
        .strip_prefix(crate::defined_constants::GENERATED_SOURCES_DIR.as_path())
        .unwrap()
        .display()
        .to_string();

    let path_tok = syn::Lit::Str(LitStr::new(&path, Span::call_site()));

    quote!(include!(concat!(env!("OUT_DIR"), "/", #path_tok));)
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

pub(super) fn include_tree() -> TokenStream {
    let out_dir = env::var("OUT_DIR").unwrap();

    // path to file descriptor if present
    let mut fd: Option<PathBuf> = None;

    let out_dir_path = Path::new(out_dir.as_str());
    let dir_contents = out_dir_path.read_dir().unwrap().filter_map(|entry| {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                if let Some(file_name) = entry.path().file_name() {
                    let fn_string = file_name.to_string_lossy();
                    if fn_string == "__fd.bin" {
                        fd = Some(entry.path())
                    } else if let Some(ext) = entry.path().extension() {
                        if ext == "rs" && fn_string != "__pb.rs" {
                            return Some(
                                entry
                                    .file_name()
                                    .to_string_lossy()
                                    .to_string()
                                    .trim_end_matches(".rs")
                                    .to_string(),
                            );
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
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

    let fd_token_stream = if let Some(fd_path) = fd {
        let file_path = fd_path
            .strip_prefix(crate::defined_constants::GENERATED_SOURCES_DIR.as_path())
            .unwrap()
            .display()
            .to_string();
        Some(
            quote_spanned!(call_site=> pub const FILE_DESCRIPTOR_BYTES: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/", #file_path));),
        )
    } else {
        None
    };

    quote! {
        #(#tree_items)*

        #fd_token_stream
    }
}
