pub mod common_renderer;

pub use common_renderer::*;
use wasm_bindgen::intern;

mod attr_values;
pub mod attrs;
pub mod elements;
pub mod event;

use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::cell::RefCell;
use std::future::Future;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use hashbrown::HashMap;
use slotmap::{Key, KeyData, SlotMap};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, Node, Window};

use rxy_core::{
   AttrIndex, DeferredNodeTreeScoped, Element, ElementAttr, ElementAttrType, ElementTypeUnTyped,
   ElementViewChildren, IntoView, MaybeSend, MaybeSync, NodeTree, Renderer, RendererNodeId,
   RendererWorld, View, ViewCtx, ViewKey,
};

pub fn log(s: &str) {
   web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s));
}

pub type WebElement<E, VM> = Element<WebRenderer, E, VM>;

pub type WebElementViewChildren<CV, E, VM> =
   ElementViewChildren<WebRenderer, Element<WebRenderer, E, VM>, CV>;

pub type WebElementAttrMember<EA> = ElementAttr<WebRenderer, EA>;

pub struct WebTask;

#[derive(Default)]
pub struct NodeStates(HashMap<TypeId, Option<Box<dyn Any>>>);

impl NodeStates {
   pub fn new() -> Self {
      Default::default()
   }

   pub fn get<S: 'static>(&self) -> Option<&S> {
      self
         .0
         .get(&TypeId::of::<S>())
         .and_then(|state| state.as_ref().unwrap().downcast_ref())
   }

   pub fn get_mut<S: 'static>(&mut self) -> Option<&mut S> {
      self
         .0
         .get_mut(&TypeId::of::<S>())
         .and_then(|state| state.as_mut().unwrap().downcast_mut())
   }

   pub fn remove<S: 'static>(&mut self) -> Option<S> {
      self
         .0
         .remove(&TypeId::of::<S>())
         .and_then(|state| state.unwrap().downcast().ok().map(|state| *state))
   }

   pub fn set<S: 'static>(&mut self, state: S) {
      self.0.insert(TypeId::of::<S>(), Some(Box::new(state)));
   }

   pub fn try_scoped<U, S: 'static>(&mut self, f: impl FnOnce(Option<&mut S>) -> U) -> U {
      if let Some(boxed_state) = self.0.get_mut(&TypeId::of::<S>()) {
         let taken_boxed_state = boxed_state.take().unwrap();
         let mut state = taken_boxed_state.downcast().unwrap();
         let r = f(Some(&mut *state));
         *boxed_state = Some(state);
         r
      } else {
         f(None)
      }
   }
   pub fn scoped<U, S: 'static>(&mut self, f: impl FnOnce(&mut S) -> U) -> Option<U> {
      if let Some(boxed_state) = self.0.get_mut(&TypeId::of::<S>()) {
         let taken_boxed_state = boxed_state.take().unwrap();
         let mut state = taken_boxed_state.downcast().unwrap();
         let r = f(&mut *state);
         *boxed_state = Some(state);
         Some(r)
      } else {
         None
      }
   }
}

impl Deref for NodeStates {
   type Target = HashMap<TypeId, Option<Box<dyn Any>>>;

   fn deref(&self) -> &Self::Target {
      &self.0
   }
}

impl DerefMut for NodeStates {
   fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
   }
}

slotmap::new_key_type! {
    pub struct NodeStateId;
}

impl NodeStateId {
   pub fn to_string(&self) -> String {
      self.data().as_ffi().to_string()
   }

   pub fn from_string(str: String) -> Self {
      NodeStateId::from(KeyData::from_ffi(str.parse().unwrap()))
   }
}

pub struct WebDomNodeStates {
   states: SlotMap<NodeStateId, NodeStates>,
}

impl WebDomNodeStates {
   fn ensure_spawn_data_id(&mut self, reserve_node_id: &RendererNodeId<WebRenderer>) {
      if reserve_node_id.get_state_id().is_none() {
         let id = self.states.insert(NodeStates::new());
         reserve_node_id.set_state_id(id);
      }
   }
}

thread_local! {
    static DOM_NODE_TREE: RefCell<WebDomNodeStates> = RefCell::new(WebDomNodeStates {
        states: Default::default(),
    });
}

pub fn build_on_body<V>(view: V) -> <V::View as View<WebRenderer>>::Key
where
   V: IntoView<WebRenderer>,
{
   DOM_NODE_TREE.with_borrow_mut(|n| {
      view.into_view().build(
         ViewCtx {
            world: n,
            parent: body().into(),
         },
         None,
         false,
      )
   })
}

