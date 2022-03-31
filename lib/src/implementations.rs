#![allow(non_snake_case)]

use crate::*;
use std::collections::*;

macro_rules! int_auto_impl {
    ( $($t:ty),* ) => {
        $(
            impl Hashable for $t {
                fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
                    hasher.update(&self.to_le_bytes());
                }
            }
        )*
    }
}

int_auto_impl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: Hashable),+> Hashable for ($($name,)+)
        {
            fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
                let ($($name,)+) = self;
                $($name.update_hasher(hasher);)+
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }

impl<T: Hashable> Hashable for [T] {
    fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
        self.len().update_hasher(hasher);
        for item in self.iter() {
            item.update_hasher(hasher);
        }
    }
}

impl Hashable for str {
    fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
        self.len().update_hasher(hasher);
        hasher.update(self.as_bytes());
    }
}

impl<T: Hashable> Hashable for dyn AsRef<T> {
    fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
        self.as_ref().update_hasher(hasher);
    }
}

impl<K: Hashable, V: Hashable> Hashable for BTreeMap<K, V> {
    fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
        self.len().update_hasher(hasher);
        for (key, value) in self.iter() {
            key.update_hasher(hasher);
            value.update_hasher(hasher);
        }
    }
}

impl<K: Hashable, V: Hashable> Hashable for HashMap<K, V> {
    fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
        self.len().update_hasher(hasher);
        for (key, value) in self.iter() {
            key.update_hasher(hasher);
            value.update_hasher(hasher);
        }
    }
}

impl<T: Hashable> Hashable for Option<T> {
    fn update_hasher(&self, hasher: &mut impl sha2::Digest) {
        match self {
            Some(item) => {
                1.update_hasher(hasher);
                item.update_hasher(hasher);
            }
            None => {
                0.update_hasher(hasher);
            }
        }
    }
}
