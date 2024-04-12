use bevy_app::{Plugin, PreUpdate};
use bevy_ecs::query::With;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_ecs::{change_detection::DetectChangesMut, entity::Entity, prelude::Resource};
use bevy_input::prelude::KeyCode;
use bevy_input::{ButtonInput, InputSystem};
use bevy_render::view::ViewVisibility;
use bevy_ui::{Interaction, UiStack, UiSystem};

use crate::{Focusable, FocusedEntity};

#[derive(Default, Debug)]
pub struct RxyKeyboardNavigationPlugin {}

impl Plugin for RxyKeyboardNavigationPlugin {
   fn build(&self, app: &mut bevy_app::App) {
      app.init_resource::<KeyboardNavigationInput>()
         .add_systems(
            PreUpdate,
            keyboard_navigation_system
               .in_set(UiSystem::Focus)
               .run_if(tab_pressed)
               .after(InputSystem),
         )
         .add_systems(
            PreUpdate,
            keyboard_click
               .after(UiSystem::Focus)
               // .in_set(UiSystem::Interactions)
               .run_if(trigger_click),
         )
         .add_systems(
            PreUpdate,
            end_keyboard_click
               .after(UiSystem::Focus)
               // .in_set(UiSystem::Interactions)
               .run_if(trigger_click_end),
         );
   }
}

/// Resource for the configuration of keyboard navigation of UI with <kbd>tab</kbd>.
#[derive(Resource)]
pub struct KeyboardNavigationInput {
   pub enabled: bool,
}

impl Default for KeyboardNavigationInput {
   fn default() -> Self {
      Self { enabled: true }
   }
}

/// Should the [`keyboard_navigation_system`] run?
pub(crate) fn tab_pressed(
   keyboard_input: Res<ButtonInput<KeyCode>>,
   keyboard_navigation: Res<KeyboardNavigationInput>,
) -> bool {
   keyboard_navigation.enabled && keyboard_input.just_pressed(KeyCode::Tab)
}

/// The system updates the [`Focus`] resource when the user uses keyboard navigation with <kbd>tab</kbd> or <kbd>shift</kbd> + <kbd>tab</kbd>.
///
/// Entities can be focused if [`ComputedVisibility`] is visible and they have the [`Focusable`] component.
pub(crate) fn keyboard_navigation_system(
   mut focus: ResMut<FocusedEntity>,
   mut interactions: Query<&mut Interaction>,
   focusables: Query<&ViewVisibility, With<Focusable>>,
   keyboard_input: Res<ButtonInput<KeyCode>>,
   ui_stack: Res<UiStack>,
) {
   let reverse_order =
      keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight);

   let can_focus = |entity: &&Entity| {
      focusables
         .get(**entity)
         .map_or(false, |computed_visibility| computed_visibility.get())
   };

   let ui_nodes = &ui_stack.uinodes;

   // Current index of the focused entity within the ui nodes list.
   let current_index = ui_nodes
      .iter()
      .position(|&ui_node| Some(ui_node) == focus.0);

   let new_focus = if reverse_order {
      // Start with the entity before the current focused or at the end of the list
      let first_index = current_index.unwrap_or_default();

      let before = ui_nodes.iter().take(first_index);
      let after = ui_nodes.iter().skip(first_index);
      let mut wrapped = before.rev().chain(after.rev());
      wrapped.find(can_focus).copied()
   } else {
      // Start with the entity after the current focused or at the start of the list
      let first_index = current_index.map(|index| index + 1).unwrap_or_default();

      let after = ui_nodes.iter().skip(first_index);
      let before = ui_nodes.iter().take(first_index);
      let mut wrapped = after.chain(before);
      wrapped.find(can_focus).copied()
   };

   // Reset the clicked state
   if new_focus != focus.0 {
      if let Some(mut interaction) = focus.0.and_then(|entity| interactions.get_mut(entity).ok()) {
         if *interaction == Interaction::Pressed {
            *interaction = Interaction::None;
         }
      }
   }

   if focus.0 != new_focus {
      *focus = FocusedEntity(new_focus);
   }
   // focus.set_if_neq(Focus {
   //     entity: new_focus,
   //     focus_visible: true,
   // });
}

/// Should the [`keyboard_click`] system run?
pub(crate) fn trigger_click(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
   keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Enter)
}

/// Trigger the [`Focus`] entity to be clicked.
pub(crate) fn keyboard_click(mut interactions: Query<&mut Interaction>, focus: Res<FocusedEntity>) {
   if let Some(mut interaction) = focus.0.and_then(|entity| interactions.get_mut(entity).ok()) {
      interaction.set_if_neq(Interaction::Pressed);
   }
}

/// Should the [`end_keyboard_click`] system run?
pub(crate) fn trigger_click_end(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
   keyboard_input.just_released(KeyCode::Space) || keyboard_input.just_released(KeyCode::Enter)
}

/// Reset the clicked state.
pub(crate) fn end_keyboard_click(mut interactions: Query<&mut Interaction>) {
   interactions.iter_mut().for_each(|mut interaction| {
      if *interaction == Interaction::Pressed {
         // The click was triggered by the keyboard, so it doesn't make sense to go to `Interaction::Hovered`.
         *interaction = Interaction::None;
      }
   });
}
/*
#[cfg(test)]
mod test {
    use super::Focusable;
    use super::*;
    use bevy_ecs::prelude::*;
    use bevy_ecs::system::CommandQueue;
    use bevy_hierarchy::BuildChildren;
    use bevy_ui::prelude::{ButtonBundle, NodeBundle};
    use bevy_ui::ui_stack_system;
    use bevy_utils::default;

    #[test]
    fn keyboard_navigation() {
        let mut world = World::default();

        let mut schedule = Schedule::default();
        schedule.add_systems((
            ui_stack_system,
            keyboard_navigation_system.after(ui_stack_system),
            update_focused_state.after(keyboard_navigation_system),
        ));

        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &world);

        commands.init_resource::<bevy_a11y::Focus>();
        commands.init_resource::<ButtonInput<KeyCode>>();
        commands.init_resource::<UiStack>();

        let mut children = Vec::new();
        commands.spawn(NodeBundle::default()).with_children(|parent| {
            for _child in 0..3 {
                children.push(
                    parent
                        .spawn(ButtonBundle {
                            computed_visibility: ComputedVisibility::VISIBLE,
                            ..default()
                        })
                        .id(),
                );
            }
        });
        queue.apply(&mut world);

        schedule.run(&mut world);
        assert!(
            world.get::<Focusable>(children[0]).unwrap().is_focused(),
            "navigation should start at the first button"
        );
        schedule.run(&mut world);
        assert!(
            world.get::<Focusable>(children[1]).unwrap().is_focused(),
            "navigation should go to the second button"
        );

        // Simulate pressing shift
        let mut keyboard_input =
            world.get_resource_mut::<ButtonInput<KeyCode>>().expect("keyboard input resource");
        keyboard_input.press(KeyCode::ShiftLeft);

        schedule.run(&mut world);
        assert!(
            world.get::<Focusable>(children[0]).unwrap().is_focused(),
            "backwards navigation"
        );
        schedule.run(&mut world);
        assert!(
            world.get::<Focusable>(children[2]).unwrap().is_focused(),
            "navigation should loop around"
        );
    }
}
 */
