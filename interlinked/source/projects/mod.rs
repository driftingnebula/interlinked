//! All [`Project`]s created with Interlinked.

use crate::Project;

pub mod year_2022;

/// Get all [`Project`]s in a single [`Vec`].
pub fn all_projects() -> Vec<Project> {
  vec![
    year_2022::day_2022_03_06(),
    year_2022::day_2022_03_07(),
    year_2022::day_2022_03_08(),
    year_2022::day_2022_03_09(),
    year_2022::day_2022_03_10(),
    year_2022::day_2022_03_11(),
    year_2022::day_2022_03_12(),
    year_2022::day_2022_03_13(),
    year_2022::day_2022_03_14(),
    year_2022::day_2022_03_15(),
  ]
}
