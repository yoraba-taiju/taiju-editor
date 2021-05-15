use bevy::{prelude::*, reflect::TypeUuid, render::{mesh::{self, Indices, shape}, pipeline::{PipelineDescriptor, RenderPipeline}, render_graph::{base, RenderGraph, RenderResourcesNode}, renderer::RenderResources, shader::{ShaderStage, ShaderStages}}};

const VERT_SHADER: &'static str = r###"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Color;

layout(location = 0) out vec3 v_Color;

layout(set = 0, binding = 0) uniform CameraViewProj {
  mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
  mat4 Model;
};

void main() {
  v_Color = Vertex_Color;
  gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
"###;

const FRAG_SHADER: &'static str = r###"
#version 450

layout(location = 0) in vec3 v_Color;

layout(location = 0) out vec4 o_Target;

void main() {
  o_Target = vec4(v_Color, 1.0);
}
"###;

pub const TEST_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 6657636463977685992);
#[derive(Debug)]
pub struct TestPlugin;

impl Plugin for TestPlugin {
  fn build(&self, app: &mut AppBuilder) {
    let world_cell = app.world_mut().cell();
    let mut pipelines = world_cell
      .get_resource_mut::<Assets<PipelineDescriptor>>()
      .unwrap();
    let mut shaders = world_cell.get_resource_mut::<Assets<Shader>>().unwrap();
    let mut render_graph = world_cell.get_resource_mut::<RenderGraph>().unwrap();

    // Create a new shader pipeline.
    let pipeline = PipelineDescriptor::default_config(ShaderStages {
      vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERT_SHADER)),
      fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAG_SHADER))),
    });
    pipelines.set_untracked(TEST_PIPELINE_HANDLE,pipeline);

  }
}

pub fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
) {

  let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::LineStrip);
  mesh.set_indices(Some(Indices::U32(vec![0, 2, 1, 0, 3, 2])));
  let vetexes: Vec<[f32; 3]> = vec![
    [-2.5, -2.5, 0.0],
    [-2.5, 2.5, 0.0],
    [2.5, 2.5, 0.0],
    [2.5, -2.5, 0.0],
  ];
  let uvs: Vec<[f32; 3]> = vec![
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 1.0],
  ];
  mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vetexes);
  mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, uvs);
  println!("{:?}", mesh);
  //mesh = Mesh::from(shape::Quad::new(Vec2::new(5.0, 5.0)));
  //println!("{:?}", mesh);

  // Spawn a quad and insert the `TimeComponent`.
  commands
      .spawn_bundle(MeshBundle {
          mesh: meshes.add(mesh),
          render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            TEST_PIPELINE_HANDLE.typed(),
          )]),
          transform: Transform::from_xyz(0.0, 0.0, 0.0),
          ..Default::default()
      });

  // Spawn a camera.
  commands.spawn_bundle(PerspectiveCameraBundle {
      transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..Default::default()
  });
}
