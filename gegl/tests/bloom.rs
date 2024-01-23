use {
  gegl::{Bloom, GeglOperation},
  insta::assert_debug_snapshot,
};

#[test]
fn test_bloom_graph() {
  let mut graphs = vec![];

  let op = Bloom {
    limit_exposure: true,
    radius: 100_f64,
    softness: 100_f64,
    ..Default::default()
  };

  let default = Bloom::default();

  graphs.push(("non-default: exclude defaults", op.graph(false)));
  graphs.push(("non-default: include defaults", op.graph(true)));
  graphs.push(("default: exclude defaults", default.graph(false)));
  graphs.push(("default: include defaults", default.graph(true)));

  assert_debug_snapshot!("graphs", graphs);
}
