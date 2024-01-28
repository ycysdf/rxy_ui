use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[pin_project(project = EitherProj)]
pub enum Either<L, R> {
    Left(#[pin] L),
    Right(#[pin] R),
}

impl<L, R> Iterator for Either<L, R>
where
    L: Iterator,
    R: Iterator<Item = L::Item>,
{
    type Item = L::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(l) => l.next(),
            Either::Right(r) => r.next(),
        }
    }
}

impl<L, R, LO, RO> Future for Either<L, R>
where
    L: Future<Output = LO>,
    R: Future<Output = RO>,
{
    type Output = Either<LO, RO>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            EitherProj::Left(l) => l.poll(cx).map(Either::Left),
            EitherProj::Right(r) => r.poll(cx).map(Either::Right),
        }
    }
}

pub trait EitherExt: Sized {
    fn either_left<T>(self) -> Either<Self, T> {
        Either::Left(self)
    }
    fn either_right<T>(self) -> Either<T, Self> {
        Either::Right(self)
    }
}

impl<T: Sized> EitherExt for T {}

impl<L, R> Either<L, R> {
    pub fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(r),
        }
    }
    pub fn map_left<U>(self, f: impl FnOnce(L) -> U) -> Either<U, R> {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }
    pub fn unwrap_left(self) -> L {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => panic!("unwrap_left on Either::Right"),
        }
    }
    pub fn unwrap_right(self) -> R {
        match self {
            Either::Left(_) => panic!("unwrap_right on Either::Left"),
            Either::Right(r) => r,
        }
    }
    pub fn map_right<U>(self, f: impl FnOnce(R) -> U) -> Either<L, U> {
        match self {
            Either::Right(r) => Either::Right(f(r)),
            Either::Left(r) => Either::Left(r),
        }
    }
}

impl<T> Either<T, T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Either<U, U> {
        match self {
            Either::Left(n) => Either::Left(f(n)),
            Either::Right(n) => Either::Right(f(n)),
        }
    }
    pub fn into_inner(self) -> T {
        match self {
            Either::Left(n) => n,
            Either::Right(n) => n,
        }
    }
}

impl<L, R> From<Result<L, R>> for Either<L, R> {
    fn from(value: Result<L, R>) -> Self {
        match value {
            Ok(r) => Either::Left(r),
            Err(r) => Either::Right(r),
        }
    }
}

impl<LS, LSK: Clone, RS, RSK: Clone> Either<(LS, LSK), (RS, RSK)> {
    pub fn map_to_state_key(&self) -> Either<LSK, RSK> {
        match self {
            Either::Left((_, state_key)) => Either::Left(state_key.clone()),
            Either::Right((_, state_key)) => Either::Right(state_key.clone()),
        }
    }
}
