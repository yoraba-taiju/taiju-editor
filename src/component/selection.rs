use bevy::{prelude::*, reflect::TypeUuid};
/******************************************************************************
 ** Init(Plugin)
 ******************************************************************************/
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
  fn build(&self, app: &mut AppBuilder) {
    init(app);
  }
}

/******************************************************************************
 ** Selectable
 ******************************************************************************/
#[derive(Debug)]
pub struct SelectableComponent {
  pub size: Vec2,
}

impl SelectableComponent {
  pub fn new(size: Vec2) -> Self {
    Self {
      size,
    }
  }
}

/******************************************************************************
 ** Selected
 ******************************************************************************/

#[derive(Debug)]
pub struct SelectedComponent;

/******************************************************************************
 ** Init
 ******************************************************************************/

pub const SELECTED_COLOR_MATERIAL_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(ColorMaterial::TYPE_UUID, 5592548940752013312);

pub fn init(app: &mut AppBuilder) {
  let world = app.world_mut();

  let color = Color::rgba(1.0, 0.5, 0.5, 0.5);

  let mut color_material: Mut<Assets<ColorMaterial>> = world.get_resource_mut().unwrap();
  color_material.set_untracked(SELECTED_COLOR_MATERIAL_HANDLE, ColorMaterial::color(color));
}

