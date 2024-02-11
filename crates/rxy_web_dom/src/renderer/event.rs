use crate::WebRenderer;
use rxy_core::MemberOwner;
use rxy_core::{
    count_macro,
    prelude::{ViewMember, ViewMemberCtx},
    ViewMemberIndex,
};
use wasm_bindgen::{
    intern,
    prelude::{Closure, JsCast, JsValue},
};

pub trait WebRendererEventType {
    const NAME: &'static str;
}

macro_rules! define_events {
    ($($name:ident $html:literal)*) => {
      impl<T> HtmlElementEvents for T where T: MemberOwner<WebRenderer> + Sized {}

      pub trait HtmlElementEvents: MemberOwner<WebRenderer> + Sized {
         count_macro::count! {
            $(
            fn $name(self, closure: impl FnMut(JsValue) + 'static) -> Self::AddMember<WebEventViewMember<_int_>> {
               self.member(WebEventViewMember {
                  closure: Box::new(closure),
               })
            }
            )*
         }
      }
      count_macro::count! {
         $(
            impl WebRendererEventType for WebEventViewMember<_int_> {
               const NAME: &'static str = $html;
            }
         )*
      }
    };
}

define_events! {
   on_abort "abort"
   on_auto_complete "autocomplete"
   on_auto_complete_error "autocompleteerror"
   on_blur "blur"
   on_cancel "cancel"
   on_canplay "canplay"
   on_canplay_through "canplaythrough"
   on_change "change"
   on_click "click"
   on_close "close"
   on_context_menu "contextmenu"
   on_cue_change "cuechange"
   on_dbl_click "dblclick"
   on_drag "drag"
   on_dragend "dragend"
   on_dragenter "dragenter"
   on_dragleave "dragleave"
   on_dragover "dragover"
   on_dragstart "dragstart"
   on_drop "drop"
   on_duration_change "durationchange"
   on_emptied "emptied"
   on_ended "ended"
   on_error "error"
   on_focus "focus"
   on_input "input"
   on_invalid "invalid"
   on_key_down "keydown"
   on_key_press "keypress"
   on_keyup "keyup"
   on_load "load"
   on_loaded_data "loadeddata"
   on_loaded_metadata "loadedmetadata"
   on_load_start "loadstart"
   on_mouse_down "mousedown"
   on_mouse_enter "mouseenter"
   on_mouse_leave "mouseleave"
   on_mouse_move "mousemove"
   on_mouse_out "mouseout"
   on_mouse_over "mouseover"
   on_mouse_up "mouseup"
   on_mouse_wheel "mousewheel"
   on_pause "pause"
   on_play "play"
   on_playing "playing"
   on_progress "progress"
   on_rate_change "ratechange"
   on_reset "reset"
   on_resize "resize"
   on_scroll "scroll"
   on_seeked "seeked"
   on_seeking "seeking"
   on_select "select"
   on_show "show"
   on_sort "sort"
   on_stalled "stalled"
   on_submit "submit"
   on_suspend "suspend"
   on_time_update "timeupdate"
   on_toggle "toggle"
   on_volume_change "volumechange"
   on_waiting "waiting"
}

pub struct WebEventState {
    pub closure: Closure<dyn FnMut(JsValue)>,
}

pub struct WebEventViewMember<const I: usize> {
    pub closure: Box<dyn FnMut(JsValue)>,
}

impl<const I: usize> ViewMember<WebRenderer> for WebEventViewMember<I>
where
    Self: WebRendererEventType,
{
    fn count() -> ViewMemberIndex {
        1
    }

    fn unbuild(mut ctx: ViewMemberCtx<WebRenderer>, _view_removed: bool) {
        let state = ctx
            .take_indexed_view_member_state::<WebEventState>()
            .unwrap();

        ctx.node_id
            .remove_event_listener_with_callback(
                intern(<Self as WebRendererEventType>::NAME),
                state.closure.as_ref().unchecked_ref(),
            )
            .unwrap();
    }

    fn build(self, mut ctx: ViewMemberCtx<WebRenderer>, _will_rebuild: bool) {
        let closure = wasm_bindgen::closure::Closure::wrap(self.closure);
        ctx.node_id
            .add_event_listener_with_callback(
                intern(<Self as WebRendererEventType>::NAME),
                closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        ctx.set_indexed_view_member_state(WebEventState { closure });
    }

    fn rebuild(self, ctx: ViewMemberCtx<WebRenderer>) {
        Self::unbuild(
            ViewMemberCtx {
                index: ctx.index,
                world: &mut *ctx.world,
                node_id: ctx.node_id.clone(),
            },
            false,
        );
        self.build(ctx, true);
    }
}
