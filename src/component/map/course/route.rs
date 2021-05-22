use std::ops::DerefMut;

use bevy::prelude::*;

use crate::{bevy_render_primitive::TriangleListBuilder, model};

pub struct Route {
  pub updated_route: Option<Vec<Vec2>>,
  pub child: Entity,
}

#[derive(Bundle)]
pub struct RouteBundle {
  pub route: Route,
  pub global_transform: GlobalTransform,
  pub transform: Transform,
}

pub fn insert(
  commands: &mut ChildBuilder,
  meshes: &mut ResMut<Assets<Mesh>>,
) -> Entity {
  let mut spawner = commands.spawn();
  let mut route = Route {
    updated_route: None,
    child: Entity::new(0),
  };
  spawner.with_children(|builder| {
    let b = TriangleListBuilder::new();
    route.child = builder.spawn_bundle(b.build(&mut **meshes)).id();
  });
  spawner.insert_bundle(RouteBundle {
    route,
    global_transform: GlobalTransform::identity(),
    transform: Transform::identity(),
  });
  spawner.id()
}
