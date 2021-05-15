use bevy::prelude::*;

mod line;
pub use line::LineListBuilder;
pub use line::LineStripBuilder;

pub struct PrimitivePlugin;

impl Plugin for PrimitivePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_plugin(line::LinePlugin);
  }
}
