use bevy::{
  prelude::*,
  reflect::TypeUuid,
  render::{
    mesh::Indices,
    pipeline::*,
    render_graph::base::MainPass,
    shader::{ShaderStage, ShaderStages},
    texture::TextureFormat
  }
};
pub struct TrianglePlugin;
impl Plugin for TrianglePlugin {
  fn build(&self, app: &mut AppBuilder) {
    //let mut world = app.world_mut();
    let world_cell = app.world_mut().cell();
    let mut pipelines = world_cell
      .get_resource_mut::<Assets<PipelineDescriptor>>()
      .unwrap();
    let mut shaders = world_cell.get_resource_mut::<Assets<Shader>>().unwrap();
    //let mut meshes = world_cell.get_resource_mut::<Assets<Mesh>>().unwrap();
    pipelines.set_untracked(TRIANGLE_PIPELINE_HANDLE,build_line_pipeline(&mut shaders, PrimitiveTopology::TriangleList));
    pipelines.set_untracked(TRIANGLE_STRIP_PIPELINE_HANDLE,build_line_pipeline(&mut shaders, PrimitiveTopology::LineStrip));
  }
}

pub struct TriangleStripBuilder {
  default_color: Option<[f32; 4]>,
  indecies: Vec<u32>,
  points: Vec<[f32; 3]>,
  colors: Vec<[f32; 4]>,
}

#[allow(dead_code)]
impl TriangleStripBuilder {
  pub fn new() -> Self {
    Self {
      default_color: None,
      indecies: Default::default(),
      points: Default::default(),
      colors: Default::default(),
    }
  }
  pub fn set_default_color(mut self, color: Color) -> Self {
    self.default_color = Some([color.r(), color.g(), color.b(), color.a()]);
    self
  }
  pub fn push_vertex(mut self, point: Vec3) -> Self {
    self.points.push([point.x, point.y, point.z]);
    if let Some(default_color) = self.default_color {
      self.colors.push(default_color);
    } else if self.colors.is_empty() {
      self.colors.push([1.0, 1.0, 1.0, 1.0]);
    } else {
      let last = self.colors.last().unwrap().clone();
      self.colors.push(last);
    }
    self
  }
  pub fn push_vertex_and_color(mut self, point: Vec3, color: Color) -> Self {
    self.points.push([point.x, point.y, point.z]);
    self.colors.push([color.r(), color.g(), color.b(), color.a()]);
    self
  }
  pub fn push_indices(mut self, indices: &[u32]) -> Self {
    self.indecies.extend(indices);
    self
  }
  pub fn push_index(mut self, index: u32) -> Self {
    self.indecies.push(index);
    self
  }
  pub fn build(self, meshes: &mut ResMut<Assets<Mesh>>) -> TriangleBundle {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
    mesh.set_indices(Some(Indices::U32(self.indecies)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, self.points);
    mesh.set_attribute("Vertex_ColorWithAlpha", self.colors);

    TriangleBundle {
      mesh: meshes.add(mesh),
      render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
        TRIANGLE_STRIP_PIPELINE_HANDLE.typed(),
      )]),
      ..Default::default()
    }
  }
}

pub struct TriangleListBuilder {
  default_color: Option<[f32; 4]>,
  indecies: Vec<u32>,
  points: Vec<[f32; 3]>,
  colors: Vec<[f32; 4]>,
}

