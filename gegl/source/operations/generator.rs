//! Macro-based [`crate::GeglOperation`] struct generator.

/// A macro to generate a struct that implements [`crate::GeglOperation`].
#[macro_export]
macro_rules! gegl_operation {
  (
    struct_name: $struct_name:ident,
    gegl_name: $gegl_name:expr,
    append_crop: $append_crop:expr,
    values: ($($key:ident: $key_type:ty, $key_default:expr, $key_doc:expr),*,),
  ) => {
    #[doc = concat!(" The `gegl:", $gegl_name, "` operation.")]
    #[derive(Debug)]
    pub struct $struct_name {
      $(
        #[doc = concat!(" ", $key_doc)]
        pub $key: $key_type,
      )*
    }

    impl Default for $struct_name {
      fn default() -> $struct_name {
        $struct_name {
          $($key: $key_default,)*
        }
      }
    }

    impl $crate::GeglOperation for $struct_name {
      fn append_crop_operation(&self) -> bool {
        $append_crop
      }

      fn name(&self) -> &'static str {
        concat!("gegl:", $gegl_name)
      }

      fn values(&self) -> $crate::GeglData {
        $crate::indexmap! {
          $(stringify!($key) => self.$key.to_string(),)*
        }
      }
    }
  };
}
