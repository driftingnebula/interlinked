//! The CLI logic.

use {
  crate::{all_projects, utilities::shell_command, CliArgs, Parser},
  color_eyre::{eyre::OptionExt, Result},
  gegl::GeglOperation,
  std::{
    fs::{create_dir_all, write},
    path::Path,
    time::Instant,
  },
};

/// Run the CLI.
pub fn run() -> Result<()> {
  let args = CliArgs::parse();

  let projects = if let Some(filter) = args.filter {
    all_projects()
      .into_iter()
      .filter(|project| project.name.starts_with(&filter))
      .collect::<Vec<_>>()
  } else {
    all_projects().into_iter().collect()
  };

  let base_out_dir = Path::new("output/");
  for project in projects {
    let start = Instant::now();
    let (width, height) = project.resolution;
    let out_dir = base_out_dir.join(&project.name);
    create_dir_all(&out_dir)?;

    println!(
      "→ {} ({}x{}, {} base operations ({} including crops))",
      project.name,
      width,
      height,
      project.operations.len(),
      project
        .operations
        .iter()
        .map(|op| if op.append_crop_operation() { 2 } else { 1 })
        .sum::<i32>(),
    );

    let graph = project
      .operations
      .into_iter()
      .flat_map(|op| {
        let mut graph = vec![op.graph(args.include_defaults)];
        if op.append_crop_operation() {
          graph.push(
            gegl::Crop::default()
              .with_height(height as f64)
              .with_width(width as f64)
              .graph(args.include_defaults),
          )
        }

        graph
      })
      .collect::<Vec<_>>();

    let pretty_graph = graph
      .iter()
      .map(|params| {
        params
          .iter()
          .map(|param| {
            if param.starts_with("gegl:") {
              param.to_string()
            } else {
              format!("  {}", param)
            }
          })
          .collect::<Vec<_>>()
          .join("\n")
      })
      .collect::<Vec<_>>()
      .join("\n\n");

    let graph_file = out_dir.join(format!("{}.txt", project.name));
    write(graph_file, pretty_graph)?;

    if args.no_render {
      let end = Instant::now();
      println!("← {:?} (not rendered)", end - start);
      continue;
    }

    let out_file = out_dir.join(format!("{}.png", project.name));
    let out_file = out_file
      .to_str()
      .ok_or_eyre("Failed to convert out_file to str")?;

    let input = if project.create_input_image {
      shell_command(format!(
        "convert -size {}x{} xc:white {}",
        width, height, out_file
      ))?;
      format!("-i {}", out_file)
    } else {
      String::new()
    };

    let graph = graph
      .into_iter()
      .map(|op| op.join(" "))
      .collect::<Vec<_>>()
      .join(" ");
    shell_command(format!("gegl {} -o {} -- {}", input, out_file, graph))?;

    if project.turn_off_alpha {
      shell_command(format!("convert {0} -alpha Off {0}", out_file))?;
    }

    let end = Instant::now();
    println!("← {:?}", end - start);
  }

  Ok(())
}
