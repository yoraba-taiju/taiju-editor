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
pub struct LinePlugin;
impl Plugin for LinePlugin {
  fn build(&self, app: &mut AppBuilder) {
    //let mut world = app.world_mut();
    let world_cell = app.world_mut().cell();
    let mut pipelines = world_cell
      .get_resource_mut::<Assets<PipelineDescriptor>>()
      .unwrap();
    let mut shaders = world_cell.get_resource_mut::<Assets<Shader>>().unwrap();
    //let mut meshes = world_cell.get_resource_mut::<Assets<Mesh>>().unwrap();
    pipelines.set_untracked(LINE_LIST_PIPELINE_HANDLE,build_line_pipeline(&mut shaders, PrimitiveTopology::LineList));
    pipelines.set_untracked(LINE_STRIP_PIPELINE_HANDLE,build_line_pipeline(&mut shaders, PrimitiveTopology::LineStrip));
  }
}

pub struct LineStripBuilder {
  looped: bool,
  default_color: Option<[f32; 4]>,
  points: Vec<[f32; 3]>,
  colors: Vec<[f32; 4]>,
}

#[allow(dead_code)]
impl LineStripBuilder {
  pub fn new_non_loop() -> Self {
    Self {
      looped: false,
      default_color: None,
      points: Default::default(),
      colors: Default::default(),
    }
  }
  pub fn new_loop() -> Self {
    Self {
      looped: true,
      default_color: None,
      points: Default::default(),
      colors: Default::default(),
    }
  }
  pub fn set_default_color(mut self, color: Color) -> Self {
    self.default_color = Some([color.r(), color.g(), color.b(), color.a()]);
    self
  }
  pub fn append(mut self, point: Vec3) -> Self {
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

  pub fn append_with_color(mut self, point: Vec3, color: Color) -> Self {
    self.points.push([point.x, point.y, point.z]);
    self.colors.push([color.r(), color.g(), color.b(), color.a()]);
    self
  }

  pub fn build_bundle(self, meshes: &mut Assets<Mesh>) -> LineBundle {
    let mut indicies: Vec<u32> = (0..self.points.len() as u32).collect();
    if self.looped {
      indicies.push(0);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
    mesh.set_indices(Some(Indices::U32(indicies)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, self.points);
    mesh.set_attribute("Vertex_ColorWithAlpha", self.colors);

    LineBundle {
      mesh: meshes.add(mesh),
      render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
        LINE_STRIP_PIPELINE_HANDLE.typed(),
      )]),
      ..Default::default()
    }
  }
}

pub struct LineListBuilder {
  default_color: Option<[f32; 4]>,
  points: Vec<[f32; 3]>,
  colors: Vec<[f32; 4]>,
}

#[allow(dead_code)]
impl LineListBuilder {
  pub fn new() -> Self {
    Self {
      default_color: None,
      points: Default::default(),
      colors: Default::default(),
    }
  }
  pub fn set_default_color(mut self, color: Color) -> Self {
    self.default_color = Some([color.r(), color.g(), color.b(), color.a()]);
    self
  }

  pub fn append(mut self, from: Vec3, to: Vec3) -> Self {
    self.points.push([from.x, from.y, from.z]);
    self.points.push([to.x, to.y, to.z]);
    if let Some(default_color) = self.default_color {
      self.colors.push(default_color);
      self.colors.push(default_color);
    } else if self.colors.is_empty() {
      self.colors.push([1.0, 1.0, 1.0, 1.0]);
      self.colors.push([1.0, 1.0, 1.0, 1.0]);
    } else {
      let last1 = self.colors[self.colors.len()-2];
      let last2 = self.colors[self.colors.len()-1];
      self.colors.push(last1);
      self.colors.push(last2);
    }
    self
  }
  pub fn append_with_color(mut self, from: Vec3, to: Vec3, color1: Color, color2: Color) -> Self {
    self.points.push([from.x, from.y, from.z]);
    self.points.push([to.x, to.y, to.z]);
    self.colors.push([color1.r(), color1.g(), color1.b(), color1.a()]);
    self.colors.push([color2.r(), color2.g(), color2.b(), color2.a()]);
    self
  }
  pub fn build(self, meshes: &mut Assets<Mesh>) -> LineBundle {
    let indicies = (0..(self.points.len() as u32)).collect::<Vec<u32>>();
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_indices(Some(Indices::U32(indicies)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, self.points);
    mesh.set_attribute("Vertex_ColorWithAlpha", self.colors);

    LineBundle {
      mesh: meshes.add(mesh),
      render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
        LINE_LIST_PIPELINE_HANDLE.typed(),
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

pub const LINE_LIST_PIPELINE_HANDLE: HandleUntyped =
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
layout(location = 1) in vec4 Vertex_ColorWithAlpha;

layout(location = 0) out vec4 v_Color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
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

layout(location = 0) in vec4 v_Color;

layout(location = 0) out vec4 o_Target;

void main() {
  o_Target = v_Color;
}
"###;
