use bevy_a11y::Focus;
use bevy_app::{Plugin, PreUpdate};
use bevy_ecs::prelude::{DetectChangesMut, Entity, Local, Query, Res};
use bevy_ecs::{
    component::Component, prelude::resource_changed, schedule::IntoSystemConfigs,
    system::SystemParam,
};
use bevy_reflect::Reflect;
use bevy_ui::UiSystem;

pub struct FocusablePlugin {}

impl Plugin for FocusablePlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<Focusable>().add_systems(
            PreUpdate,
            FocusableSystemParam::update_focused_state
                // .in_set(UiSystem::Interactions)
                .after(UiSystem::Focus)
                .run_if(resource_changed::<Focus>()),
        );
    }
}

#[derive(SystemParam)]
struct FocusableSystemParam<'w, 's> {
    query: Query<'w, 's, &'static mut Focusable>,
}

impl FocusableSystemParam<'_, '_> {
    pub fn set_focus_state(&mut self, entity: Entity, focus_state: FocusState) {
        if let Ok(mut focusable) = self.query.get_mut(entity) {
            focusable.set_if_neq(Focusable { focus_state });
        }
    }

    #[inline(always)]
    pub fn blur(&mut self, entity: Entity) {
        self.set_focus_state(entity, FocusState::None)
    }
    #[inline(always)]
    pub fn focus(&mut self, entity: Entity) {
        self.set_focus_state(entity, FocusState::Focused)
    }

    pub fn update_focused_state(
        mut query: FocusableSystemParam,
        focus: Res<Focus>,
        mut old_focused: Local<Option<Entity>>,
    ) {
        // Remove the interaction from the last focused entity
        if let Some(old_focused) = *old_focused {
            if focus.0 != Some(old_focused) {
                query.blur(old_focused);
            }
        }

        // Set the focused interaction on the newly focused entity
        if let Some(new_focused) = focus.0 {
            query.focus(new_focused);
        }

        *old_focused = focus.0;
    }
}

/// A component that represents if a UI element is focused.
#[derive(Reflect, Component, Clone, Debug, Default, Eq, PartialEq)]
pub struct Focusable {
    pub focus_state: FocusState,
}

impl Focusable {
    /// The entity is currently focused, similar to the `:focus` css pseudo-class.
    /// To check if the focus has been achieved through keyboard navigation, see [`Focusable::is_focus_visible`].
    pub fn is_focused(&self) -> bool {
        matches!(self.focus_state, FocusState::Focused /*  { .. } */)
    }

    /*     /// Focus has been reached through keyboard navigation and so a focus style should be displayed.
    /// This is similar to the `:focus-visible` pseudo-class in css.
    pub fn is_focus_visible(&self) -> bool {
        matches!(self.focus_state, FocusState::Focused { visible: true })
    } */
}

#[derive(Reflect, Clone, Debug, Default, Eq, PartialEq)]
pub enum FocusState {
    /// Entity is not focused
    #[default]
    None,
    /// Entity is focused
    Focused, /*  {
                 /// Focus has been reached through keyboard navigation and so a focus style should be displayed.
                 /// This is similar to the `:focus-visible` pseudo-class in css.
                 // visible: bool,
             } */
}
