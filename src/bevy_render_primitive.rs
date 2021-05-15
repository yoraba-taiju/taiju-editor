use bevy::{prelude::*, reflect::TypeUuid, render::{mesh::Indices, pipeline::*, render_graph::base::MainPass, shader::{ShaderStage, ShaderStages}, texture::TextureFormat}};
pub struct PrimitiveRendererPlugin;
impl Plugin for PrimitiveRendererPlugin {
  fn build(&self, app: &mut AppBuilder) {
    //let mut world = app.world_mut();
    let world_cell = app.world_mut().cell();
    let mut pipelines = world_cell
      .get_resource_mut::<Assets<PipelineDescriptor>>()
      .unwrap();
    let mut shaders = world_cell.get_resource_mut::<Assets<Shader>>().unwrap();
    //let mut meshes = world_cell.get_resource_mut::<Assets<Mesh>>().unwrap();
    pipelines.set_untracked(LINES_PIPELINE_HANDLE,build_line_pipeline(&mut shaders, PrimitiveTopology::LineList));
    pipelines.set_untracked(LINE_STRIP_PIPELINE_HANDLE,build_line_pipeline(&mut shaders, PrimitiveTopology::LineStrip));
  }
}

#[derive(Debug)]
pub struct LineStripBuilder {
  looped: bool,
  points: Vec<[f32; 3]>,
  colors: Vec<[f32; 3]>,
}

#[allow(dead_code)]
impl LineStripBuilder {
  pub fn new_non_loop() -> Self {
    Self {
      looped: false,
      points: Default::default(),
      colors: Default::default(),
    }
  }
  pub fn new_loop() -> Self {
    Self {
      looped: true,
      points: Default::default(),
      colors: Default::default(),
    }
  }
  pub fn append(mut self, point: Vec3) -> Self {
    self.points.push([point.x, point.y, point.z]);
    if self.colors.is_empty() {
      self.colors.push([1.0, 1.0, 1.0]);
    } else {
      let last = self.colors.last().unwrap().clone();
      self.colors.push(last);
    }
    self
  }

  pub fn append_with_color(mut self, point: Vec3, color: Vec3) -> Self {
    self.points.push([point.x, point.y, point.z]);
    self.colors.push([color.x, color.y, color.z]);
    self
  }

  pub fn build(self, meshes: &mut ResMut<Assets<Mesh>>) -> LineBundle {
    let mut indicies = (0..(self.points.len() as u32)).collect::<Vec<u32>>();
    if self.looped {
      indicies.push(0);
    }
    println!("{:?}", self);
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_indices(Some(Indices::U32(indicies)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, self.points);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, self.colors);

    LineBundle {
      mesh: meshes.add(mesh),
      render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
        LINE_STRIP_PIPELINE_HANDLE.typed(),
      )]),
      ..Default::default()
    }
  }
}

#[derive(Debug)]
pub struct LinesBuilder {
  points: Vec<[f32; 3]>,
  colors: Vec<[f32; 3]>,
}

#[allow(dead_code)]
impl LinesBuilder {
  pub fn new() -> Self {
    Self {
      points: Default::default(),
      colors: Default::default(),
    }
  }
  pub fn append(mut self, from: Vec3, to: Vec3) -> Self {
    self.points.push([from.x, from.y, from.z]);
    self.points.push([to.x, to.y, to.z]);
    if self.colors.is_empty() {
      self.colors.push([1.0, 1.0, 1.0]);
      self.colors.push([1.0, 1.0, 1.0]);
    } else {
      let last1 = self.colors[self.colors.len()-2];
      let last2 = self.colors[self.colors.len()-1];
      self.colors.push(last1);
      self.colors.push(last2);
    }
    self
  }
  pub fn append_with_color(mut self, from: Vec3, to: Vec3, color1: Vec3, color2: Vec3) -> Self {
    self.points.push([from.x, from.y, from.z]);
    self.points.push([to.x, to.y, to.z]);
    self.colors.push([color1.x, color1.y, color1.z]);
    self.colors.push([color2.x, color2.y, color2.z]);
    self
  }
  pub fn build(self, meshes: &mut ResMut<Assets<Mesh>>) -> LineBundle {
    let indicies = (0..(self.points.len() as u32)).collect::<Vec<u32>>();
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_indices(Some(Indices::U32(indicies)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, self.points);
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, self.colors);

    LineBundle {
      mesh: meshes.add(mesh),
      render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
        LINES_PIPELINE_HANDLE.typed(),
      )]),
      ..Default::default()
    }
  }
}

#[derive(Default, Debug, Bundle, Clone)]
pub struct LineBundle {
  pub mesh: Handle<Mesh>,
  pub draw: Draw,
  pub visible: Visible,
  pub render_pipelines: RenderPipelines,
  pub main_pass: MainPass,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
}

pub const LINES_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 16566712244797685760);
pub const LINE_STRIP_PIPELINE_HANDLE: HandleUntyped =
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
layout(location = 1) in vec3 Vertex_Color;

layout(location = 0) out vec3 v_Color;

layout(set = 0, binding = 0) uniform CameraViewProj {
  mat4 ViewProj;
};

layout(set = 2, binding = 0) uniform Transform {
  mat4 Model;
};

void main() {
  v_Color = Vertex_Color;
  vec3 position = Vertex_Position;
  gl_Position = ViewProj * Model * vec4(position, 1.0);
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
