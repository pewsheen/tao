use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

fn main() {
  env_logger::init();
  let event_loop = EventLoop::new();

  let _focusable = WindowBuilder::new()
    .with_title("Focusable")
    .with_inner_size(tao::dpi::LogicalSize::new(420.0, 200.0))
    .with_position(tao::dpi::LogicalPosition::new(100.0, 100.0))
    .with_focusable(true)
    .with_minimizable(true)
    .build(&event_loop)
    .unwrap();

  let _non_focusable = WindowBuilder::new()
    .with_title("Non-focusable")
    .with_inner_size(tao::dpi::LogicalSize::new(420.0, 200.0))
    .with_position(tao::dpi::LogicalPosition::new(550.0, 100.0))
    .with_focusable(false)
    .with_minimizable(true)
    .build(&event_loop)
    .unwrap();

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => {
        *control_flow = ControlFlow::Exit;
      }
      _ => {}
    }
  });
}