#[allow(dead_code)]
impl TriangleListBuilder {
  pub fn new() -> Self {
    Self {
      default_color: None,
      indecies: Default::default(),
      points: Default::default(),
      colors: Default::default(),
    }
  }
  pub fn set_default_color(mut self, color: Color) -> Self {
    self.default_color = Some([color.r(), color.g(), color.b(), color.a()]);
    self
  }
  pub fn push_vertex(mut self, point: Vec3) -> Self {
    self.points.push([point.x, point.y, point.z]);
    if let Some(default_color) = self.default_color {
      self.colors.push(default_color);
    } else if self.colors.is_empty() {
      self.colors.push([1.0, 1.0, 1.0, 1.0]);
    } else {
      let last = self.colors.last().unwrap().clone();
      self.colors.push(last);
    }
    self
  }
  pub fn push_vertex_and_color(mut self, point: Vec3, color: Color) -> Self {
    self.points.push([point.x, point.y, point.z]);
    self.colors.push([color.r(), color.g(), color.b(), color.a()]);
    self
  }
  pub fn push_triangle(mut self, i1: u32, i2: u32, i3:u32) -> Self {
    self.indecies.extend(&[i1, i2, i3]);
    self
  }
  pub fn build(self, meshes: &mut ResMut<Assets<Mesh>>) -> TriangleBundle {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
    mesh.set_indices(Some(Indices::U32(self.indecies)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, self.points);
    mesh.set_attribute("Vertex_ColorWithAlpha", self.colors);

    TriangleBundle {
      mesh: meshes.add(mesh),
      render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
        TRIANGLE_STRIP_PIPELINE_HANDLE.typed(),
      )]),
      ..Default::default()
    }
  }
}

#[derive(Default, Debug, Bundle, Clone)]
pub struct TriangleBundle {
  pub mesh: Handle<Mesh>,
  pub draw: Draw,
  pub visible: Visible,
  pub render_pipelines: RenderPipelines,
  pub main_pass: MainPass,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
}

pub const TRIANGLE_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 16566712244797685760);
pub const TRIANGLE_STRIP_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 6657636463977684992);

fn build_line_pipeline(shaders: &mut Assets<Shader>, topology: PrimitiveTopology) -> PipelineDescriptor {
  PipelineDescriptor {
      depth_stencil: Some(DepthStencilState {
          format: TextureFormat::Depth32Float,
          depth_write_enabled: true,
          depth_compare: CompareFunction::LessEqual,
          stencil: StencilState {
              front: StencilFaceState::IGNORE,
              back: StencilFaceState::IGNORE,
              read_mask: 0,
              write_mask: 0,
          },
          bias: DepthBiasState {
              constant: 0,
              slope_scale: 0.0,
              clamp: 0.0,
          },
          clamp_depth: false,
      }),
      color_target_states: vec![ColorTargetState {
          format: TextureFormat::default(),
          color_blend: BlendState {
              src_factor: BlendFactor::SrcAlpha,
              dst_factor: BlendFactor::OneMinusSrcAlpha,
              operation: BlendOperation::Add,
          },
          alpha_blend: BlendState {
              src_factor: BlendFactor::One,
              dst_factor: BlendFactor::One,
              operation: BlendOperation::Add,
          },
          write_mask: ColorWrite::ALL,
      }],
      primitive: PrimitiveState {
          topology,
          strip_index_format: None,
          front_face: FrontFace::Ccw,
          cull_mode: CullMode::None,
          polygon_mode: PolygonMode::Fill,
      },
      ..PipelineDescriptor::new(ShaderStages {
          vertex: shaders.add(Shader::from_glsl(
              ShaderStage::Vertex,
              VERT_SHADER,
          )),
          fragment: Some(shaders.add(Shader::from_glsl(
              ShaderStage::Fragment,
              FRAG_SHADER,
          ))),
      })
  }
}

const VERT_SHADER: &'static str = r###"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec4 Vertex_ColorWithAlpha;

layout(location = 0) out vec4 v_Color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Color = Vertex_ColorWithAlpha;
    vec3 position = Vertex_Position;
    gl_Position = ViewProj * Model * vec4(position, 1.0);
}
"###;

const FRAG_SHADER: &'static str = r###"
#version 450

layout(location = 0) in vec4 v_Color;

layout(location = 0) out vec4 o_Target;

void main() {
  o_Target = v_Color;
}
"###;
