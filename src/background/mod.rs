use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

use rand_distr::{Distribution, UnitSphere};

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform BackgroundMaterial_color {
    vec4 color;
};
void main() {
    o_Target = color;
}
"#;

const NUM_STARS: i32 = 0;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0dbbc306-413c-4a2f-8f24-4a8212205619"]
pub struct BackgroundMaterial {
    pub color: Color,
}

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<BackgroundMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let mut rng = rand::thread_rng();
    let star_pos: Vec<Vec3> = UnitSphere
        .sample_iter(&mut rng)
        .take(NUM_STARS as usize)
        .map(|xyz| 800. * Vec3::new(xyz[0], xyz[1], xyz[2]))
        .collect();

    star_pos.into_iter().for_each(|_pos| {});

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    render_graph.add_system_node(
        "background_material",
        AssetRenderResourcesNode::<BackgroundMaterial>::new(true),
    );

    render_graph
        .add_node_edge("background_material", base::node::MAIN_PASS)
        .unwrap();

    // cube
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1000.0 }));
    let material = custom_materials.add(BackgroundMaterial {
        color: Color::DARK_GRAY,
    });

    let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
    transform.scale = Vec3::new(-1., -1., -1.);

    commands
        .spawn_bundle(MeshBundle {
            mesh,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform,
            ..Default::default()
        })
        .insert(material);
}