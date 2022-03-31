#![allow(clippy::needless_doctest_main)]

//! This crate provides a simple [`Hashable`](https://docs.rs/sha2-derive/latest/sha2_derive/trait.Hashable.html) trait that can be derived easily.  
//! As of now, the derive macro only works on structs.
//! 
//! # Example
//! 
//! ```rust
//! use sha2_derive::*;
//! 
//! #[derive(Hashable)]
//! struct User {
//!     username: String,
//!     age: u8,
//!     timestamps: Vec<u64>,
//! }
//! 
//! fn main() {
//!     let user = User {
//!         username: String::from("Mubelotix"),
//!         age: 42,
//!         timestamps: vec![1, 2, 3],
//!     };
//! 
//!     let hash = user.hash();
//! }
//! ```
//! 
//! # Implementing Hashable on a custom type
//! 
//! ```rust
//! struct CustomType {
//!    // fields
//! }
//! 
//! impl Hashable for CustomType {
//!     fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
//!         todo!()
//!     }
//! }
//! ```

pub use sha2_derive_proc_macro::Hashable;
pub(crate) use sha2::*;
pub(crate) use generic_array::*;

mod implementations;

pub trait Hashable {
    fn update_hasher(&self, hasher: &mut impl sha2::Digest);

    fn hash(&self) -> GenericArray<u8, typenum::U32> {
        self.hash_sha256()
    }

    fn hash_sha256(&self) -> GenericArray<u8, typenum::U32> {
        let mut hasher = sha2::Sha256::new();
        self.update_hasher(&mut hasher);
        hasher.finalize()
    }

    fn hash_sha512(&self) -> GenericArray<u8, typenum::U64> {
        let mut hasher = sha2::Sha512::new();
        self.update_hasher(&mut hasher);
        hasher.finalize()
    }

    fn hash_sha<T: Digest>(&self) -> GenericArray<u8, T::OutputSize> {
        let mut hasher = T::new();
        self.update_hasher(&mut hasher);
        hasher.finalize()
    }
}
