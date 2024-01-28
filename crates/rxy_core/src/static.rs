use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Static<T>(pub T);

impl<T> Static<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Static<T> {
    fn from(value: T) -> Self {
        Static(value)
    }
}

impl<T> Deref for Static<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Static<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
