use num_traits::Zero;

use crate::shortest_paths::common::AddRef;

/// Automatically implemented trait for types that can be used as a measure in the Dijkstra
/// algorithm.
///
/// This trait is implemented for all types that implement the supertraits mentioned in the trait
/// definition.
/// These traits either originate from [`core`] or [`num_traits`].
/// Special attention must be paid to the [`AddRef`] trait, which is a proxy trait which is
/// implemented for types that implement: `&Self: Add<&Self, Output = Self>`.
///
/// # Note on floating point types
///
/// This trait is not implemented for both [`f32`] and [`f64`], because they do not implement
/// [`Ord`], which is required for the binary heap used in the Dijkstra algorithm.
/// You can instead use [`ordered_float::NotNan`], which is a wrapper type that implements [`Ord`].
///
/// Be aware that [`ordered_float::OrderedFloat`] does not implement [`AddRef`] and cannot be used,
/// for a reason as to why see [this issue](https://github.com/reem/rust-ordered-float/issues/145).
///
/// You can read about the reason why [`f32`] and [`f64`] do not implement [`Ord`]
/// in the Rust documentation [here](https://doc.rust-lang.org/std/primitive.f32.html).
///
/// # Example
///
/// ```rust
/// use core::num::Wrapping;
/// use core::num::Saturating;
/// use ordered_float::{NotNan, OrderedFloat};
/// use petgraph_algorithms::shortest_paths::dijkstra::DijkstraMeasure;
/// use static_assertions::assert_impl_all;
///
/// // Some examples of types that implement DijkstraMeasure
/// assert_impl_all!(u8: DijkstraMeasure);
/// assert_impl_all!(u16: DijkstraMeasure);
/// assert_impl_all!(u32: DijkstraMeasure);
/// assert_impl_all!(u64: DijkstraMeasure);
/// assert_impl_all!(u128: DijkstraMeasure);
/// assert_impl_all!(usize: DijkstraMeasure);
///
/// assert_impl_all!(i8: DijkstraMeasure);
/// assert_impl_all!(i16: DijkstraMeasure);
/// assert_impl_all!(i32: DijkstraMeasure);
/// assert_impl_all!(i64: DijkstraMeasure);
/// assert_impl_all!(i128: DijkstraMeasure);
/// assert_impl_all!(isize: DijkstraMeasure);
///
/// // f32 and f64 are not implemented because they are not Ord
/// // use `ordered_float::NotNan` instead.
/// assert_impl_all!(NotNan<f32>: DijkstraMeasure);
/// assert_impl_all!(NotNan<f64>: DijkstraMeasure);
///
/// assert_impl_all!(Wrapping<u8>: DijkstraMeasure);
/// assert_impl_all!(Wrapping<u16>: DijkstraMeasure);
/// assert_impl_all!(Wrapping<u32>: DijkstraMeasure);
/// assert_impl_all!(Wrapping<u64>: DijkstraMeasure);
/// assert_impl_all!(Wrapping<u128>: DijkstraMeasure);
/// assert_impl_all!(Wrapping<usize>: DijkstraMeasure);
///
/// assert_impl_all!(Wrapping<i8>: DijkstraMeasure);
/// assert_impl_all!(Wrapping<i16>: DijkstraMeasure);
/// assert_impl_all!(Wrapping<i32>: DijkstraMeasure);
/// assert_impl_all!(Wrapping<i64>: DijkstraMeasure);
/// assert_impl_all!(Wrapping<i128>: DijkstraMeasure);
///
/// // TODO: these won't work :/
/// assert_impl_all!(Saturating<u8>: DijkstraMeasure);
/// assert_impl_all!(Saturating<u16>: DijkstraMeasure);
/// assert_impl_all!(Saturating<u32>: DijkstraMeasure);
/// assert_impl_all!(Saturating<u64>: DijkstraMeasure);
/// assert_impl_all!(Saturating<u128>: DijkstraMeasure);
/// assert_impl_all!(Saturating<usize>: DijkstraMeasure);
///
/// assert_impl_all!(Saturating<i8>: DijkstraMeasure);
/// assert_impl_all!(Saturating<i16>: DijkstraMeasure);
/// assert_impl_all!(Saturating<i32>: DijkstraMeasure);
/// assert_impl_all!(Saturating<i64>: DijkstraMeasure);
/// assert_impl_all!(Saturating<i128>: DijkstraMeasure);
/// assert_impl_all!(Saturating<isize>: DijkstraMeasure);
/// ```
pub trait DijkstraMeasure: Clone + PartialOrd + Ord + AddRef<Self, Output = Self> + Zero {}

impl<T> DijkstraMeasure for T where T: Clone + PartialOrd + Ord + AddRef<Self, Output = Self> + Zero {}
