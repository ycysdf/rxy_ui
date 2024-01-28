use crate::style_sheet_items::StyleSheetItems;
use crate::{
    AppliedStyleSheet, StyleEntityMutExt, StyleSheetDefinition, StyleSheetsInfo, StyleWorldExt,
};
use bevy_ecs::all_tuples;
use bevy_ecs::system::Resource;
use bevy_ecs::world::FromWorld;
use core::marker::PhantomData;
use rxy_bevy::BevyRenderer;
use rxy_core::Renderer;
use rxy_style::{StyleSheetCtx, StyleSheetOwner};
use std::any::TypeId;
use std::iter::once;

pub trait StyleSheets<R>: Send + 'static
where
    R: Renderer,
{
    fn style_sheets(
        self,
        ctx: StyleSheetCtx<R>,
    ) -> (
        impl Iterator<Item = AppliedStyleSheet> + Send + 'static,
        StyleSheetsInfo,
    );
}

pub struct XRes<T, M>(T, PhantomData<M>);

pub fn res<F, Res, T>(f: F) -> XRes<F, Res>
where
    F: Fn(&Res) -> T + Send + 'static,
    Res: Resource,
    T: StyleSheets<BevyRenderer>,
{
    XRes(f, Default::default())
}

impl<F, Res, T> StyleSheets<BevyRenderer> for XRes<F, Res>
where
    F: Fn(&Res) -> T + Send + 'static,
    Res: Resource + FromWorld,
    T: StyleSheets<BevyRenderer>,
{
    fn style_sheets(
        self,
        ctx: StyleSheetCtx<BevyRenderer>,
    ) -> (
        impl Iterator<Item = AppliedStyleSheet> + Send + 'static,
        StyleSheetsInfo,
    ) {
        let f = self.0;
        if !ctx.world.contains_resource::<Res>() {
            let res = Res::from_world(ctx.world);
            ctx.world.insert_resource(res);
        }
        let res = ctx.world.resource::<Res>();
        f(res).style_sheets(ctx)
    }
}

impl<T> StyleSheets<BevyRenderer> for StyleSheetOwner<T>
where
    T: StyleSheetItems<BevyRenderer>,
{
    fn style_sheets(
        self,
        ctx: StyleSheetCtx<BevyRenderer>,
    ) -> (
        impl Iterator<Item = AppliedStyleSheet> + Send + 'static,
        StyleSheetsInfo,
    ) {
        (
            once(AppliedStyleSheet::Inline(StyleSheetDefinition {
                interaction: self.0,
                items: T::iter(self.1, ctx).collect(),
            })),
            StyleSheetsInfo {
                inline_style_sheet_count: 1,
                shared_style_sheet_count: 0,
            },
        )
    }
}

// impl<T> StyleSheets<BevyRenderer> for BevyWrapper<T>
// where
//     T: TypedStyleLabel,
// {
//     fn style_sheets(
//         self,
//         ctx: StyleSheetCtx<BevyRenderer>,
//     ) -> (
//         impl Iterator<Item = AppliedStyleSheet> + Send + 'static,
//         StyleSheetsInfo,
//     ) {
//         todo!()
//     }
// }

pub fn typed_shared_style_sheets(
    type_id: TypeId,
    ctx: StyleSheetCtx<BevyRenderer>,
) -> (
    impl Iterator<Item = AppliedStyleSheet> + Send + 'static,
    StyleSheetsInfo,
) {
    let entity = ctx.world.get_typed_entity(type_id).unwrap();
    {
        let mut entity_world_mut = ctx.world.entity_mut(entity);
        let shared_style_sheets = entity_world_mut.get_shared_style_state().unwrap();
        shared_style_sheets.add_subscriber(ctx.node_id);
    }
    let mut entity_world_mut = ctx.world.entity_mut(entity);

    let style_sheets_state = entity_world_mut.get_style_sheets_state().unwrap();
    (
        style_sheets_state.apply_as_shared(entity, ctx.shared_style_sheet_index),
        style_sheets_state.style_sheets_info(),
    )
}

impl StyleSheets<BevyRenderer> for TypeId {
    fn style_sheets(
        self,
        ctx: StyleSheetCtx<BevyRenderer>,
    ) -> (
        impl Iterator<Item = AppliedStyleSheet> + Send + 'static,
        StyleSheetsInfo,
    ) {
        typed_shared_style_sheets(self, ctx)
    }
}

// impl<R, LSS, RSS> StyleSheets<R> for Either<LSS, RSS>
// where
//     R: Renderer,
//     LSS: StyleSheets<R>,
//     RSS: StyleSheets<R>,
// {
//     fn style_sheets(
//         self,
//         ctx: StyleSheetCtx<R>,
//     ) -> (
//         impl Iterator<Item = AppliedStyleSheet> + Send + 'static,
//         StyleSheetsInfo,
//     ) {
//         match self {
//             Either::Left(l) => {
//                 let x = l.style_sheets(ctx);
//                 (x.0.either_left(), x.1)
//             }
//             Either::Right(r) => {
//                 let x = r.style_sheets(ctx);
//                 (x.0.either_right(), x.1)
//             }
//         }
//     }
// }

macro_rules! impl_style_sheets_for_tuple {
    ($($t:ident),*) => {
        #[allow(non_snake_case)]
        impl<R, $($t),*> StyleSheets<R> for ($($t,)*)
        where
            R: Renderer,
            $($t: StyleSheets<R>),*
        {
            #[inline]
            fn style_sheets(
                self,
                ctx: StyleSheetCtx<R>,
            ) -> (impl Iterator<Item = AppliedStyleSheet> + Send + 'static,StyleSheetsInfo) {
                let ($($t,)*) = self;
                let r = core::iter::empty();
                let mut _r_info  = StyleSheetsInfo{
                    inline_style_sheet_count: ctx.inline_style_sheet_index,
                    shared_style_sheet_count: ctx.shared_style_sheet_index,
                };
                $(
                    let (style_sheets,info) = $t.style_sheets(StyleSheetCtx {
                        inline_style_sheet_index: _r_info.inline_style_sheet_count,
                        shared_style_sheet_index: _r_info.shared_style_sheet_count,
                        // world: &mut *ctx.world,
                        world: unsafe {&mut *(ctx.world as *mut _)},
                        node_id: ctx.node_id.clone(),
                    });
                    _r_info += info;
                    let r = r.chain(style_sheets);
                )*
                (r,_r_info)
            }
        }
    };
}
all_tuples!(impl_style_sheets_for_tuple, 0, 12, T);
