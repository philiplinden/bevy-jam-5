//! A shader and a material that uses it.
use bevy::{
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{ComponentUniforms, DynamicUniformIndex, ExtractComponent},
        globals::{GlobalsBuffer, GlobalsUniform},
        render_asset::RenderAssets,
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, ViewNode, ViewNodeRunner, RenderLabel,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            AsBindGroupShaderType, ShaderRef, *,
        },
        renderer::{RenderContext, RenderDevice},
        texture::{BevyDefault, GpuImage},
        view::ViewTarget,
        RenderApp,
    },
    sprite::{Material2d, Material2dPlugin},
};

use crate::ui::palette::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        Material2dPlugin::<OscilloscopeMaterial>::default(),
        CrtMaterialPlugin,
));
}

// FIXME: This should be loaded in `assets.rs` as a resource
const WAVE_SHADER_ASSET_PATH: &str = "shaders/oscilloscope.wgsl";

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// #[reflect(Default, Debug)]
#[uniform(0, OscilloscopeMaterialUniform)]
pub struct OscilloscopeMaterial {
    pub foreground: Color,
    pub background: Color,
    #[storage(2, read_only)]
    pub points: Vec<Vec2>,
    #[storage(3, read_only)]
    pub lines: Vec<UVec2>,
    // #[texture(3)]
    // #[sampler(4)]
    // color_texture: Option<Handle<Image>>,
}

impl Default for OscilloscopeMaterial {
    fn default() -> Self {
        OscilloscopeMaterial {
            foreground: WAVEFORM_COLOR,
            background: OSCILLOSCOPE_SCREEN_COLOR,
            points: vec![],
            lines: vec![],
        }
    }
}

/// The GPU representation of the uniform data of a [`OscilloscopeMaterial`].
#[derive(Clone, Default, ShaderType)]
struct OscilloscopeMaterialUniform {
    pub foreground: LinearRgba,
    pub background: LinearRgba,
}

impl AsBindGroupShaderType<OscilloscopeMaterialUniform> for OscilloscopeMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<GpuImage>,
    ) -> OscilloscopeMaterialUniform {
        // let mut flags = OscilloscopeMaterialFlags::NONE;
        // if self.texture.is_some() {
        //     flags |= OscilloscopeMaterialFlags::TEXTURE;
        // }

        OscilloscopeMaterialUniform {
            foreground: LinearRgba::from(self.foreground),
            background: LinearRgba::from(self.background),
        }
    }
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for OscilloscopeMaterial {
    fn fragment_shader() -> ShaderRef {
        WAVE_SHADER_ASSET_PATH.into()
    }
}

/// Based on [CRT Effect by Jasper](https://www.shadertoy.com/view/4sf3Dr)
const CRT_SHADER_ASSET_PATH: &str = "shaders/crt.wgsl";

pub struct CrtMaterialPlugin;

