#[allow(unused_imports)]
use paste::paste;

#[macro_export]
macro_rules! define_attr_get_fn {
    ($renderer:ident) => {
        pub fn get_attr_by_index(index: AttrIndex) -> &'static dyn ElementAttrUntyped<$renderer> {
            let mut index = index as usize;
            for attrs in ALL_ATTRS {
                if index < attrs.len() {
                    return attrs[index];
                }
                index -= attrs.len();
            }
            unreachable!();
        }
    };
}

#[macro_export]
macro_rules! impl_index_for_tys {
    (
        index_start = $index_start:expr;
        types = [
            $($ty:ty)*
        ]
    ) => {
        count_macro::count! {
            $(
            impl $crate::HasIndex for $ty{
               const INDEX: $crate::AttrIndex = $index_start+_int_;
            }
            )*
        }
    };
    ($($ty:ty)*) => {
        $crate::impl_index_for_tys! {
            index_start = 0;
            types = [
                $($ty)*
            ]
        }
    };
}


#[macro_export]
macro_rules! attrs_fn_define {
    (
        renderer = $renderer:ty;
        name = $name:ident;
        $(element = $element:ident;)?
        attrs = [
            $({
                name = $attr_name:tt,
                ty = $attr_ty:ty
            })*
        ]
    ) => {
        paste! {
            pub trait [<$name:camel ViewBuilder>]: $crate::MemberOwner<$renderer> + Sized {
                $(
                    #[inline]
                    fn [<$attr_name:snake>]<T>(self, value: impl $crate::XNest<MapInner<$crate::MapToAttrMarker<$attr_ty>> = T>) -> Self::AddMember<T>
                    where
                        T: $crate::ElementAttrMember<$renderer, $attr_ty>,
                        (Self::VM, T): $crate::ViewMember<$renderer>
                    {
                        self.member(value.map_inner::<$crate::MapToAttrMarker<$attr_ty>>())
                    }
                )*
            }

            impl<T> [<$name:camel ViewBuilder>] for T
                where T: $crate::MemberOwner<$renderer$(,E=$element)?>
            {}
        }
    };
    (
        renderer = $renderer:ty;
        element = $element:ident;
        attrs = [
            $({
                name = $attr_name:tt,
                ty = $attr_ty:ty
            })*
        ]
    ) => {
        paste!{
            attrs_fn_define! {
                renderer = $renderer;
                name = [<$element:camel AttrsViewBuilder>];
                element = $element;
                attrs = [
                    $({
                        name = $attr_name,
                        ty = $attr_ty
                    })*
                ]
            }
        }
    };
    (
        renderer = $renderer:ty;
        attrs = [
            $({
                name = $attr_name:tt,
                ty = $attr_ty:ty
            })*
        ]
    ) => {
        attrs_fn_define! {
            renderer = $renderer;
            name = CommonAttrs;
            attrs = [
                $({
                    name = $attr_name,
                    ty = $attr_ty
                })*
            ]
        }
    };
}

#[macro_export]
macro_rules! impl_attrs_for_element_type {
    (
        renderer = $renderer:ty;
        element = $element:ident;
        attrs = [
            $($attr:ident)*
        ]
    ) => {
        impl $crate::ElementTypeAttrs<$renderer> for $element {
            const ATTRS: &'static [&'static dyn $crate::ElementAttrUntyped<$renderer>] = &[
                $(
                    &all_attrs::$attr,
                )*
            ];
        }
    };
}