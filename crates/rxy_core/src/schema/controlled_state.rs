use crate::{InnerSchemaCtx, RenderSchemaCtx, Renderer, MaybeSend, MaybeSync};
use async_channel::Sender;
use core::fmt::Debug;
use xy_reactive::prelude::{
    create_effect, use_rw_signal, ReadSignal, RwSignal, SignalGet, SignalGetUntracked, SignalSet,
};

impl<R> RenderSchemaCtx<R>
    where
        R: Renderer,
{
    pub fn use_controlled_state<T>(
        &mut self,
        value: ReadSignal<T>,
        onchange: Sender<T>,
    ) -> RwSignal<T>
        where
            T: Debug + MaybeSend + MaybeSync + PartialEq + Clone + 'static,
    {
        self.mut_scoped(|ctx| ctx.use_controlled_state(value, onchange))
    }
}

impl<'a, R, U> InnerSchemaCtx<'a, R, U>
    where
        R: Renderer,
{
    pub fn use_controlled_state<T>(
        &mut self,
        value: ReadSignal<T>,
        onchange: Sender<T>,
    ) -> RwSignal<T>
        where
            T: Debug + MaybeSend + MaybeSync + PartialEq + Clone + 'static,
    {
        let signal = use_rw_signal(value.get_untracked());
        let (read_signal, write_signal) = signal.split();
        let event_effect = create_effect(move |_| {
            let value = read_signal.get();
            onchange.send_blocking(value).unwrap();
        });
        let control_effect = create_effect(move |_| {
            let value = value.get();
            if value != read_signal.get_untracked() {
                write_signal.set(value);
            }
        });

        self.effect_state()
            .extend([event_effect.erase(), control_effect.erase()]);

        signal
    }
}
