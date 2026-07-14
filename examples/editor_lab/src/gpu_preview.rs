use std::{fmt, fs, path::Path};

use eframe::{
    egui,
    egui_wgpu::RenderState,
    wgpu::{self, util::DeviceExt as _},
};
use serde_json::json;

const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const UNIFORM_SIZE: u64 = 16;
const RETIRE_AFTER_FRAMES: u64 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecreateReason {
    InitialAllocation,
    Resize,
    Manual,
    DeviceRecoverySimulation,
}

impl fmt::Display for RecreateReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InitialAllocation => "initial-allocation",
            Self::Resize => "resize",
            Self::Manual => "manual",
            Self::DeviceRecoverySimulation => "device-recovery-simulation",
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PreviewStats {
    pub texture_size: [u32; 2],
    pub generation: u64,
    pub render_passes: u64,
    pub texture_registrations: u64,
    pub texture_releases: u64,
    pub cpu_pixel_readbacks: u64,
    pub cpu_pixel_upload_bytes: u64,
    pub last_recreate_reason: RecreateReason,
}

struct RegisteredTexture {
    _texture: wgpu::Texture,
    view: wgpu::TextureView,
    egui_id: egui::TextureId,
    size: [u32; 2],
    generation: u64,
}

struct RetiredTexture {
    _texture: wgpu::Texture,
    egui_id: egui::TextureId,
    release_at_frame: u64,
}

pub struct GpuPreview {
    render_state: RenderState,
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    active: Option<RegisteredTexture>,
    retired: Vec<RetiredTexture>,
    frame_number: u64,
    generation: u64,
    render_passes: u64,
    texture_registrations: u64,
    texture_releases: u64,
    last_recreate_reason: RecreateReason,
}

