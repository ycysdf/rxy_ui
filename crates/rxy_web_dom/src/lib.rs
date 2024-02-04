use crate::renderer::{WebDomNodeStates, WebRenderer};
use rxy_core::{RendererNodeId, RendererWorld};
use rxy_element::{AttrIndex, ElementNodeTree, ElementAttrUntyped};

pub mod renderer;

impl ElementNodeTree<WebRenderer> for WebDomNodeStates {
    fn prepare_set_attr_and_get_is_init(
        &mut self,
        node_id: &RendererNodeId<WebRenderer>,
        attr_index: AttrIndex,
    ) -> bool {
        false
    }
}

pub struct WebWrapper<T>(T);

pub mod prelude {}
