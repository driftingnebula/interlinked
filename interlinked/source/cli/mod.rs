//! The [`clap`] command-line definitions.

mod run;

pub use {clap::Parser, run::*};

/// The Interlinked CLI.
#[derive(Debug, Parser)]
pub struct CliArgs {
  /// Only render projects starting with the filter.
  #[clap(short, long)]
  pub filter: Option<String>,

  /// Include default values in the GEGL graphs.
  #[clap(long, default_value = "false")]
  pub include_defaults: bool,

  /// Don't render any images.
  #[clap(long, default_value = "false")]
  pub no_render: bool,
}
