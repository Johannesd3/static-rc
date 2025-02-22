//! `StaticRc` uses Rust's affine type system and const generics to track the shared ownership of a heap-allocated
//! value safely at compile-time, with no run-time overhead.
//!
//! The amount of `unsafe` used within is minimal, and `StaticRc` mostly leverages `Box` for most of the heavy-duty
//! operations.
//!
//! #   Example of usage.
//!
//! ```
//! use static_rc::StaticRc;
//!
//! type Full<T> = StaticRc<T, 3, 3>;
//! type TwoThird<T> = StaticRc<T, 2, 3>;
//! type OneThird<T> = StaticRc<T, 1, 3>;
//!
//! let mut full = Full::new("Hello, world!".to_string());
//!
//! assert_eq!("Hello, world!", &*full);
//!
//! //  Mutation is allowed when having full ownership, just like for `Box`.
//! *full = "Hello, you!".to_string();
//!
//! assert_eq!("Hello, you!", &*full);
//!
//! //  Mutation is no longer allowed from now on, due to aliasing, just like for `Rc`.
//! let (two_third, one_third) = Full::split::<2, 1>(full);
//!
//! assert_eq!("Hello, you!", &*two_third);
//! assert_eq!("Hello, you!", &*one_third);
//!
//! let mut full = Full::join(one_third, two_third);
//!
//! assert_eq!("Hello, you!", &*full);
//!
//! //  Mutation is allowed again, since `full` has full ownership.
//! *full = "Hello, world!".to_string();
//!
//! assert_eq!("Hello, world!", &*full);
//!
//! //  Finally, the value is dropped when `full` is.
//! ```
//!
//! #   Options
//!
//! The crate is defined for `no_std` environment and only relies on `core` and `alloc`.
//!
//! The crate only uses stable features by default, with a MSRV of 1.51 due to the use of const generics.
//!
//! Additional, the crate offers several optional features which unlock additional capabilities by using nightly.
//! Please see `Cargo.toml` for an up-to-date list of features, and their effects.

//  Regular features
#![cfg_attr(not(test), no_std)]

//  Nightly features
#![cfg_attr(compile_time_ratio, feature(const_generics))]
#![cfg_attr(compile_time_ratio, feature(const_evaluatable_checked))]
#![cfg_attr(nightly_async_stream, feature(async_stream))]
#![cfg_attr(nightly_coerce_unsized, feature(coerce_unsized))]
#![cfg_attr(nightly_dispatch_from_dyn, feature(dispatch_from_dyn))]
#![cfg_attr(nightly_generator_trait, feature(generator_trait))]

//  Lints
#![deny(missing_docs)]

extern crate alloc;

mod static_rc;

pub use self::static_rc::StaticRc;