pub trait DomNodeExt {
   fn get_state_id(&self) -> Option<NodeStateId>;
   fn set_state_id(&self, id: NodeStateId);
}

impl DomNodeExt for Node {
   fn get_state_id(&self) -> Option<NodeStateId> {
      match self.node_type() {
         Node::ELEMENT_NODE => self
            .dyn_ref::<web_sys::Element>()
            .unwrap()
            .get_attribute(ID_ATTR)
            .map(NodeStateId::from_string),
         Node::COMMENT_NODE => self.node_value().map(NodeStateId::from_string),
         // Node::TEXT_NODE
         _ => {
            unreachable!()
         }
      }
   }
   fn set_state_id(&self, id: NodeStateId) {
      match self.node_type() {
         Node::ELEMENT_NODE => self
            .dyn_ref::<web_sys::Element>()
            .unwrap()
            .set_attribute(ID_ATTR, id.to_string().as_str())
            .unwrap(),
         Node::COMMENT_NODE => self.set_node_value(Some(id.to_string().as_str())),
         // Node::TEXT_NODE
         _ => {
            unreachable!()
         }
      }
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WebRenderer;

impl Renderer for WebRenderer {
   type NodeId = Node;
   type NodeTree = WebDomNodeStates;
   type Task<T: MaybeSend + 'static> = ();

   fn spawn_task<T>(future: impl Future<Output = T> + MaybeSend + 'static) -> Self::Task<T>
   where
      T: MaybeSend + 'static,
   {
      use wasm_bindgen_futures::spawn_local;

      spawn_local(async move {
         future.await;
      });
   }
}

const ID_ATTR: &'static str = "data-rxy-id";

pub fn window() -> Window {
   web_sys::window().unwrap()
}

pub fn document() -> Document {
   window().document().unwrap()
}

pub fn body() -> HtmlElement {
   document().body().unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WebDomScoped;

impl DeferredNodeTreeScoped<WebRenderer> for WebDomScoped {
   fn scoped(&self, f: impl FnOnce(&mut RendererWorld<WebRenderer>) + 'static) {
      DOM_NODE_TREE.with_borrow_mut(|tree| f(&mut tree.borrow_mut()));
   }
}

impl NodeTree<WebRenderer> for WebDomNodeStates {
   fn prepare_set_attr_and_get_is_init(
      &mut self,
      node_id: &RendererNodeId<WebRenderer>,
      attr_index: AttrIndex,
   ) -> bool {
      false
   }

   fn set_attr<A: ElementAttrType<WebRenderer>>(
      &mut self,
      node_id: RendererNodeId<WebRenderer>,
      value: A::Value,
   ) {
      A::update_value(self, node_id, value);
   }

   fn unset_attr<A: ElementAttrType<WebRenderer>>(&mut self, node_id: RendererNodeId<WebRenderer>) {
      if let Some(element) = node_id.dyn_ref::<web_sys::Element>() {
         let _ = element.remove_attribute(intern(A::NAME));
      } else {
      }
   }

   fn deferred_world_scoped(&self) -> impl DeferredNodeTreeScoped<WebRenderer> {
      WebDomScoped
   }

   fn get_node_state_mut<S: MaybeSend + MaybeSync + 'static>(
      &mut self,
      node_id: &RendererNodeId<WebRenderer>,
   ) -> Option<&mut S> {
      // self.ensure_spawn_data_id(node_id);
      self
         .states
         .get_mut(node_id.get_state_id()?)
         .and_then(|states| states.get_mut::<S>())
   }

   fn get_node_state_ref<S: MaybeSend + MaybeSync + 'static>(
      &self,
      node_id: &RendererNodeId<WebRenderer>,
   ) -> Option<&S> {
      // self.ensure_spawn_data_id(node_id);
      self
         .states
         .get(node_id.get_state_id()?)
         .and_then(|states| states.get::<S>())
   }

   fn take_node_state<S: MaybeSend + MaybeSync + 'static>(
      &mut self,
      node_id: &RendererNodeId<WebRenderer>,
   ) -> Option<S> {
      // self.ensure_spawn_data_id(node_id);
      self
         .states
         .get_mut(node_id.get_state_id()?)
         .and_then(|node_states| node_states.remove::<S>())
   }

   fn set_node_state<S: MaybeSend + MaybeSync + 'static>(
      &mut self,
      node_id: &RendererNodeId<WebRenderer>,
      state: S,
   ) {
      self.ensure_spawn_data_id(node_id);
      self
         .states
         .get_mut(node_id.get_state_id().unwrap())
         .unwrap()
         .set(state);
   }

