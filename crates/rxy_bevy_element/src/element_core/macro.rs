#[macro_export]
macro_rules! attrs_trait_define {
   ($name:ident;$index_start:expr;$($attr:ident)*) => {
        count_macro::count! {
            $(
            impl $crate::element_core::HasIndex for $crate::all_attrs::$attr{
               const INDEX: $crate::AttrIndex = $index_start+_int_;
            }
            )*
        }
        #[allow(non_upper_case_globals)]
        #[allow(non_camel_case_types)]
        pub trait $name {
            const ATTRS: &'static [&'static dyn $crate::ElementUnitAttrUntyped] = &[
                $(&$crate::all_attrs::$attr,)*
            ];
        }
   };
}

#[macro_export]
macro_rules! define_elements {
   (
      $(
         $(#[$m_attr:meta])*
            $name:ident {
            [attrs]
            $($attr:ident)*
         }
     )*
   ) => {

      impl $crate::BevyDioxusAppExt for bevy_app::App{
         fn register_elements_type(&mut self)-> &mut Self{
            self
                $(
                   .register_type::<$crate::elements::$name>()
                )*
         }
      }

     pub fn try_get_element_type(name: &str) -> Option<&'static dyn ElementTypeUnTyped> {
         match name {
            $(
               stringify!($name) => Some(&$crate::elements::$name),
                )*
                _ => None,
            }
      }

     pub fn try_get_element_type_by_type_id(type_id: std::any::TypeId) -> Option<&'static dyn ElementTypeUnTyped> {
        // static MAP: HashMap<TypeId,&'static dyn ElementTypeUnTyped> = {
        //   let result =HashMap::new();
        //   result
        // };
         match type_id {
            $(
               n if n == std::any::TypeId::of::<$crate::elements::$name>() => Some(&$crate::elements::$name),
            )*
                _ => None,
            }
      }

      pub fn get_element_type(name: &str) -> &'static dyn ElementTypeUnTyped {
         try_get_element_type(name).unwrap_or_else(|| panic!("No Found ElementType by {:#?}", name))
      }

      pub fn get_element_type_by_type_id(type_id: std::any::TypeId) -> &'static dyn ElementTypeUnTyped {
         try_get_element_type_by_type_id(type_id).unwrap_or_else(|| panic!("No Found ElementType by {:#?}", type_id))
      }

      $(
        $crate::define_element!(
            $(#[$m_attr])*
            $name {
               [attrs]
               $($attr)*
            }
         );
      )*
   }
}

#[macro_export]
macro_rules! define_element {
    (
         $(#[$m_attr:meta])*
         $name:ident {
            [attrs]
            $($attr:ident)*
         }
    ) => {
       #[allow(non_camel_case_types)]
        $( #[$m_attr] )*
        pub struct $name;

        paste::paste!{
            $crate::attrs_trait_define!([<$name:camel Attrs>];<$name as $crate::CommonAttrs>::ATTRS.len() as u8;
                $($attr),*
            );

            impl $crate::CommonAttrs for $name {}
            impl [<$name:camel Attrs>] for $name {}

            impl $crate::ElementTypeBase for $name {
                const TAG_NAME: &'static str = stringify!($name);
                const ATTRS: &'static [&'static [&'static dyn $crate::ElementUnitAttrUntyped]] = &[
                    <Self as $crate::CommonAttrs>::ATTRS,
                    <Self as $crate::elements::[<$name:camel Attrs>]>::ATTRS,
                ];
            }
        }
   }
}
