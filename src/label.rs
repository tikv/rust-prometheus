use std::slice::Iter;

pub trait LabelLengthPlaceholder: 'static + Send + Sync {}

pub trait IntoLabelVec {
    type Len: LabelLengthPlaceholder;

    fn into(self) -> Vec<String>;
}

pub trait AsLabelValuesIter<Len: LabelLengthPlaceholder> {
    type Item: AsRef<str>;

    fn values_iter(&self) -> Iter<Self::Item>;
}

macro_rules! array_impl {
    ($len: expr) => {
        impl LabelLengthPlaceholder for [(); $len] {}

        impl<T: Into<String>> IntoLabelVec for [T; $len] {
            type Len = [(); $len];

            fn into(self) -> Vec<String> {
                let vec = arrayvec::ArrayVec::from(self);
                vec.into_iter().map(|v| v.into()).collect()
            }
        }

        impl<T: AsRef<str>> AsLabelValuesIter<[(); $len]> for [T; $len] {
            type Item = T;

            fn values_iter(&self) -> Iter<T> {
                self.iter()
            }
        }
    };
}

macro_rules! array_impl_many {
    ($($len: expr,)*) => {
        $(array_impl!($len);)*
    };
}

array_impl_many!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31,
);
