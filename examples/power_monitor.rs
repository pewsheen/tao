// Copyright 2014-2021 The winit contributors
// Copyright 2021-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use tao::{
  event::{Event, PowerEvent, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  power_monitor::PowerMonitor,
  window::WindowBuilder,
};

#[allow(clippy::single_match)]
fn main() {
  let event_loop = EventLoop::new();

  let mut window = Some(
    WindowBuilder::new()
      .with_title("A fantastic window!")
      .with_inner_size(tao::dpi::LogicalSize::new(128.0, 128.0))
      .build(&event_loop)
      .unwrap(),
  );

  PowerMonitor::new();

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::PowerEvent {
        event: PowerEvent::ScreenLocked,
        ..
      } => {
        println!("ScreenLocked");
      }
      Event::PowerEvent {
        event: PowerEvent::ScreenUnlocked,
        ..
      } => {
        println!("ScreenUnlocked");
      }
      Event::PowerEvent {
        event: PowerEvent::Suspend,
        ..
      } => {
        println!("Suspend");
      }
      Event::PowerEvent {
        event: PowerEvent::Resume,
        ..
      } => {
        println!("Resume");
      }
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        window_id: _,
        ..
      } => {
        // drop the window to fire the `Destroyed` event
        window = None;
      }
      Event::WindowEvent {
        event: WindowEvent::Destroyed,
        window_id: _,
        ..
      } => {
        *control_flow = ControlFlow::Exit;
      }
      Event::MainEventsCleared => {
        if let Some(w) = &window {
          w.request_redraw();
        }
      }
      _ => (),
    }
  });
}
