use crate::{BoxedCloneableErasureView, BoxedErasureView, MaybeSendAnyBox, PropState, Renderer, RendererNodeId, RendererWorld, SchemaParam};
use alloc::boxed::Box;
use crate::utils::HashMap;
use core::any::TypeId;
use core::marker::PhantomData;
use core::cell::UnsafeCell;

pub type BoxedPropValue = MaybeSendAnyBox;

pub type PropHashMap<R> = HashMap<TypeId, Box<dyn PropState<R>>>;

pub struct InnerSchemaCtx<'a, R, U = ()>
where
    R: Renderer,
    U: ?Sized,
{
    pub world: &'a mut RendererWorld<R>,
    pub parent: RendererNodeId<R>,
    pub(crate) slots: &'a mut HashMap<TypeId, BoxedErasureView<R>>,
    pub(crate) cloneable_slots: &'a mut HashMap<TypeId, BoxedCloneableErasureView<R>>,
    pub(crate) prop_state: &'a mut PropHashMap<R>,
    #[cfg(feature = "xy_reactive")]
    pub(crate) effect_state: &'a mut Vec<xy_reactive::effect::ErasureEffect>,
    pub(crate) init_values: HashMap<TypeId, BoxedPropValue>,
    pub(crate) _marker: PhantomData<U>,
}

impl<'a, R, U> InnerSchemaCtx<'a, R, U>
where
    R: Renderer,
{
    pub fn prop_state(&mut self) -> &mut PropHashMap<R> {
        self.prop_state
    }

    #[cfg(feature = "xy_reactive")]
    pub fn effect_state(&mut self) -> &mut Vec<xy_reactive::effect::ErasureEffect> {
        self.effect_state
    }

    pub fn cast<T>(self) -> InnerSchemaCtx<'a, R, T> {
        InnerSchemaCtx::<'a, R, T> {
            world: self.world,
            parent: self.parent,
            slots: self.slots,
            init_values: self.init_values,
            prop_state: self.prop_state,
            _marker: Default::default(),
            cloneable_slots: self.cloneable_slots,
            #[cfg(feature = "xy_reactive")]
            effect_state: self.effect_state,
        }
    }

    pub fn get_init_value<T: 'static>(&mut self, prop_type_id: TypeId) -> Option<T> {
        self.init_values
            .remove(&prop_type_id)
            .map(|v| *v.downcast::<T>().unwrap())
    }
}

pub struct RenderSchemaCtx<R>
where
    R: Renderer,
{
    inner: UnsafeCell<*mut InnerSchemaCtx<'static, R>>,
}

impl<R> RenderSchemaCtx<R>
where
    R: Renderer,
{
    pub fn mut_scoped<U>(&mut self, f: impl FnOnce(&mut InnerSchemaCtx<'_, R>) -> U) -> U
    where
        U: 'static,
    {
        let ctx = unsafe { &mut **self.inner.get() };
        f(ctx)
    }

    pub fn ref_scoped<U>(&self, f: impl FnOnce(&InnerSchemaCtx<'_, R>) -> U) -> U
    where
        U: 'static,
    {
        let ctx = unsafe { &**self.inner.get() };
        f(ctx)
    }
    pub fn world_mut_scoped<U>(&mut self, f: impl FnOnce(&mut RendererWorld<R>) -> U) -> U
    where
        U: 'static,
    {
        let ctx = unsafe { &mut **self.inner.get() };
        f(ctx.world)
    }

    pub fn world_ref_scoped<U>(&self, f: impl FnOnce(&RendererWorld<R>) -> U) -> U
    where
        U: 'static,
    {
        let ctx = unsafe { &**self.inner.get() };
        f(ctx.world)
    }
}

impl<R> SchemaParam<R> for RenderSchemaCtx<R>
where
    R: Renderer,
{
    fn from<const I: usize>(ctx: &mut InnerSchemaCtx<R>) -> Self {
        RenderSchemaCtx {
            inner: UnsafeCell::new(ctx as *mut InnerSchemaCtx<'_, R> as _),
        }
    }
}
#[cfg(feature = "send_sync")]
unsafe impl<R> crate::MaybeSend for RenderSchemaCtx<R> where R: Renderer {}
