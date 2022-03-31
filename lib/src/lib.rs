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
