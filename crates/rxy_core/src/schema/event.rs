use core::any::{Any, TypeId};
use core::marker::PhantomData;
use std::sync::{Arc, Mutex};

pub use async_channel::Sender;
use async_channel::{unbounded, Receiver};

use crate::{
   BoxedPropValue, ConstIndex, InnerSchemaCtx, IntoSchemaPropValue, IntoSchemaPropValueWrapper,
   MaybeSend, PropState, Renderer, SchemaParam,
};

pub struct EventHandler<Args>
where
   Args: MaybeSend + 'static,
{
   f: BoxedPropValue,
   call_f: fn(&mut BoxedPropValue, Args),
}

impl<Args> EventHandler<Args>
where
   Args: MaybeSend + 'static,
{
   pub fn new<F>(f: F) -> Self
   where
      F: FnMut(Args) + MaybeSend + 'static,
   {
      Self {
         f: Box::new(f),
         call_f: |f, args| {
            let f: &mut F = f.downcast_mut::<F>().unwrap();
            f(args);
         },
      }
   }

   pub fn call(&mut self, args: Args) {
      let f = self.call_f;
      f(&mut self.f, args);
   }
}

pub struct EventHandlerState<R, Args>
where
   R: Renderer,
   Args: MaybeSend + 'static,
{
   _task: R::Task<()>,
   event_handler: Arc<Mutex<Option<BoxedPropValue>>>,
   _marker: PhantomData<Args>,
}

impl<R, Args> EventHandlerState<R, Args>
where
   R: Renderer,
   Args: MaybeSend + 'static,
{
   pub fn new(receiver: Receiver<Args>, event_handler: Option<BoxedPropValue>) -> Self {
      let event_handler = Arc::new(Mutex::new(event_handler));
      Self {
         event_handler: event_handler.clone(),
         _task: R::spawn_task(async move {
            while let Ok(args) = receiver.recv().await {
               let mut event_handler = event_handler.lock().unwrap();
               if let Some(event_handler) = event_handler.as_mut() {
                  let event_handler = event_handler.downcast_mut::<EventHandler<Args>>().unwrap();
                  event_handler.call(args)
               }
            }
         }),
         _marker: Default::default(),
      }
   }
}

impl<R, Args> PropState<R> for EventHandlerState<R, Args>
where
   R: Renderer,
   Args: MaybeSend + 'static,
{
   fn apply(&mut self, new_value: BoxedPropValue, _world: &mut R::NodeTree) {
      *self.event_handler.lock().unwrap() = Some(new_value);
   }

   fn as_any_mut(&mut self) -> &mut (dyn Any + MaybeSend) {
      self
   }
}

impl<R, Args> SchemaParam<R> for Sender<Args>
where
   R: Renderer,
   Args: MaybeSend + 'static,
{
   fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
      let type_id = TypeId::of::<ConstIndex<I>>();

      let event_handler = ctx.init_values.remove(&type_id);
      let (sender, receiver) = unbounded();
      ctx.prop_state()
         .entry(type_id)
         .or_insert_with(|| Box::new(EventHandlerState::new(receiver, event_handler)));
      sender
   }
}

impl<Args, F> IntoSchemaPropValue<IntoSchemaPropValueWrapper<EventHandler<Args>>> for F
where
   F: FnMut(Args) + MaybeSend + 'static,
   Args: MaybeSend + 'static,
{
   fn into(self) -> IntoSchemaPropValueWrapper<EventHandler<Args>> {
      IntoSchemaPropValueWrapper(EventHandler::new(self))
   }
}
