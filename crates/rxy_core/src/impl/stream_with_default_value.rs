/* use crate::{mutable_view_rebuild, IntoView, MutableView, Renderer, RendererNodeId, View, ViewCtx, ViewReBuilder, stream};
use bevy_utils::futures::now_or_never;
use futures_lite::{Stream, StreamExt};

pub struct StreamWithDefaultValue<S>
where
    S: Stream + Send + 'static,
{
    stream: S,
    default_value: S::Item,
}

// impl<R, S> IntoView<R> for StreamWithDefaultValue<S>
// where
//     R: Renderer,
//     S: Stream + Send + 'static,
//     S::Item: IntoView<R>,
// {
//     type View = futures_lite::stream::Boxed<<S::Item as IntoView<R>>::View>;
//
//     fn into_view(self) -> Self::View {
//         self.0.map(|n| n.into_view()).boxed()
//     }
// }

pub fn stream_with_default_value<R, S>(
    stream: S,
    default_value: S::Item,
) -> StreamWithDefaultValue<S>
where
    R: Renderer,
    S: Stream + Send + 'static,
    S::Item: IntoView<R>,
{
    StreamWithDefaultValue {
        stream,
        default_value,
    }
}

impl<R, S> View<R> for StreamWithDefaultValue<S>
where
    R: Renderer,
    S: Stream + Send + 'static,
    S::Item: IntoView<R>,
{
    type Key = ();

    fn build(
        self,
        ctx: ViewCtx<R>,
        reserve_key: Option<Self::Key>,
        will_rebuild: bool,
    ) -> Self::Key {
        StreamWithDefaultValue {
            stream,
            default_value,
        } = self;
        if let Some(item) = now_or_never(stream.next()) {
            let Some(v) = item else {
                return;
            };
            mutable_view_rebuild(
                v,
                ViewCtx {
                    world: &mut *ctx.world,
                    parent: ctx.parent.clone(),
                },
                state_node_id.clone(),
            );
        }

        let mut re_builder = R::get_view_re_builder(ctx);

        R::spawn_and_detach({
            async move {
                while let Some(v) = stream.next().await {
                    re_builder.mutable_rebuild(v, &state_node_id);
                }
            }
        });
    }

    fn rebuild(self, ctx: ViewCtx<R>, key: Self::Key) {
    }
}
 */