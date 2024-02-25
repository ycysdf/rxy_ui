use bevy::prelude::*;
use rxy_bevy::navigation::RxyKeyboardNavigationPlugin;
use rxy_ui::prelude::*;
use std::borrow::Cow;

use async_channel::Receiver;
use std::fmt::Debug;
use std::ops::Deref;

mod components;

use components::*;
use hooked_collection::{HookVec, HookedVec, VecOperation};
use rxy_bevy::vec_data_source::use_hooked_vec_resource_source;
use rxy_core::UseListOperation;

pub const COLOR_PRIMARY: Color = Color::BLUE;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    Setting,
    InGame,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RxyPlugin::default(),
        RxyStyleSheetPlugin::default(),
        RxyKeyboardNavigationPlugin::default(),
    ))
    .add_state::<GameState>()
    .add_systems(Startup, setup);

    app.run();
}

const INVENTORY_WIDTH: u16 = 10;
const INVENTORY_HEIGHT: u16 = 5;

fn setup(mut world: &mut World) {
    world.spawn(Camera2dBundle::default());

    let (sender, receiver) = async_channel::unbounded();
    let items = vec![None; (INVENTORY_WIDTH * INVENTORY_HEIGHT).into()];
    world.insert_resource(InventoryItems(HookedVec::from_vec(items, sender)));
    world.insert_resource(InventoryItemsOpReceiver(receiver));

    world.spawn_rxy_ui(game_ui);
}

#[derive(TypedStyle)]
struct FocusStyle;

#[derive(Clone, Debug)]
pub struct InventoryItem {
    pub name: String,
    pub icon: Handle<Image>,
    pub count: u32,
}

#[derive(Resource, Deref, DerefMut)]
pub struct InventoryItems(
    HookedVec<Option<InventoryItem>, Sender<VecOperation<Option<InventoryItem>>>>,
);

#[derive(Resource)]
pub struct InventoryItemsOpReceiver(Receiver<VecOperation<Option<InventoryItem>>>);

fn game_ui() -> impl IntoView<BevyRenderer> {
    div()
        .p(20)
        .size_screen()
        .flex()
        .center()
        .children(view_builder(|ctx: ViewCtx<BevyRenderer>, _| {
            let receiver = ctx
                .world
                .remove_resource::<InventoryItemsOpReceiver>()
                .unwrap();
            let source = use_hooked_vec_resource_source::<InventoryItems>(receiver.0);
            div()
                .bg_color(Color::GRAY)
                .grid()
                .gap(10)
                .padding(10)
                .grid_template_columns(vec![RepeatedGridTrack::auto(INVENTORY_WIDTH)])
                .grid_template_rows(vec![RepeatedGridTrack::auto(INVENTORY_HEIGHT)])
                .children(x_iter_source(source, |item: Cow<Option<InventoryItem>>| {
                    let root = div().size(50).style(
                        x().bg_color(Color::WHITE)
                            .border(1)
                            .border_color(Color::BLACK),
                    );
                    root.children(if item.as_ref().is_some() {
                        let item = item.into_owned().unwrap();
                        (img().size_full().src(item.icon),).either_left()
                    } else {
                        ().either_right()
                    })
                }))
        }))
}

#[derive(ElementSchema)]
pub struct InventoryItemView {}

impl SchemaElementView<BevyRenderer> for InventoryItemView {
    fn view(self) -> impl IntoElementView<BevyRenderer> {
        let InventoryItemView { .. } = self;
        div()
    }
}
