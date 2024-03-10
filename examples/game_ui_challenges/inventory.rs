use bevy::prelude::*;
use rxy_bevy::navigation::RxyKeyboardNavigationPlugin;
use rxy_ui::prelude::*;
use std::borrow::Cow;

use async_channel::Receiver;
use bevy::asset::AssetLoader;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::ui::{FocusPolicy, RelativeCursorPosition};
use bevy::utils::OnDrop;
use bevy_mod_picking::prelude::Pickable;
use rxy_ui::remove_on_drop::RemoveOnDropWorldExt;
use std::fmt::Debug;
use std::ops::Deref;

mod components;

use crate::InventoryDraggingStatus::NoDragging;
use components::*;
use hooked_collection::{HookVec, HookedVec, VecOperation};
use rxy_bevy::vec_data_source::use_hooked_vec_resource_source;
use rxy_core::remove_on_drop::ViewRemoveOnDrop;
use rxy_core::NodeTree;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RxyPlugin::default(),
        RxyStyleSheetPlugin::default(),
        RxyKeyboardNavigationPlugin::default(),
    ))
    .init_resource::<DraggingInventoryItem>()
    .init_resource::<InventoryDraggingStatus>()
    .init_resource::<InventoryCursorPosition>()
    .init_resource::<HoveredInventoryItem>()
    .add_systems(
        Update,
        mange_hover_inventory_item_ui.run_if(resource_changed::<HoveredInventoryItem>),
    )
    .add_systems(Startup, setup);

    app.run();
}

const INVENTORY_WIDTH: u16 = 10;
const INVENTORY_HEIGHT: u16 = 5;

#[derive(Resource)]
pub struct SampleItems(Vec<Item>);

fn setup(mut world: &mut World) {
    world.spawn(Camera2dBundle::default());

    let (sender, receiver) = async_channel::unbounded();
    let mut items =
        vec![InventoryItemContainer::default(); (INVENTORY_WIDTH * INVENTORY_HEIGHT).into()];

    {
        let asset_server = world.resource_mut::<AssetServer>();

        let sample_items = vec![
            Item {
                name: "Item 1".to_string(),
                icon: asset_server.load::<Image>("items/0.png"),
            },
            Item {
                name: "Item 2".to_string(),
                icon: asset_server.load::<Image>("items/1.png"),
            },
            Item {
                name: "Item 3".to_string(),
                icon: asset_server.load::<Image>("items/2.png"),
            },
            Item {
                name: "Item 4".to_string(),
                icon: asset_server.load::<Image>("items/3.png"),
            },
            Item {
                name: "Item 5".to_string(),
                icon: asset_server.load::<Image>("items/4.png"),
            },
        ];
        items[0] = InventoryItemContainer::new(sample_items[0].clone(), 2);
        items[4] = InventoryItemContainer::new(sample_items[4].clone(), 1);
        items[10] = InventoryItemContainer::new(sample_items[0].clone(), 1);
        items[20] = InventoryItemContainer::new(sample_items[1].clone(), 10);
        items[33] = InventoryItemContainer::new(sample_items[1].clone(), 2);
        items[35] = InventoryItemContainer::new(sample_items[3].clone(), 4);
        items[8] = InventoryItemContainer::new(sample_items[2].clone(), 6);
        world.insert_resource(SampleItems(sample_items));
    }
    world.insert_resource(InventoryItems(HookedVec::from_vec(items, sender)));
    world.insert_resource(InventoryItemsOpReceiver(receiver));

    world.spawn_view_on_root(game_ui());
}

#[derive(TypedStyle)]
struct FocusStyle;

#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub icon: Handle<Image>,
}

#[derive(Clone, Debug)]
pub struct InventoryItem {
    pub item: Item,
    pub count: u32,
}

#[derive(Clone, Debug, PropValueWrapper, Default)]
pub struct InventoryItemContainer(Option<InventoryItem>);

