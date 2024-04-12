use std::iter::once;

use crate::style::{StyleItemValue, StyleSheetCtx};
use crate::utils::all_tuples;
use crate::{smallbox, ElementAttr, ElementAttrType, Renderer};

pub trait StyleSheetItems<R>: Send + 'static
where
   R: Renderer,
{
   fn iter(self, ctx: StyleSheetCtx<R>) -> impl Iterator<Item = StyleItemValue> + 'static;
}

impl<R, EA> StyleSheetItems<R> for ElementAttr<R, EA>
where
   R: Renderer,
   EA: ElementAttrType<R>,
{
   #[inline]
   fn iter(self, _ctx: StyleSheetCtx<R>) -> impl Iterator<Item = StyleItemValue> + 'static {
      once(StyleItemValue {
         attr_id: EA::INDEX,
         value: smallbox!(self.0),
      })
   }
}

macro_rules! impl_style_sheet_items_for_tuple {
    ($($t:ident),*) => {
        #[allow(non_snake_case)]
        impl<R, $($t),*> StyleSheetItems<R> for ($($t,)*)
        where
            R: Renderer,
            $($t: StyleSheetItems<R>),*
        {
            #[inline]
            fn iter(
                self,
                _ctx: StyleSheetCtx<R>,
            ) -> impl Iterator<Item = StyleItemValue> + 'static {
                let ($($t,)*) = self;
                core::iter::empty()
                $(
                    .chain($t.iter(StyleSheetCtx{
                        inline_style_sheet_index: _ctx.inline_style_sheet_index,
                        shared_style_sheet_index: _ctx.shared_style_sheet_index,
                        world: unsafe {&mut *(_ctx.world as *mut _)},
                        node_id: _ctx.node_id.clone(),
                    }))
                )*
            }
        }
    };
}
all_tuples!(impl_style_sheet_items_for_tuple, 0, 4, T);
