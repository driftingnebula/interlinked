//! All [`Project`]s created with Interlinked.

use crate::Project;

pub mod year_2022;

/// Get all [`Project`]s in a single [`Vec`].
pub fn all_projects() -> Vec<Project> {
  vec![year_2022::day_2022_03_06(), year_2022::day_2022_03_07()]
}
