use taiju::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scape {
  pub scape_kind: taiju::chapter::components::scape::ScapeKind,
  pub pos: Vec2,
  pub size: Vec2,
  pub hit_areas: Vec<(Vec2, Vec2)>,
}
