use proc_macro::TokenStream;

mod hashable;
mod parser;

#[proc_macro_derive(Hashable)]
pub fn derive_hash(item: TokenStream) -> TokenStream {
    println!("{:?}", item);
    let struct_desc = crate::parser::read_struct(item);

    hashable::derive_hashable(struct_desc)
}