   fn exist_node_id(&mut self, node_id: &RendererNodeId<WebRenderer>) -> bool {
      node_id
         .get_state_id()
         .is_some_and(|id| self.states.contains_key(id))
   }

   fn reserve_node_id(&mut self) -> RendererNodeId<WebRenderer> {
      document().create_comment("reserve_node").into()
   }

   fn spawn_placeholder(
      &mut self,
      name: impl Into<Cow<'static, str>>,
      parent: Option<&RendererNodeId<WebRenderer>>,
      reserve_node_id: Option<RendererNodeId<WebRenderer>>,
   ) -> RendererNodeId<WebRenderer> {
      let comment = if let Some(reserve_node_id) = reserve_node_id {
         reserve_node_id
      } else {
         document().create_comment(&name.into()).into()
      };
      if let Some(parent) = parent {
         parent.append_child(&comment).unwrap();
      } else {
         body().append_child(&comment).unwrap();
      }
      comment.into()
   }

   fn ensure_spawn(&mut self, reserve_node_id: RendererNodeId<WebRenderer>) {
      self.ensure_spawn_data_id(&reserve_node_id)
   }

   fn spawn_empty_node(
      &mut self,
      parent: Option<&RendererNodeId<WebRenderer>>,
      reserve_node_id: Option<RendererNodeId<WebRenderer>>,
   ) -> RendererNodeId<WebRenderer> {
      self.spawn_placeholder("empty", parent, reserve_node_id)
   }

   fn spawn_data_node(&mut self) -> RendererNodeId<WebRenderer> {
      let node = self.spawn_placeholder("data", None, None);
      let id = self.states.insert(NodeStates::new());
      node.set_state_id(id);
      node
   }

   fn get_parent(
      &self,
      node_id: &RendererNodeId<WebRenderer>,
   ) -> Option<RendererNodeId<WebRenderer>> {
      node_id.parent_node()
   }

   fn remove_node(&mut self, node_id: &RendererNodeId<WebRenderer>) {
      let parent = node_id.parent_node().unwrap();
      parent.remove_child(&node_id).unwrap();
      if let Some(id) = node_id.get_state_id() {
         self.states.remove(id);
      }
   }

   fn insert_before(
      &mut self,
      parent: Option<&RendererNodeId<WebRenderer>>,
      before_node_id: Option<&RendererNodeId<WebRenderer>>,
      inserted_node_ids: &[RendererNodeId<WebRenderer>],
   ) {
      for node in inserted_node_ids {
         if let Some(parent) = parent {
            parent.insert_before(&node, before_node_id).unwrap();
         } else {
            body().append_child(&node).unwrap();
         }
      }
   }

   fn set_visibility(&mut self, hidden: bool, node_id: &RendererNodeId<WebRenderer>) {
      if let Some(element) = node_id.dyn_ref::<HtmlElement>() {
         element
            .style()
            .set_property("visibility", if hidden { "hidden" } else { "visible" })
            .unwrap();
      } else {
         unreachable!()
      }
   }

   fn get_visibility(&self, node_id: &RendererNodeId<WebRenderer>) -> bool {
      if let Some(element) = node_id.dyn_ref::<HtmlElement>() {
         element.style().get_property_value("visibility").as_deref() == Ok("hidden")
      } else {
         unreachable!()
      }
   }
}

impl ViewKey<WebRenderer> for Node {
   fn remove(self, world: &mut RendererWorld<WebRenderer>) {
      world.remove_node(&self);
   }

   fn insert_before(
      &self,
      world: &mut RendererWorld<WebRenderer>,
      parent: Option<&RendererNodeId<WebRenderer>>,
      before_node_id: Option<&RendererNodeId<WebRenderer>>,
   ) {
      world.insert_before(parent, before_node_id, core::slice::from_ref(self));
   }

   fn set_visibility(&self, world: &mut RendererWorld<WebRenderer>, hidden: bool) {
      world.set_visibility(hidden, self);
   }

   fn state_node_id(&self) -> Option<RendererNodeId<WebRenderer>> {
      Some(self.clone())
   }

   fn reserve_key(
      world: &mut RendererWorld<WebRenderer>,
      _will_rebuild: bool,
      parent: RendererNodeId<WebRenderer>,
      spawn: bool,
   ) -> Self {
      world.reserve_node_id_or_spawn(parent, spawn)
   }

   fn first_node_id(
      &self,
      _world: &RendererWorld<WebRenderer>,
   ) -> Option<RendererNodeId<WebRenderer>> {
      Some(self.clone())
   }
}
