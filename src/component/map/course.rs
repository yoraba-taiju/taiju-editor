use bevy::prelude::*;

use crate::model;

pub mod current_frame;
pub mod key_frame;
pub mod route;

/******************************************************************************
 ** Course
 ******************************************************************************/

 #[derive(Debug, Default)]
 pub struct CourseComponent {
   pub length: usize,
 }
 
 #[derive(Debug, Default, Bundle)]
 pub struct CourseBundle {
   pub course_componense: CourseComponent,
   pub global_transform: GlobalTransform,
   pub transform: Transform,
 }
 
 impl CourseBundle {
   pub fn new(length: usize) -> Self {
     Self {
       course_componense: CourseComponent {
         length,
       },
       ..Default::default()
     }
   }
 }

 pub fn insert(
  commands: &mut ChildBuilder,
  mut meshes: &mut ResMut<Assets<Mesh>>,
  course: &model::map::Course,
) -> Entity {
  commands
    .spawn_bundle(CourseBundle::new(course.length))
    .with_children(|builder|{
      for (at, pos) in &course.keyframes {
        key_frame::insert(builder, *at, *pos);
      }
      route::insert(builder, &mut meshes);
    }).id()
}
