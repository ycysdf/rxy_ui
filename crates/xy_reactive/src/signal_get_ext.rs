use crate::prelude::SignalGet;

pub trait BoolSignalExt: SignalGet<Value = bool> {
   fn then_some<U>(&self, then: U) -> Option<U> {
      self.get().then_some(then)
   }
   fn not_then_some<U>(&self, then: U) -> Option<U> {
      (!self.get()).then_some(then)
   }
}

impl<T> BoolSignalExt for T where T: SignalGet<Value = bool> {}
