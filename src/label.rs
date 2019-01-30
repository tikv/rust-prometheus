use std::slice::Iter;

pub trait LabelLengthPlaceholder: 'static {}

impl LabelLengthPlaceholder for [(); 1] {}

impl LabelLengthPlaceholder for [(); 2] {}

pub trait IntoLabelVec {
    type Len: LabelLengthPlaceholder;

    fn into(self) -> Vec<String>;
}

impl<T: Into<String>> IntoLabelVec for [T; 1] {
    type Len = [(); 1];

    fn into(self) -> Vec<String> {
        let [v1] = self;
        vec![v1.into()]
    }
}

impl<T: Into<String>> IntoLabelVec for [T; 2] {
    type Len = [(); 2];

    fn into(self) -> Vec<String> {
        let [v1, v2] = self;
        vec![v1.into(), v2.into()]
    }
}

pub trait AsLabelValuesIter<Len: LabelLengthPlaceholder> {
    type Item: AsRef<str>;

    fn values_iter(&self) -> Iter<Self::Item>;
}

impl<T: AsRef<str>> AsLabelValuesIter<[(); 1]> for [T; 1] {
    type Item = T;

    fn values_iter(&self) -> Iter<T> {
        self.iter()
    }
}

impl<T: AsRef<str>> AsLabelValuesIter<[(); 2]> for [T; 2] {
    type Item = T;

    fn values_iter(&self) -> Iter<T> {
        self.iter()
    }
}
