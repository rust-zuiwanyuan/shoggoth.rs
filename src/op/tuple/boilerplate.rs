use syntax::product::{
    ToList,
    ToTuple,
};
use op::tuple;

macro_rules! impl_is_pair_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> tuple::IsPair for ($($seq,)*) {
            type H = seq_head!($($seq),*);
            type T = seq_tail!($($seq),*);
            #[inline]
            fn pair(self) -> (seq_head!($($seq),*), seq_tail!($($seq),*)) {
                let ($($seq,)*) = self;
                (seq_head!($($seq),*), seq_tail!($($seq),*))
            }
        }
    }
}

macro_rules! impl_to_list_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> Fn<(($($seq,)*),)> for ToList {
            type Output = List![$($seq),*];
            #[inline]
            extern "rust-call" fn call(&self, (arg,): (($($seq,)*),)) -> List![$($seq),*] {
                match arg { ($($seq,)*) => list![$($seq),*] }
            }
        }
    }
}

macro_rules! impl_to_tuple_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> Fn<(($($seq,)*),)> for ToTuple {
            type Output = ($($seq,)*);
            #[inline]
            extern "rust-call" fn call(&self, (arg,): (($($seq,)*),)) -> ($($seq,)*) {
                arg
            }
        }
    }
}

impl_for_seq_upto!{  impl_is_pair_for_seq, 16 }
impl_for_seq_upto!{  impl_to_list_for_seq, 16 }
impl_for_seq_upto!{ impl_to_tuple_for_seq, 16 }
