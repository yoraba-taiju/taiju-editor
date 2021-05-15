use bevy::{prelude::*, reflect::TypeUuid, render::{mesh::{self, Indices, shape}, pipeline::{PipelineDescriptor, RenderPipeline}, render_graph::{base, RenderGraph, RenderResourcesNode}, renderer::RenderResources, shader::{ShaderStage, ShaderStages}}};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct TimeUniform {
  value: f32,
}

const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 0) out vec2 v_Uv;

layout(set = 0, binding = 0) uniform CameraViewProj {
  mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
  mat4 Model;
};

void main() {
  gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
  v_Uv = Vertex_Uv;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform TimeUniform_value {
  float time;
};

void main() {
  float speed = 0.7;
  float translation = sin(time * speed);
  float percentage = 0.6;
  float threshold = v_Uv.x + translation * percentage;

  vec3 red = vec3(1., 0., 0.);
  vec3 blue = vec3(0., 0., 1.);
  vec3 mixed = mix(red, blue, threshold);

  o_Target = vec4(mixed, 1.0);
}
"#;

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
      vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
      fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    });
    pipelines.set_untracked(TEST_PIPELINE_HANDLE,pipeline);

    // Add a `RenderResourcesNode` to our `RenderGraph`. This will bind `TimeComponent` to our
    // shader.
    render_graph.add_system_node(
        "time_uniform",
        RenderResourcesNode::<TimeUniform>::new(true),
    );

    // Add a `RenderGraph` edge connecting our new "time_component" node to the main pass node. This
    // ensures that "time_component" runs before the main pass.
    render_graph
        .add_node_edge("time_uniform", base::node::MAIN_PASS)
        .unwrap();

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
  let uvs: Vec<[f32; 2]> = vec![
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 1.0],
    [1.0, 0.0],
  ];
  mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vetexes);
  mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
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
      })
      .insert(TimeUniform { value: 0.0 });

  // Spawn a camera.
  commands.spawn_bundle(PerspectiveCameraBundle {
      transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..Default::default()
  });
}

/// In this system we query for the `TimeComponent` and global `Time` resource, and set
/// `time.seconds_since_startup()` as the `value` of the `TimeComponent`. This value will be
/// accessed by the fragment shader and used to animate the shader.
pub fn animate_shader(time: Res<Time>, mut query: Query<&mut TimeUniform>) {
  let mut time_uniform = query.single_mut().unwrap();
  time_uniform.value = time.seconds_since_startup() as f32;
}
