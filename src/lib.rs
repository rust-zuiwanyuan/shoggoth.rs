#![feature(hash)]
#![feature(on_unimplemented)]

// pub use self::hlist::{
//     Cons,
//     HC,
//     HList,
//     HN,
//     IsCons,
//     Nil,
// };
// pub use self::product::{
//     ProductOps,
//     ToHList,
//     ToTuple,
// };
// pub use self::tuple::{
//     IsComposite,
//     TupleOps,
// };

/// Heterogeneous lists
pub mod hlist;
// mod product;
// mod tuple;

/// Type-level programming
pub mod ty;
