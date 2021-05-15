use bevy::prelude::*;

mod line;
pub use line::LineListBuilder;
pub use line::LineStripBuilder;

mod triangle;
pub use triangle::TriangleListBuilder;
pub use triangle::TriangleStripBuilder;

pub struct PrimitivePlugin;

impl Plugin for PrimitivePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_plugin(line::LinePlugin);
    app.add_plugin(triangle::TrianglePlugin);
  }
}