impl GpuPreview {
    pub fn new(render_state: RenderState) -> Self {
        let shader = render_state
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("finui-editor-mock-texture-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("mock_texture.wgsl").into()),
            });
        let uniform_buffer =
            render_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("finui-editor-mock-texture-uniform"),
                    contents: &[0; UNIFORM_SIZE as usize],
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        let bind_group_layout =
            render_state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("finui-editor-mock-texture-bind-group-layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(UNIFORM_SIZE),
                        },
                        count: None,
                    }],
                });
        let uniform_bind_group =
            render_state
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("finui-editor-mock-texture-bind-group"),
                    layout: &bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    }],
                });
        let pipeline_layout =
            render_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("finui-editor-mock-texture-pipeline-layout"),
                    bind_group_layouts: &[Some(&bind_group_layout)],
                    immediate_size: 0,
                });
        let pipeline =
            render_state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("finui-editor-mock-texture-pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vertex_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        buffers: &[],
                    },
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fragment_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: TEXTURE_FORMAT,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    multiview_mask: None,
                    cache: None,
                });

        Self {
            render_state,
            pipeline,
            uniform_buffer,
            uniform_bind_group,
            active: None,
            retired: Vec::new(),
            frame_number: 0,
            generation: 0,
            render_passes: 0,
            texture_registrations: 0,
            texture_releases: 0,
            last_recreate_reason: RecreateReason::InitialAllocation,
        }
    }

    pub fn adapter_summary(&self) -> String {
        let info = self.render_state.adapter.get_info();
        format!("{} ({:?}, {:?})", info.name, info.backend, info.device_type)
    }

    pub fn force_recreate(&mut self, reason: RecreateReason) {
        if let Some(size) = self.active.as_ref().map(|texture| texture.size) {
            self.recreate_texture(size, reason);
        }
    }

    pub fn render(&mut self, requested_size: [u32; 2], elapsed_seconds: f32) -> egui::TextureId {
        self.frame_number += 1;
        self.release_retired_textures();

        let size = quantize_size(requested_size);
        let recreate_reason = match self.active.as_ref() {
            None => Some(RecreateReason::InitialAllocation),
            Some(active) if active.size != size => Some(RecreateReason::Resize),
            Some(_) => None,
        };
        if let Some(reason) = recreate_reason {
            self.recreate_texture(size, reason);
        }

        let active = self.active.as_ref().expect("texture was allocated above");
        let uniform = uniform_bytes(elapsed_seconds, active.size, active.generation as f32);
        self.render_state
            .queue
            .write_buffer(&self.uniform_buffer, 0, &uniform);

        let mut encoder =
            self.render_state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("finui-editor-mock-texture-encoder"),
                });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("finui-editor-mock-texture-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &active.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }
        self.render_state.queue.submit([encoder.finish()]);
        self.render_passes += 1;
        active.egui_id
    }

    pub fn stats(&self) -> PreviewStats {
        let active = self.active.as_ref();
        PreviewStats {
            texture_size: active.map_or([0, 0], |texture| texture.size),
            generation: active.map_or(0, |texture| texture.generation),
            render_passes: self.render_passes,
            texture_registrations: self.texture_registrations,
            texture_releases: self.texture_releases,
            cpu_pixel_readbacks: 0,
            cpu_pixel_upload_bytes: 0,
            last_recreate_reason: self.last_recreate_reason,
        }
    }

    pub fn write_smoke_receipt(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }

        let info = self.render_state.adapter.get_info();
        let stats = self.stats();
        let receipt = json!({
            "adapter": {
                "backend": format!("{:?}", info.backend),
                "device_type": format!("{:?}", info.device_type),
                "name": info.name,
            },
            "bridge": "wgpu-render-attachment-to-egui-native-texture",
            "cpu_pixel_readbacks": stats.cpu_pixel_readbacks,
            "cpu_pixel_upload_bytes": stats.cpu_pixel_upload_bytes,
            "generation": stats.generation,
            "last_recreate_reason": stats.last_recreate_reason.to_string(),
            "platform": std::env::consts::OS,
            "render_passes": stats.render_passes,
            "texture_registrations": stats.texture_registrations,
            "texture_releases": stats.texture_releases,
            "texture_size": stats.texture_size,
        });
        let contents = serde_json::to_string_pretty(&receipt).map_err(|error| error.to_string())?;
        fs::write(path, format!("{contents}\n")).map_err(|error| error.to_string())
    }

    fn recreate_texture(&mut self, size: [u32; 2], reason: RecreateReason) {
        let texture = self
            .render_state
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("finui-editor-mock-texture"),
                size: wgpu::Extent3d {
                    width: size[0],
                    height: size[1],
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: TEXTURE_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let egui_id = self.render_state.renderer.write().register_native_texture(
            &self.render_state.device,
            &view,
            wgpu::FilterMode::Linear,
        );

        self.generation += 1;
        self.texture_registrations += 1;
        self.last_recreate_reason = reason;
        let replacement = RegisteredTexture {
            _texture: texture,
            view,
            egui_id,
            size,
            generation: self.generation,
        };
        if let Some(previous) = self.active.replace(replacement) {
            self.retired.push(RetiredTexture {
                _texture: previous._texture,
                egui_id: previous.egui_id,
                release_at_frame: self.frame_number + RETIRE_AFTER_FRAMES,
            });
        }
    }

    fn release_retired_textures(&mut self) {
        let frame_number = self.frame_number;
        let renderer = self.render_state.renderer.clone();
        let mut renderer = renderer.write();
        self.retired.retain(|texture| {
            if texture.release_at_frame <= frame_number {
                renderer.free_texture(&texture.egui_id);
                self.texture_releases += 1;
                false
            } else {
                true
            }
        });
    }
}

impl Drop for GpuPreview {
    fn drop(&mut self) {
        let mut renderer = self.render_state.renderer.write();
        if let Some(active) = self.active.take() {
            renderer.free_texture(&active.egui_id);
        }
        for retired in self.retired.drain(..) {
            renderer.free_texture(&retired.egui_id);
        }
    }
}

fn quantize_size(requested: [u32; 2]) -> [u32; 2] {
    requested.map(|value| {
        let clamped = value.clamp(16, 2048);
        clamped.div_ceil(16) * 16
    })
}

fn uniform_bytes(elapsed_seconds: f32, size: [u32; 2], generation: f32) -> [u8; 16] {
    let values = [elapsed_seconds, size[0] as f32, size[1] as f32, generation];
    let mut bytes = [0; 16];
    for (index, value) in values.into_iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_ne_bytes());
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::{quantize_size, uniform_bytes};

    #[test]
    fn texture_size_is_bounded_and_quantized() {
        assert_eq!(quantize_size([1, 17]), [16, 32]);
        assert_eq!(quantize_size([1919, 1080]), [1920, 1088]);
        assert_eq!(quantize_size([9000, 9000]), [2048, 2048]);
    }

    #[test]
    fn frame_uniform_has_four_f32_values() {
        let bytes = uniform_bytes(1.5, [640, 360], 3.0);
        let decoded = std::array::from_fn::<_, 4, _>(|index| {
            f32::from_ne_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
        });
        assert_eq!(decoded, [1.5, 640.0, 360.0, 3.0]);
    }
}
