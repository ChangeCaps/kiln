use std::{path::PathBuf, time::Instant};

use clap::{crate_authors, crate_version, Parser};
use futures_lite::future;
use linked_hash_map::LinkedHashMap;
use winit::event::Event;

use crate::{
    error::{Error, Result},
    manifest::Manifest,
    render::Renderer,
    shader::{Shader, ShaderUniforms},
    shader_processor::ShaderProcessor,
    window::Window,
};

#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Show {
    #[clap(default_value = ".")]
    pub path: PathBuf,
}

impl Show {
    pub fn run(self) -> Result<()> {
        let manifest_path = self.path.join(Manifest::DEFAULT_PATH);
        let mut manifest = Manifest::load(&manifest_path)?;
        let mut last_modified = manifest_path.metadata()?.modified()?;

        let mut window = Window::new();
        window.title = format!("Kiln - {}", manifest.project.name);
        let mut shader_processor = ShaderProcessor::new();
        let mut shaders = LinkedHashMap::new();
        let mut shaders_invalid = false;

        let start = Instant::now();

        let mut draw_frame = move |renderer: &mut Renderer| -> Result<()> {
            let modified = manifest_path.metadata()?.modified()?;
            let manifest_updated = modified > last_modified;
            if manifest_updated {
                last_modified = modified;
                manifest = Manifest::load(&manifest_path)?;
            }

            renderer
                .device
                .push_error_scope(wgpu::ErrorFilter::Validation);

            if shaders.len() != manifest.shaders.len() || manifest_updated {
                shaders = manifest
                    .shaders
                    .iter()
                    .map(|(name, shader)| {
                        let mut fragment = if let Some(ref fragment) = shader.fragment {
                            fragment.clone()
                        } else {
                            PathBuf::from(format!("{}.wgsl", name))
                        };

                        fragment = self.path.join(fragment);

                        (
                            name.clone(),
                            Shader::new(
                                &renderer.device,
                                &mut shader_processor,
                                shader.vertex.clone(),
                                fragment,
                            ),
                        )
                    })
                    .try_fold(LinkedHashMap::new(), |mut shaders, (name, shader)| {
                        shaders.insert(name, shader?);
                        Result::<_>::Ok(shaders)
                    })?;

                shaders_invalid = false;
            }

            for (_, shader) in shaders.iter_mut() {
                if shader.update(&renderer.device, &mut shader_processor)? {
                    shaders_invalid = false;
                }
            }

            let error = renderer.device.pop_error_scope();

            if let Some(error) = future::block_on(error) {
                shaders_invalid = true;
                return Err(Error::Wgpu(error));
            }

            if shaders_invalid {
                return Ok(());
            }

            renderer.configure();

            let target = renderer.surface.get_current_texture()?;
            let target_view = target.texture.create_view(&Default::default());

            let mut encoder = renderer.device.create_command_encoder(&Default::default());

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("kiln-show-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            let time = Instant::now() - start;
            let uniforms = ShaderUniforms {
                view: manifest.camera.view().to_cols_array_2d(),
                aspect: renderer.config.width as f32 / renderer.config.height as f32,
                time: time.as_secs_f32(),
            };

            for shader in shaders.values() {
                shader.write_uniforms(&renderer.queue, &uniforms);

                render_pass.set_pipeline(&shader.pipeline);
                render_pass.set_bind_group(0, &shader.uniforms_group, &[]);
                render_pass.draw(0..6, 0..1);
            }

            drop(render_pass);

            renderer.queue.submit(std::iter::once(encoder.finish()));

            target.present();

            Ok(())
        };

        window.run(move |event, renderer| match event {
            Event::RedrawRequested(_) => match draw_frame(renderer) {
                Err(err) => {
                    println!("{}", err);
                }
                _ => {}
            },
            Event::RedrawEventsCleared => renderer.request_redraw(),
            _ => {}
        })?;

        Ok(())
    }
}
