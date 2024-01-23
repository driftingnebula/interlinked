use {
  gegl::{Cartoon, GeglOperation},
  insta::assert_debug_snapshot,
};

#[test]
fn test_cartoon_graph() {
  let mut graphs = vec![];

  let op = Cartoon {
    mask_radius: 0.1,
    ..Default::default()
  };

  let default = Cartoon::default();

  graphs.push(("non-default: exclude defaults", op.graph(false)));
  graphs.push(("non-default: include defaults", op.graph(true)));
  graphs.push(("default: exclude defaults", default.graph(false)));
  graphs.push(("default: include defaults", default.graph(true)));

  assert_debug_snapshot!("graphs", graphs);
}
