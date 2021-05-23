use bevy::{prelude::*, reflect::TypeUuid};

/******************************************************************************
 ** KeyFrame
 ******************************************************************************/

 #[derive(Debug, Default)]
 pub struct KeyFrameComponent {
   pub at: usize,
 }
 
 #[derive(Debug, Default, Bundle)]
 pub struct KeyFrameBundle {
   pub key_frame_component: KeyFrameComponent,
   pub global_transform: GlobalTransform,
   pub transform: Transform,
 }
 
 
 impl KeyFrameBundle {
   pub fn new(at: usize, pos: Vec2) -> Self {
     Self {
       key_frame_component: KeyFrameComponent {
         at,
       },
       transform: Transform::from_xyz(pos.x, pos.y, 0.0),
       ..Default::default()
     }
   }
 }
 
 pub fn insert(
  commands: &mut ChildBuilder,
  at: usize,
  pos: Vec2,
) -> Entity {
  commands
    .spawn_bundle(KeyFrameBundle::new(at, pos))
    .with_children(|builder| {
       builder.spawn_bundle(SpriteBundle {
         material: MATERIAL_HANDLE.typed(),
         sprite: Sprite {
           size: Vec2::new(128.0, 128.0),
           resize_mode: SpriteResizeMode::Manual,
           ..Default::default()
         },
         ..Default::default()
       });
    })
    .id()
}

/******************************************************************************
 ** Init
 ******************************************************************************/

const MATERIAL_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(ColorMaterial::TYPE_UUID, 9202969657271345152);

pub fn init(app: &mut AppBuilder) {
  let world = app.world_mut();

  let asset_server: &AssetServer = world.get_resource().unwrap();
  let texture: Handle<Texture> = asset_server.load("component/map/course/key_frame.png");

  let mut color_material: Mut<Assets<ColorMaterial>> = world.get_resource_mut().unwrap();
  color_material.set_untracked(MATERIAL_HANDLE, ColorMaterial::texture(texture));
}

