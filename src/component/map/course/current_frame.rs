use bevy::{prelude::*, reflect::TypeUuid};

/******************************************************************************
 ** CourseCurrentFrame
 ******************************************************************************/

 #[derive(Debug, Default)]
 pub struct CurrentFrameComponent {
  pub at: usize,
}

#[derive(Debug, Default, Bundle)]
pub struct CurrentFrameBundle {
  pub course_current_frame_component: CurrentFrameComponent,
  pub global_transform: GlobalTransform,
  pub transform: Transform,
}

impl CurrentFrameBundle {
  pub fn new(at: usize) -> Self {
    Self {
      course_current_frame_component: CurrentFrameComponent {
        at,
      },
      ..Default::default()
    }
  }
}

const COLOR_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(ColorMaterial::TYPE_UUID, 17941059021890322432);

pub fn init(app: &mut AppBuilder) {
  let world = app.world_mut();
  let mut color_materials: Mut<Assets<ColorMaterial>> = world.get_resource_mut().unwrap();
  color_materials.set_untracked(COLOR_HANDLE, ColorMaterial::color(Color::rgba(0.5, 0.5, 1.0, 0.5)));
}

pub fn insert(
  commands: &mut ChildBuilder,
) -> Entity {
  let color_material: Handle<ColorMaterial> = COLOR_HANDLE.typed();
  commands.spawn_bundle(CurrentFrameBundle::new(0))
  .with_children(|builder| {
    builder.spawn_bundle(SpriteBundle{
      sprite: Sprite {
        size: Vec2::new(1920.0, 10.0),
        ..Default::default()
      },
      material: color_material.clone(),
      transform: Transform::from_xyz(0.0, 1080.0/2.0, 0.0),
      ..Default::default()
    });
    builder.spawn_bundle(SpriteBundle{
      sprite: Sprite {
        size: Vec2::new(1920.0, 10.0),
        ..Default::default()
      },
      material: color_material.clone(),
      transform: Transform::from_xyz(0.0, -1080.0/2.0, 0.0),
      ..Default::default()
    });
    builder.spawn_bundle(SpriteBundle{
      sprite: Sprite {
        size: Vec2::new(10.0, 1080.0),
        ..Default::default()
      },
      material: color_material.clone(),
      transform: Transform::from_xyz(1920.0/2.0, 0.0, 0.0),
      ..Default::default()
    });
    builder.spawn_bundle(SpriteBundle{
      sprite: Sprite {
        size: Vec2::new(10.0, 1080.0),
        ..Default::default()
      },
      material: color_material.clone(),
      transform: Transform::from_xyz(-1920.0/2.0, 0.0, 0.0),
      ..Default::default()
    });
  })
  .id()
}