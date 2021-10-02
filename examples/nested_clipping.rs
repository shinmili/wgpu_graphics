use graphics::{
    clear,
    draw_state::{Blend, DrawState, Stencil},
    Rectangle,
};
use piston::{
    EventSettings, Events, PressEvent, RenderEvent, ResizeArgs, ResizeEvent, Window, WindowSettings,
};
use winit_window::WinitWindow;

fn main() {
    let mut window = WinitWindow::new(&WindowSettings::new(
        "wgpu_graphics: nested_clipping",
        (640, 480),
    ));

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window.get_window()) };
    let adapter =
        futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

    let (device, queue) = futures::executor::block_on(
        adapter.request_device(&wgpu::DeviceDescriptor::default(), None),
    )
    .unwrap();
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width: window.draw_size().width as u32,
        height: window.draw_size().height as u32,
        present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &surface_config);

    let mut wgpu2d = wgpu_graphics::Wgpu2d::new(&device, &surface_config);
    let mut events = Events::new(EventSettings::new());

    let increment = DrawState::new_increment();
    let inside_level1 = DrawState {
        blend: Some(Blend::Alpha),
        stencil: Some(Stencil::Inside(1)),
        scissor: None,
    };
    let inside_level2 = DrawState {
        blend: Some(Blend::Alpha),
        stencil: Some(Stencil::Inside(2)),
        scissor: None,
    };
    let inside_level3 = DrawState {
        blend: Some(Blend::Alpha),
        stencil: Some(Stencil::Inside(3)),
        scissor: None,
    };
    let mut clip = true;
    while let Some(event) = events.next(&mut window) {
        event.resize(
            |&ResizeArgs {
                 draw_size: [width, height],
                 ..
             }| {
                surface_config = wgpu::SurfaceConfiguration {
                    width,
                    height,
                    ..surface_config
                };
                surface.configure(&device, &surface_config);
            },
        );
        event.render(|render_args| {
            let surface_texture = &surface.get_current_frame().unwrap().output.texture;
            let surface_view = surface_texture.create_view(&wgpu::TextureViewDescriptor::default());

            let command_buffer = wgpu2d.draw(
                &device,
                &surface_config,
                &surface_view,
                render_args.viewport(),
                |c, g| {
                    clear([0.8, 0.8, 0.8, 1.0], g);

                    if clip {
                        Rectangle::new([1.0; 4]).draw(
                            [10.0, 10.0, 200.0, 200.0],
                            &increment,
                            c.transform,
                            g,
                        );
                        Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                            [10.0, 10.0, 200.0, 200.0],
                            &inside_level1,
                            c.transform,
                            g,
                        );

                        Rectangle::new([1.0; 4]).draw(
                            [100.0, 100.0, 200.0, 200.0],
                            &increment,
                            c.transform,
                            g,
                        );
                        Rectangle::new([0.0, 0.0, 1.0, 1.0]).draw(
                            [100.0, 100.0, 200.0, 200.0],
                            &inside_level2,
                            c.transform,
                            g,
                        );

                        Rectangle::new([1.0; 4]).draw(
                            [100.0, 100.0, 200.0, 200.0],
                            &increment,
                            c.transform,
                            g,
                        );
                        Rectangle::new([0.0, 1.0, 0.0, 1.0]).draw(
                            [50.0, 50.0, 200.0, 100.0],
                            &inside_level3,
                            c.transform,
                            g,
                        );
                    } else {
                        Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
                            [10.0, 10.0, 200.0, 200.0],
                            &c.draw_state,
                            c.transform,
                            g,
                        );

                        Rectangle::new([0.0, 0.0, 1.0, 1.0]).draw(
                            [100.0, 100.0, 200.0, 200.0],
                            &c.draw_state,
                            c.transform,
                            g,
                        );

                        Rectangle::new([0.0, 1.0, 0.0, 1.0]).draw(
                            [50.0, 50.0, 200.0, 100.0],
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                },
            );
            queue.submit(std::iter::once(command_buffer));
        });
        event.press(|_| {
            clip = !clip;
        });
    }
}
