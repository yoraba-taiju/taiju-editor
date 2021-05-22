use bevy::prelude::*;

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
    .id()
}