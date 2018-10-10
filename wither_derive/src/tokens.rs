use quote::ToTokens;
use proc_macro2::TokenStream;

use bson::Document;
use mongodb::coll::options::{IndexModel, IndexOptions};

pub struct Indexes(pub Vec<IndexModel>);

impl ToTokens for Indexes {
    /// Implement `ToTokens` for the `Indexes` type.
    ///
    /// This type is a simple wrapper around a `Vec<IndexModel>` and when it is converted to a
    /// token stream, it will simply be returned as the underlying `Vec` type.
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // If we have no indexes, then code an empty vec.
        if self.0.len() == 0 {
            tokens.extend(quote!{vec![]});
            return;
        }

        // Else, build up a vector of token streams which we will interpolate later.
        let index_tokens = self.0.iter().map(|index| {
            // Desctructure variables needed for interpolation. Use struct destructuring syntax
            // to ensure we are not missing any fields.
            let IndexOptions{
                background, expire_after_seconds, name, sparse, storage_engine, unique, version, default_language,
                language_override, text_version, weights, sphere_version, bits, max, min, bucket_size,
            } = index.options.clone();
            let background = option_to_tokens(background);
            let expire_after_seconds = option_to_tokens(expire_after_seconds);
            let name = option_to_tokens_for_string(name);
            let sparse = option_to_tokens(sparse);
            let storage_engine = option_to_tokens_for_string(storage_engine);
            let unique = option_to_tokens(unique);
            let version = option_to_tokens(version);
            let default_language = option_to_tokens_for_string(default_language);
            let language_override = option_to_tokens_for_string(language_override);
            let text_version = option_to_tokens(text_version);
            // let weights = None; // option_to_tokens(weights);
            let sphere_version = option_to_tokens(sphere_version);
            let bits = option_to_tokens(bits);
            let max = option_to_tokens(max);
            let min = option_to_tokens(min);
            let bucket_size = option_to_tokens(bucket_size);

            // Need to take special care with the index keys.
            let keys = build_index_keys(&index.keys);

            quote!(IndexModel{
                keys: #keys,
                options: IndexOptions{
                    background: #background, expire_after_seconds: #expire_after_seconds, name: #name, sparse: #sparse,
                    storage_engine: #storage_engine, unique: #unique, version: #version, default_language: #default_language,
                    language_override: #language_override, text_version: #text_version, weights: None, sphere_version: #sphere_version,
                    bits: #bits, max: #max, min: #min, bucket_size: #bucket_size,
                },
            })
        }).collect::<Vec<TokenStream>>();

        tokens.extend(quote!{
            use mongodb::coll::options::{IndexModel, IndexOptions};
            vec![
                #(#index_tokens),*
            ]
        });
    }
}

/// NOTE WELL: the token stream returned from this method evaluates to a single-line
/// `doc!` macro invocation.
fn build_index_keys(doc: &Document) -> TokenStream {
    let key_vals = doc.iter().map(|(key, val)| {
        // TODO: need to update this by using an intermediate representation of index models
        // instead of using actual index models. For now, just unwrap.
        let val = val.as_i32().unwrap();
        quote!(#key: #val)
    }).collect::<Vec<TokenStream>>();

    quote!(doc!{ #(#key_vals),* })
}

fn option_to_tokens<T: ToTokens>(target: Option<T>) -> TokenStream {
    match target {
        Some(t) => quote!(Some(#t)),
        None => quote!(None),
    }
}

fn option_to_tokens_for_string(target: Option<String>) -> TokenStream {
    match target {
        Some(t) => quote!(Some(String::from(#t))),
        None => quote!(None),
    }
}
