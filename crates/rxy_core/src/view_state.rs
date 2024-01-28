use crate::Renderer;

pub struct ViewState<S>(S);

pub trait RendererViewExt: Renderer {
    fn get_view_state_mut<'w, S: Send + Sync + 'static>(
        world: &'w mut Self::World,
        node_id: &Self::NodeId,
    ) -> Option<&'w mut S> {
        Self::get_state_mut::<ViewState<S>>(world, node_id).map(|n| &mut n.0)
    }
    fn get_view_state_ref<'w, S: Send + Sync + 'static>(
        world: &'w Self::World,
        node_id: &Self::NodeId,
    ) -> Option<&'w S> {
        Self::get_state_ref::<ViewState<S>>(world, node_id).map(|n| &n.0)
    }
    fn set_view_state<S: Send + Sync + 'static>(
        world: &mut Self::World,
        node_id: &Self::NodeId,
        state: S,
    ) {
        Self::set_state::<ViewState<S>>(world, node_id, ViewState(state))
    }
}

impl<T: Renderer> RendererViewExt for T {}
