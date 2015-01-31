pub use self::apply::{
    AppEval,
};
pub use self::eval::{
    Eval,
};
pub use self::thunk::{
    Thunk,
};
use hlist::*;
use ty::{
    Infer,
    Ty,
};

mod apply;
mod eval;
mod thunk;

/// Type-level operations
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Arrow<D, C>
where
    C: Ty,
    D: HList,
    D: Ty,
{}

impl<
    C,
    D,
>
    Ty
for
    Arrow<D, C>
where
    C: Ty,
    D: HList,
    D: Ty,
{}

/// Predicate providing access to (co)domain of type-level arrows
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level arrow type"]
pub trait
    IsArrow
where
    Self: Ty,
{
    type Dom: Ty + HList;
    type Cod: Ty;
}

impl<
    C,
    D,
>
    IsArrow
for
    Arrow<D, C>
where
    C: Ty,
    D: HList,
    D: Ty,
{
    type Dom = D;
    type Cod = C;
}

/// Alias for arrow types with an n-ary domain
pub type Ar<D, C> = Arrow<D, C>;

/// Alias for arrow types with a nullary domain
pub type Ar0<C> = Ar<HN, C>;

/// Alias for arrow types with a unary domain
pub type Ar1<D, C> = Ar<HC<D, HN>, C>;

/// Alias for partially applying terms of arrow types to many
/// arguments
pub type Ap<Fx, Xs> =
    <Xs as
        AppEval<
             <Fx as Infer>::Mode,
            <<Fx as Infer>::Ty as IsArrow>::Dom,
            Fx,
        >
    >::Out;

/// Alias for partially applying terms of arrow types to a single
/// argument
pub type Ap1<Fx, X> = Ap<Fx, HC<X, HN>>;