// essentially the usefull chunk of <https://github.com/d-e-s-o/uid>

// #![warn(
//     missing_copy_implementations,
//     missing_debug_implementations,
//     missing_docs,
//     rust_2018_compatibility,
//     trivial_casts,
//     trivial_numeric_casts,
//     unsafe_op_in_unsafe_fn,
//     unstable_features,
//     unused_import_braces,
//     unused_qualifications,
//     unused_results
// )]
#![allow(unused_variables, dead_code)]

// A crate providing in-memory IDs. Among others, the IDs are
// guaranteed to be unique, even when created on different threads.

use core::{
    fmt::{Debug, Display, Formatter, Result},
    num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering},
};

macro_rules! IdImpl {
  ( $(#[$docs:meta])* struct $name: ident, $int_type:ty, $non_zero_type:ty, $atomic_type: ty ) => {
    $(#[$docs])*
    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
    #[repr(transparent)]
    pub struct $name {
        id: $non_zero_type,

    }

    impl $name {
        // Create a new ID using the given value.
        //
        // This constructor panics if an overflow of the underlying
        // counter occurred.
        //
        // - `id` must not be zero
        // - `id` should be unique with respect to other IDs created for this
        #[inline]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe fn new_unchecked(id: $int_type) -> Self {
            Self {
                id: <$non_zero_type>::new_unchecked(id) ,
            }
        }

        // Create a new unique ID.
        //
        // This constructor panics if an overflow of the underlying
        // counter occurred.
        #[inline]
        pub fn new() -> Self {
            static NEXT_ID: $atomic_type = <$atomic_type>::new(1);

            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            assert_ne!(
                id, 0,
                "overflow detected; please use a larger integer to or reconsider your use case"
            );

            // The provided ID cannot be 0 (unless we overflow, in which
            // case we have other problems). We ensure uniqueness
            // because we increment IDs and this is the only constructor
            // for ID objects.
            unsafe { Self::new_unchecked(id) }
        }

        // Retrieve the underlying integer value.
        #[inline]
        pub fn get(self) -> $int_type {
                self.id.get()
            }
        }

        impl Default for $name {
            // Create a new unique ID.
            #[inline]
            fn default() -> Self {
                Self::new()
            }
        }

        impl Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                f.debug_tuple(stringify!($name)).field(&self.id).finish()
            }
        }

        impl Display for $name {
            // Format the ID with the given formatter.
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{}", self.id)
            }
        }
    }
}

IdImpl! {
    struct Id, usize, NonZeroUsize, AtomicUsize
}
IdImpl! {
    struct IdU8, u8, NonZeroU8, AtomicU8
}
IdImpl! {
    struct IdU16, u16, NonZeroU16, AtomicU16
}
IdImpl! {
    struct IdU32, u32, NonZeroU32, AtomicU32
}
IdImpl! {
    struct IdU64, u64, NonZeroU64, AtomicU64
}
