use hlist::*;
use hlist::{
    self,
};
use ty::{
    _0,
    Ap,
    Ap1,
    Ar,
    Ar1,
    Cmp1,
    Eval,
    Eval1,
    Infer,
    Lens,
    List,
    MkStore,
    Store,
    Tm,
    Ty,
    infer,
    nat,
};

/// Type-level zippers for lists
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Zipper<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// ---------------
/// Zipper<A> :: Ty
/// ```
impl<
       A: Ty,
>
    Ty
for
    Zipper<A>
{}



/// Zipper constructor
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub struct
    MkZipper<Xs, Ys>(Xs, Ys)
where
      Xs: HList,
      Ys: HList;

/// ```ignore
/// A :: Ty
/// xs : List<A>
/// ys : List<A>
/// -----------------------------
/// mk_zipper(xs, ys) : Zipper<A>
/// ```
impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
      Ys: Tm<List<A>> + HList,
>
    Tm<Zipper<A>>
for
    MkZipper<Xs, Ys>
{}



/// Type-level list to zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Zip<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// xs : List<A>
/// -------------------
/// zip(xs) : Zipper<A>
/// ```
impl<
       A: Ty,
>
    Infer
for
    Zip<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<List<A>, Zipper<A>>;
}

/// `zip(xs) ==> mk_zipper(nil, xs)`
impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
>
    Eval<Zip<A>>
for
    HC<Xs, HN>
{
    type Out = MkZipper<HN, Xs>;
}



/// Type-level zipper to list
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Unzip<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper<A>
/// -------------------
/// unzip(zs) : List<A>
/// ```
impl<
       A: Ty,
>
    Infer
for
    Unzip<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, List<A>>;
}

/// `unzip(mk_zipper(xs, ys)) ==> append(reverse(xs), ys)`
impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
      Ys: Tm<List<A>> + HList,
     Rec: Tm<List<A>>,
>
    Eval<Unzip<A>>
for
    HC<MkZipper<Xs, Ys>, HN>
where
      Xs: hlist::ReversePrepend<Ys, Out = Rec>,
{
    type Out = Rec;
}



/// Type-level move right for zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Right<A>
where
       A: Ty,
{}

impl<
       A: Ty,
>
    Infer
for
    Right<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<nat::Nat,
                 HC<Zipper<A>,
                 HN>>,
                 Zipper<A>>;
}

impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
      Ys: Tm<List<A>> + HList,
>
    Eval<Right<A>>
for
    HC<_0,
    HC<MkZipper<Xs, Ys>,
    HN>>
{
    type Out = MkZipper<Xs, Ys>;
}

impl<
       A: Ty,
       P: Tm<nat::pos::Pos>,
    Rec0: Tm<nat::Nat>,
    Rec1: Tm<Zipper<A>>,
      Xs: Tm<List<A>> + HList,
       Y: Tm<A>,
      Ys: Tm<List<A>> + HList,
>
    Eval<Right<A>>
for
    HC<P,
    HC<MkZipper<Xs, Cons<Y, Ys>>,
    HN>>
where
       P: Eval1<nat::Pred, Out = Rec0>,
    HC<Rec0,
    HC<MkZipper<Cons<Y, Xs>, Ys>,
    HN>>
        : Eval<Right<A>, Out = Rec1>,
{
    type Out = Rec1;
}



/// Type-level get from focus of zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Get<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper<A>
/// -----------
/// get(zs) : A
/// ```
impl<
       A: Ty,
>
    Infer
for
    Get<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, A>;
}

/// `get(mk_zipper(xs, cons(y, ys))) ==> y`
impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
       Y: Tm<A>,
      Ys: Tm<List<A>> + HList,
>
    Eval<Get<A>>
for
    HC<MkZipper<Xs, HC<Y, Ys>>, HN>
{
    type Out = Y;
}



/// Type-level put new element, replacing focus of zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Put<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper<A>
/// e : A
/// ----------------------
/// put(zs, e) : Zipper<A>
/// ```
impl<
       A: Ty,
>
    Infer
for
    Put<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<Zipper<A>, HC<A, HN>>, Zipper<A>>;
}

/// `put(mk_zipper(xs, cons(y, ys)), e) ==> mk_zipper(xs, cons(e, ys))`
impl<
       A: Ty,
       E: Tm<A>,
      Xs: Tm<List<A>> + HList,
       Y: Tm<A>,
      Ys: Tm<List<A>> + HList,
>
    Eval<Put<A>>
for
    HC<MkZipper<Xs, HC<Y, Ys>>, HC<E, HN>>
{
    type Out = MkZipper<Xs, HC<E, Ys>>;
}



#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    ToStore<A>
where
       A: Ty,
{}

impl<
       A: Ty,
>
    Infer
for
    ToStore<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, Store<A, List<A>>>;
}

impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
       Y: Tm<A>,
      Ys: Tm<List<A>> + HList,
>
    Eval<ToStore<A>>
for
    HC<MkZipper<Xs, Cons<Y, Ys>>,
    HN>
{
    type Out = MkStore<
        Ap<Get<A>, Self>,
        Cmp1<Ap<Put<A>, Self>, Unzip<A>>,
    >;
}



#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Nth<A>
{}

impl<
       A: Ty,
>
    Infer
for
    Nth<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<nat::Nat, Lens<List<A>, A>>;
}

impl<
       A: Ty,
       N: Tm<nat::Nat>,
>
    Eval<Nth<A>>
for
    HC<N,
    HN>
{
    type Out = Cmp1<Zip<A>, Cmp1<Ap1<Right<A>, N>, ToStore<A>>>;
}



#[cfg(test)]
mod test {
    use hlist::{
        HC,
        HN,
    };
    use ty::*;

    #[test]
    fn put() {
        let x0: Witness<
                Ap<
                    zipper::Put<Star>,
                    HC<
                        MkZipper<
                            HC<Lift<bool>, HN>,
                            HC<Lift<u8>  , HN>
                        >,
                        HC<Lift<u16>, HN>
                    >
                >
            > = Witness;
        let x1: Witness<
                MkZipper<
                    HC<Lift<bool>, HN>,
                    HC<Lift<u16> , HN>
                >
            > = Witness;
        x0 == x1;
        let x2: Witness<
                Ap1<
                    Unzip<Star>,
                    MkZipper<
                        HC<Lift<bool>, HN>,
                        HC<Lift<u16> , HN>
                    >
                >
            > = Witness;
        let x3: Witness<
                HC<Lift<bool>, HC<Lift<u16>, HN>>
            > = Witness;
        x2 == x3;
    }

    #[test]
    fn nth() {
        let x0: Witness<
                Ap1<
                    View<
                        lens::Cmp<
                            Ap1<zipper::Nth<List<Bool>>, _1b>,
                            Ap1<zipper::Nth     <Bool> , _2b>
                        >
                    >,
                    HC<HN,
                    HC<HC<FF, HC<TT, HC<FF, HC<FF, HN>>>>,
                    HN>>
                >
            > = Witness;
        let x1: Witness<FF> = Witness;
        x0 == x1;
    }
}
