//! # GEGL
//!
//! > **Ad-hoc GEGL data structure library for Rust.**

mod operations;

pub use operations::*;

pub use indexmap::indexmap;

/// Type alias for a [`indexmap::IndexMap`] with static [`str`] keys and
/// [`String`] values. The reason for [`mod@indexmap`] is to preserve insertion
/// order for simpler testing.
pub type GeglData = indexmap::IndexMap<&'static str, String>;

/// The [`GeglOperation`] trait defines a set of common functions for the
/// individual operations to implement so they can be used with the GEGL CLI.
pub trait GeglOperation: Default + std::fmt::Debug {
  /// Some GEGL operations will run infinitely unless you limit the buffer in
  /// some way, so all operations must indicate whether or not they should be
  /// followed by a crop operation.
  fn append_crop_operation(&self) -> bool;

  /// Creates the parameters for the graph to be used with the GEGL CLI.
  fn graph(&self, include_default_values: bool) -> Vec<String> {
    let mut graph = vec![self.name().to_string()];
    let defaults = Self::default().values();

    for (key, value) in self.values() {
      if !include_default_values && defaults.get(key) == Some(&value) {
        continue;
      }

      let key = key.replace('_', "-");
      graph.push(format!("{key}={value}"));
    }

    graph
  }

  /// Returns the name of the operation, starting with `gegl:`.
  fn name(&self) -> &'static str;

  /// Returns the set of configured values for this operation.
  fn values(&self) -> GeglData;
}
