//! Macro-based [`crate::GeglOperation`] struct generator.

/// A macro to generate a struct that implements [`crate::GeglOperation`].
#[macro_export]
macro_rules! gegl_operation {
  (
    struct_name: $struct_name:ident,
    gegl_name: $gegl_name:expr,
    append_crop: $append_crop:expr,
    values: (
      $(
        $(#[$key_meta:meta])*
        $key:ident: $key_type:ty, $key_default:expr
      ),*,
    ),
  ) => {
    #[doc = concat!(" The `gegl:", $gegl_name, "` operation.")]
    #[derive(Debug)]
    pub struct $struct_name {
      $(
        $(#[$key_meta])*
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

      fn default_values(&self) -> $crate::GeglData {
        Self::default().values()
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

/// A macro to generate an enum that implements [`std::fmt::Display`].
#[macro_export]
macro_rules! gegl_enum {
  (
    $(#[$enum_meta:meta])*
    $enum_name:ident,
    $($key:ident => $value:expr),*,
  ) => {
    $(#[$enum_meta])*
    #[derive(Debug)]
    pub enum $enum_name {
      $(
        #[doc = concat!("The [`", stringify!($enum_name), "`] `", $value, "`." )]
        $key,
      )*
    }

    impl std::fmt::Display for $enum_name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
          $($enum_name::$key => $value,)*
        };

        write!(f, "{display}")
      }
    }
  };
}
