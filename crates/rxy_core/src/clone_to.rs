pub trait CloneTo {
    type To;
    fn clone_to(&self) -> Self::To;
}

impl<T: Clone> CloneTo for T {
    type To = T;

    fn clone_to(&self) -> Self::To {
        self.clone()
    }
}