impl Plugin for CrtMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CrtSettings>();

        // We need to get the render app from the main app
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Bevy's renderer uses a render graph which is a collection of nodes in a directed acyclic graph.
            // It currently runs on each view/camera and executes each node in the specified order.
            // It will make sure that any node that needs a dependency from another node
            // only runs when that dependency is done.
            //
            // Each node can execute arbitrary work, but it generally runs at least one render pass.
            // A node only has access to the render world, so if you need data from the main world
            // you need to extract it manually or with the plugin like above.
            // Add a [`Node`] to the [`RenderGraph`]
            // The Node needs to impl FromWorld
            //
            // The [`ViewNodeRunner`] is a special [`Node`] that will automatically run the node for each viewwas
            // matching the [`ViewQuery`]
            .add_render_graph_node::<ViewNodeRunner<CrtMaterialNode>>(
                // Specify the label of the graph, in this case we want the graph for 3d
                Core3d, // It also needs the label of the node
                CrtMaterialLabel,
            )
            .add_render_graph_edges(
                Core3d,
                // Specify the node ordering.
                // This will automatically create all required node edges to enforce the given ordering.
                (
                    Node3d::Tonemapping,
                    CrtMaterialLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            )
            .add_render_graph_node::<ViewNodeRunner<CrtMaterialNode>>(
                // Specify the label of the graph, in this case we want the graph for 2d
                Core2d, // It also needs the label of the node
                CrtMaterialLabel,
            )
            .add_render_graph_edges(
                Core2d,
                // Specify the node ordering.
                // This will automatically create all required node edges to enforce the given ordering.
                (
                    Node2d::Tonemapping,
                    CrtMaterialLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        // We need to get the render app from the main app
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Initialize the pipeline
            .init_resource::<CrtPipeline>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct CrtMaterialLabel;

// The post process node used for the render graph
#[derive(Default)]
struct CrtMaterialNode;

// The ViewNode trait is required by the ViewNodeRunner
impl ViewNode for CrtMaterialNode {
    // The node needs a query to gather data from the ECS in order to do its rendering,
    // but it's not a normal system so we need to define it manually.
    //
    // This query will only run on the view entity
    type ViewQuery = (
        &'static ViewTarget,
        // This makes sure the node only runs on cameras with the CrtSettings component
        &'static CrtSettings,
        // As there could be multiple post processing components sent to the GPU (one per camera),
        // we need to get the index of the one that is associated with the current view.
        &'static DynamicUniformIndex<CrtSettings>,
    );

    // Runs the node logic
    // This is where you encode draw commands.
    //
    // This will run on every view on which the graph is running.
    // If you don't want your effect to run on every camera,
    // you'll need to make sure you have a marker component as part of [`ViewQuery`]
    // to identify which camera(s) should run the effect.
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _post_process_settings, settings_index): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // Get the pipeline resource that contains the global data we need
        // to create the render pipeline
        let post_process_pipeline = world.resource::<CrtPipeline>();

        // The pipeline cache is a cache of all previously created pipelines.
        // It is required to avoid creating a new pipeline each frame,
        // which is expensive due to shader compilation.
        let pipeline_cache = world.resource::<PipelineCache>();

        // Get the pipeline from the cache
        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        // Get the settings uniform binding
        let settings_uniforms = world.resource::<ComponentUniforms<CrtSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let globals_buffer = world.resource::<GlobalsBuffer>();
        let Some(global_uniforms) = globals_buffer.buffer.binding() else {
            return Ok(());
        };

        // This will start a new "post process write", obtaining two texture
        // views from the view target - a `source` and a `destination`.
        // `source` is the "current" main texture and you _must_ write into
        // `destination` because calling `post_process_write()` on the
        // [`ViewTarget`] will internally flip the [`ViewTarget`]'s main
        // texture to the `destination` texture. Failing to do so will cause
        // the current main texture information to be lost.
        let post_process = view_target.post_process_write();

        // The bind_group gets created each frame.
        //
        // Normally, you would create a bind_group in the Queue set,
        // but this doesn't work with the post_process_write().
        // The reason it doesn't work is because each post_process_write will alternate the source/destination.
        // The only way to have the correct source/destination for the bind_group
        // is to make sure you get it during the node execution.
        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layout,
            // It's important for this to match the BindGroupLayout defined in the CrtPipeline
            &BindGroupEntries::sequential((
                // Make sure to use the source view
                post_process.source,
                // Use the sampler created for the pipeline
                &post_process_pipeline.sampler,
                // Set the settings binding
                settings_binding.clone(),
                global_uniforms,
            )),
        );

        // Begin the render pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                // We need to specify the post process destination view here
                // to make sure we write to the appropriate texture.
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // This is mostly just wgpu boilerplate for drawing a fullscreen triangle,
        // using the pipeline/bind_group created above
        render_pass.set_render_pipeline(pipeline);
        // By passing in the index of the post process settings on this view, we ensure
        // that in the event that multiple settings were sent to the GPU (as would be the
        // case with multiple cameras), we use the correct one.
        render_pass.set_bind_group(0, &bind_group, &[settings_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

// This contains global data used by the render pipeline. This will be created once on startup.
#[derive(Resource)]
struct CrtPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for CrtPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // We need to define the bind group layout used for our pipeline
        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // The sampler that will be used to sample the screen texture
                    sampler(SamplerBindingType::Filtering),
                    // The settings uniform that will control the effect
                    uniform_buffer::<CrtSettings>(true),
                    uniform_buffer::<GlobalsUniform>(false),
                ),
            ),
        );

        // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        // Get the shader handle
        let shader = world.load_asset(CRT_SHADER_ASSET_PATH);

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("post_process_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}

// This is the component that will get passed to the shader
#[derive(Component, Reflect, Clone, Copy, ExtractComponent, ShaderType)]
#[reflect(Component, Default)]
pub struct CrtSettings {
    scanline_intensity: f32,
    bend_radius: f32,
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: Vec3,
}

impl Default for CrtSettings {
    fn default() -> Self {
        Self {
            scanline_intensity: 0.0,
            bend_radius: 5.0,
            _webgl2_padding: Vec3::ZERO,
        }
    }
}
