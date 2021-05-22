use bevy::prelude::*;

use crate::{bevy_render_primitive::TriangleListBuilder, component::map::course::route::Route};

pub fn update_on_changed(
  mut commands: Commands,
  mut route: Query<(Entity, &mut Route)>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  let (route_id, mut route) = if let Ok(inner) = route.single_mut() {
    inner
  } else {
    return;
  };
  let routes = if let Some(ref routes) = route.updated_route {
    routes
  } else {
    return;
  };
  commands.entity(route.child).despawn_recursive();
  let mut b =
      TriangleListBuilder::new()
        .set_default_color(Color::rgba(1.0, 1.0, 1.0, 0.5));
  let mut start_idx: u32 = 0;
  for i in 0..(routes.len() - 1) {
    let v1 = routes[i];
    let v2 = routes[i+1];
    b = b.push_vertex(Vec3::new(v1.x, v1.y - 5.0, 0.0))
         .push_vertex(Vec3::new(v1.x, v1.y + 5.0, 0.0))
         .push_vertex(Vec3::new(v2.x, v2.y + 5.0, 0.0))
         .push_vertex(Vec3::new(v2.x, v1.y - 5.0, 0.0))
         .push_triangle(start_idx+0, start_idx+1, start_idx+2)
         .push_triangle(start_idx+0, start_idx+2, start_idx+3);
    start_idx += 4;
  }
  commands.entity(route_id).with_children(|commands| {
    route.child = commands.spawn_bundle(b.build(&mut *meshes)).id();
    println!("Next child: {:?}", route.child);
  });
  route.updated_route = None;
}