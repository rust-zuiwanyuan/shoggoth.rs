use hlist::*;
use ty::{
    Infer,
    TmPrefix,
    Ty,
    infer,
};
use ty::op::{
    Ar,
    Eval,
    IsArrow,
    Thunk,
};

/// Partially apply a thunk to an argument or evaluate a constant
/// (i.e., operation symbol)
pub trait
    AppEval<
        M,
        FxDTy,
        Fx,
    >
where
    <Fx as Infer>::Ty: IsArrow<Dom = FxDTy>,
    Fx: Infer,
    M: infer::mode::Mode,
    Self: TmPrefix<
        <<Fx as Infer>::Ty as IsArrow>::Dom
    >,
    Self: HList,
{
    // FIXME: should probably put a bound on Out
    type Out;
}

impl<
    CxDTy,
    CxCTy,
    Cx,
    Args,
>
    AppEval<
        infer::mode::Constant,
        CxDTy,
        Cx,
    >
for
    Args
where
    Args: AppEval<
        infer::mode::Thunk,
        CxDTy,
        Thunk<Cx, HN>
    >,
    Args: TmPrefix<CxDTy>,
    Args: HList,
    Cx: Infer<Ty = Ar<CxDTy, CxCTy>>,
    CxCTy: Ty,
    CxDTy: HList,
    CxDTy: Ty,
{
    type Out =
        <Args as
            AppEval<
                infer::mode::Thunk,
                CxDTy,
                Thunk<Cx, HN>,
            >
        >::Out;
}

impl<
    Fx,
    FxDTy,
    TxCTy,
    Xs,
>
    AppEval<
        infer::mode::Thunk,
        HN,
        Thunk<Fx, Xs>,
    >
for
    HN
where
    Fx: Infer<Ty = Ar<FxDTy, TxCTy>>,
    FxDTy: HList,
    FxDTy: Ty,
    Thunk<Fx, Xs>: Infer<Ty = Ar<HN, TxCTy>>,
    TxCTy: Ty,
    Xs: Eval<Fx>,
    Xs: HList,
    Xs: TmPrefix<FxDTy, Out = HN>,
{
    type Out = <Xs as Eval<Fx>>::Out;
}

impl<
    Tx,
    TxCTy,
    TxDHTy,
    TxDTTy,
>
    AppEval<
        infer::mode::Thunk,
        HC<TxDHTy, TxDTTy>,
        Tx,
    >
for
    HN
where
    Tx: Infer<Ty = Ar<HC<TxDHTy, TxDTTy>, TxCTy>>,
    TxCTy: Ty,
    TxDHTy: Ty,
    TxDTTy: HList,
    TxDTTy: Ty,
{
    type Out = Tx;
}

impl<
    ArgsHTm,
    ArgsTTm,
    Fx,
    FxDHTy,
    FxDTTy,
    TxCTy,
    TxDHTy,
    TxDTTy,
    Xs,
>
    AppEval<
        infer::mode::Thunk,
        HC<TxDHTy, TxDTTy>,
        Thunk<Fx, Xs>
    >
for
    HC<ArgsHTm, ArgsTTm>
where
    ArgsTTm: AppEval<
        infer::mode::Thunk,
        TxDTTy,
        Thunk<Fx, HS<Xs, ArgsHTm>>
    >,
    ArgsTTm: HList,
    Fx: Infer<Ty = Ar<HC<FxDHTy, FxDTTy>, TxCTy>>,
    FxDHTy: Ty,
    FxDTTy: HList,
    FxDTTy: Ty,
    HC<ArgsHTm, ArgsTTm>: TmPrefix<HC<TxDHTy, TxDTTy>>,
    TxCTy: Ty,
    TxDHTy: Ty,
    TxDTTy: HList,
    TxDTTy: Ty,
    Xs: HList,
    Xs: TmPrefix<HC<FxDHTy, FxDTTy>, Out = HC<TxDHTy, TxDTTy>>,
    Xs: Snoc<ArgsHTm>,
{
    type Out =
        <ArgsTTm as AppEval<
            infer::mode::Thunk,
            TxDTTy,
            Thunk<Fx, HS<Xs, ArgsHTm>>
        >>::Out;
}