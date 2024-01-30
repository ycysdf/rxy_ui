use bevy_mod_picking::prelude::*;

macro_rules! impl_element_events {
    ($($name:ident = $event_type:ty;)*) => {
        $(
           pub type $name = $event_type;
           paste::paste!{
               pub type [<ListenerInput $name>] = ListenerInput<$name>;
           }
        )*

        impl<T> ElementEvents for T
        where
            T: rxy_core::MemberOwner<$crate::BevyRenderer>+Sized {}

        pub trait ElementEvents: rxy_core::MemberOwner<$crate::BevyRenderer>+Sized
        {
            fn on<EE: EntityEvent, Marker>(
                self,
                callback: impl bevy_ecs::prelude::IntoSystem<(), (), Marker>,
            ) -> Self::AddMember<$crate::XBundle<On<EE>>> {
                use bevy_mod_picking::prelude::*;
                self.member($crate::XBundle(On::<EE>::run(callback)))
            }
            $(
                paste::paste!{
                    fn [<on_ $name:snake>]<Marker>(
                        self,
                        callback: impl bevy_ecs::prelude::IntoSystem<(), (), Marker>,
                    ) -> Self::AddMember<$crate::XBundle<On<$name>>> {
                        self.on::<$name, Marker>(callback)
                    }
                }
            )*
        }

    };
}
impl_element_events!(
    PointerOver = Pointer<Over>;
    PointerOut = Pointer<Out>;
    PointerDown = Pointer<Down>;
    PointerUp = Pointer<Up>;
    PointerClick = Pointer<Click>;
    PointerMove = Pointer<Move>;
    PointerDragStart = Pointer<DragStart>;
    PointerDrag = Pointer<Drag>;
    PointerDragEnd = Pointer<DragEnd>;
    PointerDragEnter = Pointer<DragEnter>;
    PointerDragOver = Pointer<DragOver>;
    PointerDragLeave = Pointer<DragLeave>;
    PointerDrop = Pointer<Drop>;
);
