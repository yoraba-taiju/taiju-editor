use bevy::prelude::*;
use taiju::chapter::prelude::*;
use crate::editor::*;

pub fn reload_map(
  mut map_state: ResMut<MapState>,
  map_query: Query<Entity, With<MapAnchor>>,
  mut commands: Commands,
  enemy_server: Res<EnemyServer>,
  mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
  if let Some(map) = map_state.handle.take() {
    map_state.map = Some(map);
  } else {
    return;
  }
  // remove old maps
  for map_id in map_query.iter() {
    commands.entity(map_id).despawn_recursive();
  }
  let map = map_state.map.as_ref().unwrap();
  let map_id =  commands
    .spawn()
    .insert(MapAnchor)
    .insert(Transform::identity())
    .insert(GlobalTransform::identity())
    .id();
  let mut map_entities = Vec::<Entity>::new();
  {
    for at in 0..=map.duration {
      let pos = map.pos[at];
      if let Some(enemies) = map.enemies.get(&(at as u32)) {
        {
          let id = commands.spawn().insert_bundle(SpriteBundle{
            sprite: Sprite::new(Vec2::new(1920.0, 1080.0)),
            material: color_materials.add(Color::rgba(0.5, 0.5, 1.0, 0.5).into()),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..Default::default()
          }).id();
          map_entities.push(id);
        }
        for desc in enemies.iter() {
          let mut spr = enemy_server.sprites[&desc.body].clone();
          spr.transform.translation.x = desc.position.x + pos.x;
          spr.transform.translation.y = desc.position.y + pos.y;
          let id = commands
            .spawn()
            .insert_bundle(spr)
            .insert(EnemyAnchor)
            .insert(desc.clone())
            .id();
          map_entities.push(id);
        }
      }
    }
  }
  {
    let id = commands.spawn().insert_bundle(SpriteBundle{
      sprite: Sprite::new(Vec2::new(1920.0, 1080.0)),
      material: color_materials.add(Color::rgba(1.0, 0.5, 0.5, 0.5).into()),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..Default::default()
    }).insert(FrameAnchor).id();
    map_entities.push(id);
  }

  commands.entity(map_id).push_children(&map_entities);
}