impl InventoryItemContainer {
    pub fn new(item: Item, count: u32) -> Self {
        Self(Some(InventoryItem { item, count }))
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct InventoryItems(
    HookedVec<InventoryItemContainer, Sender<VecOperation<InventoryItemContainer>>>,
);

#[derive(Resource)]
pub struct InventoryItemsOpReceiver(Receiver<VecOperation<InventoryItemContainer>>);

fn game_ui() -> impl IntoView<BevyRenderer> {
    div()
        .p(20)
        .size_screen()
        .flex()
        .flex_col()
        .center()
        .children((view_builder(|ctx: ViewCtx<BevyRenderer>, _| {
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
                .on_pointer_move(
                    |mut cursor_position: ResMut<InventoryCursorPosition>,
                     e: Res<ListenerInputPointerMove>| {
                        cursor_position.0 = e.pointer_location.position;
                    },
                )
                .grid_template_columns(vec![RepeatedGridTrack::auto(INVENTORY_WIDTH)])
                .grid_template_rows(vec![RepeatedGridTrack::auto(INVENTORY_HEIGHT)])
                .children(x_iter_source(
                    source,
                    |item: Cow<InventoryItemContainer>, index: usize| {
                        inventory_item_view(item.into_owned(), index)
                    },
                ))
        }),))
}

#[derive(Resource, Default, Debug)]
pub struct InventoryCursorPosition(Vec2);

#[derive(Resource, Default)]
pub struct HoveredInventoryItem {
    item: Option<(usize, InventoryItem)>,
    view_key: Option<(usize, ViewRemoveOnDrop)>,
}

fn hover_inventory_item_ui(item: InventoryItem) -> impl IntoView<BevyRenderer> {
    div()
        .p(10)
        .border(1)
        .border_color(Color::DARK_GRAY)
        .bg_color(Color::GRAY)
        .absolute()
        .z(2)
        .member(x_res(|cursor_position: &InventoryCursorPosition| {
            ().left(cursor_position.0.x).top(cursor_position.0.y)
        }))
        .children(item.item.name)
}

fn mange_hover_inventory_item_ui(world: &mut World) {
    world.resource_scope(
        |world, mut hovered_inventory_item: Mut<HoveredInventoryItem>| {
            let item = hovered_inventory_item.item.clone();
            match item {
                None => {
                    if hovered_inventory_item.view_key.is_some() {
                        hovered_inventory_item.view_key = None;
                    }
                }
                Some((index, item)) => {
                    if let Some((view_index, _)) = &hovered_inventory_item.view_key {
                        if *view_index == index {
                            return;
                        }
                    }
                    let view_key = world.spawn_view_on_root(hover_inventory_item_ui(item));
                    hovered_inventory_item.view_key = Some((index, world.remove_on_drop(view_key)));
                }
            }
        },
    );
}

#[derive(Resource, Default)]
pub struct DraggingInventoryItem {
    delta: Vec2,
    index: usize,
    view_key: Option<ViewRemoveOnDrop>,
}

impl DraggingInventoryItem {
    pub fn reset(&mut self) {
        self.delta = Default::default();
        self.view_key = None;
    }
}

#[derive(Resource, Default, PartialEq, Eq, Debug, Copy, Clone)]
pub enum InventoryDraggingStatus {
    #[default]
    NoDragging,
    FullDrag,
    PartialDrag,
}

impl InventoryDraggingStatus {
    pub fn is_dragging(&self) -> bool {
        *self != NoDragging
    }
}

#[derive(ElementSchema)]
pub struct InventoryItemView {
    item: Required<ReadSignal<InventoryItemContainer>>,
    index: Required<Static<usize>>,
}

impl SchemaElementView<BevyRenderer> for InventoryItemView {
    fn view(self) -> impl IntoElementView<BevyRenderer> {
        let InventoryItemView {
            item: Required(item),
            index: Required(Static(index)),
        } = self;

        let root = div()
            .size(50)
            .style((
                x().relative()
                    .bg_color(Color::WHITE)
                    .border(1)
                    .border_color(Color::BLACK),
                x_hover().bg_color(Color::GRAY),
            ))
            .on_pointer_drop(
                move |e: Res<ListenerInputPointerDrop>,
                      mut dragging: ResMut<DraggingInventoryItem>,mut is_dragging: ResMut<InventoryDraggingStatus>,
                      mut inventory_items: ResMut<InventoryItems>| {
                    if inventory_items[dragging.index].0.is_none() {
                        return;
                    }
                    inventory_items.swap(index, dragging.index);
                    dragging.reset();
                    *is_dragging = NoDragging;
                },
            )
            // .style(x_res(|is_dragging: &InventoryDraggingStatus| {
            //     is_dragging.0.then_some(x_hover().border_color(Color::BLUE))
            // }))
            ;
        root.children(rx(move || {
            if let Some(item) = item.get().0 {
                fn item_view(InventoryItem { item, count }: InventoryItem,ignore_events: bool) -> impl ElementView<BevyRenderer> {
                    let pickable = if ignore_events {
                        Pickable::IGNORE
                    }else {
                        Pickable::default()
                    };
                    div()
                        .size_full()
                        .absolute()
                        .bundle(pickable.clone())
                        .children((
                            img().m(8).src(item.icon)
                                .bundle(pickable.clone()),
                            span(count.to_string())
                                .text_color(Color::BLUE)
                                .font_size(18)
                                .absolute()
                                .top(1)
                                .right(1)
                                .bundle(pickable),
                        ))
                }
                let events = ()
                    .on_pointer_drag(
                        move |e: Res<ListenerInputPointerDrag>,
                              mut dargging: ResMut<DraggingInventoryItem>| {
                            dargging.delta += e.delta;
                        },
                    )
                    .on_pointer_drag_end(move |mut dragging: ResMut<DraggingInventoryItem>, mut is_dragging: ResMut<InventoryDraggingStatus>| {
                        dragging.reset();
                        *is_dragging = NoDragging;
                    })
                    .on_pointer_drag_start({
                        let item = item.clone();
                        move |world: &mut World| {
                            *world.resource_mut::<InventoryDraggingStatus>() = InventoryDraggingStatus::FullDrag;
                            let e =world.resource::<ListenerInputPointerDragStart>();
                            let parent = world.get_parent(&e.listener()).unwrap();
                            let view_key =world.spawn_view(
                                into_view(item_view(item.clone(),true)
                                    .z(1)
                                    .member(
                                        x_res(move |dragging: &DraggingInventoryItem| {
                                            ().left(dragging.delta.x).top(dragging.delta.y)
                                        })
                                    )
                                ),
                                 move |_| parent
                            );
                            *world.resource_mut::<DraggingInventoryItem>() = DraggingInventoryItem {
                                delta: Vec2::default(),
                                index,
                                view_key: Some(world.remove_on_drop(view_key))
                            };
                        }
                    })

                    .on_pointer_over({
                        let item = item.clone();
                        move |mut hovered_inventory_item: ResMut<HoveredInventoryItem>| {
                            hovered_inventory_item.item = Some((index,item.clone()));
                        }
                    })
                    .on_pointer_out(move |mut hovered_inventory_item: ResMut<HoveredInventoryItem>| {
                        hovered_inventory_item.item = None;
                    })
                    ;
                into_view(item_view(item,false)
                    .visibility(x_res(move |dragging_status: &InventoryDraggingStatus| {
                        let dragging_status = dragging_status.clone();
                        member_builder(move |ctx:ViewMemberCtx<BevyRenderer>,_| {
                            if ctx.world.resource::<DraggingInventoryItem>().index == index && dragging_status == InventoryDraggingStatus::FullDrag  {
                                Visibility::Hidden
                            } else {
                                Visibility::Inherited
                            }
                        })
                    }))
                    .member(events))
                    .either_left()
            } else {
                ().either_right()
            }
        }))
    }
}
