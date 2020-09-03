#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "const_fn", feature(const_fn))]
#![cfg_attr(feature = "const_fn", feature(const_in_array_repeat_expressions))]

pub use crate::addr::{align_down, align_up, PhysAddr, VirtAddr};

/// Makes a function const only when `feature = "const_fn"` is enabled.
///
/// This is needed for const functions with bounds on their generic parameters,
/// such as those in `Page` and `PhysFrame` and many more.
macro_rules! const_fn {
    (
        $(#[$attr:meta])*
        pub $($fn:tt)*
    ) => {
        $(#[$attr])*
        #[cfg(feature = "const_fn")]
        pub const $($fn)*

        $(#[$attr])*
        #[cfg(not(feature = "const_fn"))]
        pub $($fn)*
    }
}

pub mod addr;
pub mod structures;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
