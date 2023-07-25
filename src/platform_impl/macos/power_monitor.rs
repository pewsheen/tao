use std::sync::Once;

use crate::event::Event::PowerEvent as PowerEventWrapper;
use crate::{
  event::PowerEvent,
  platform_impl::platform::{app_state::AppState, event::EventWrapper},
};
use cocoa::{
  base::{id, nil},
  foundation::NSString,
};
use libc::c_void;
use objc::{
  declare::ClassDecl,
  runtime::{Class, Object, Sel},
};

struct PowerMonitorClass(*const Class);
unsafe impl Send for PowerMonitorClass {}
unsafe impl Sync for PowerMonitorClass {}

pub struct PowerMonitor {
  monitor: Option<id>,
}

impl PowerMonitor {
  pub fn new() -> Self {
    println!("platform - new PowerMonitor");

    unsafe {
      let power_monitor_class = get_or_init_power_monitor_class();
      let monitor: id = msg_send![power_monitor_class, alloc];
      let monitor: id = msg_send![monitor, init];
      let monitor: id = msg_send![monitor, init_monitor];

      dbg!(monitor);

      Self {
        monitor: Some(monitor),
      }
    }
  }
}

impl Default for PowerMonitor {
  fn default() -> Self {
    Self::new()
  }
}

extern "C" fn init_monitor(this: &Object, _sel: Sel) -> id {
  println!("init()");
  unsafe {
    let this: id = msg_send![this, init];
    if this != nil {
      let notification_center: &Object =
        msg_send![class!(NSDistributedNotificationCenter), defaultCenter];

      let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
      let ws_notification_center: &Object = msg_send![workspace, notificationCenter];

      let () = msg_send![
          notification_center,
          addObserver: this
          selector: sel!(onScreenLocked:)
          name: NSString::alloc(nil).init_str("com.apple.screenIsLocked")
          object: nil
      ];

      let () = msg_send![
          notification_center,
          addObserver: this
          selector: sel!(onScreenUnlocked:)
          name: NSString::alloc(nil).init_str("com.apple.screenIsUnlocked")
          object: nil
      ];

      let () = msg_send![
          ws_notification_center,
          addObserver: this
          selector: sel!(onSuspend:)
          name: NSString::alloc(nil).init_str("NSWorkspaceWillSleepNotification")
          object: nil
      ];

      let () = msg_send![
          ws_notification_center,
          addObserver: this
          selector: sel!(onResume:)
          name: NSString::alloc(nil).init_str("NSWorkspaceDidWakeNotification")
          object: nil
      ];
    }
    this
  }
}

extern "C" fn on_screen_locked(_this: &Object, _sel: Sel, _state: *mut c_void) {
  AppState::queue_event(EventWrapper::StaticEvent(PowerEventWrapper {
    event: PowerEvent::ScreenLocked,
  }));
}

extern "C" fn on_screen_unlocked(_this: &Object, _sel: Sel, _state: *mut c_void) {
  AppState::queue_event(EventWrapper::StaticEvent(PowerEventWrapper {
    event: PowerEvent::ScreenUnlocked,
  }));
}

extern "C" fn on_suspend(_this: &Object, _sel: Sel, _state: *mut c_void) {
  AppState::queue_event(EventWrapper::StaticEvent(PowerEventWrapper {
    event: PowerEvent::Suspend,
  }));
}

extern "C" fn on_resume(_this: &Object, _sel: Sel, _state: *mut c_void) {
  AppState::queue_event(EventWrapper::StaticEvent(PowerEventWrapper {
    event: PowerEvent::Resume,
  }));
}

extern "C" fn dealloc(this: &Object, _sel: Sel) {
  println!("dealloc()");
  unsafe {
    let notification_center: &Object =
      msg_send![class!(NSDistributedNotificationCenter), defaultCenter];
    let () = msg_send![notification_center, removeObserver: this];
    let () = msg_send![this, dealloc];
  }
}

fn get_or_init_power_monitor_class() -> *const Class {
  static mut POWER_MONITOR_CLASS: *const Class = 0 as *const Class;
  static INIT_POWER_MONITOR_CLASS: Once = Once::new();

  INIT_POWER_MONITOR_CLASS.call_once(|| unsafe {
    println!("init POWER_MONITOR_CLASS");
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("TaoPowerMonitor", superclass).unwrap();

    decl.add_method(
      sel!(init_monitor),
      init_monitor as extern "C" fn(&Object, Sel) -> id,
    );
    decl.add_method(sel!(dealloc), dealloc as extern "C" fn(&Object, Sel));
    decl.add_method(
      sel!(onScreenLocked:),
      on_screen_locked as extern "C" fn(&Object, Sel, *mut c_void),
    );
    decl.add_method(
      sel!(onScreenUnlocked:),
      on_screen_unlocked as extern "C" fn(&Object, Sel, *mut c_void),
    );
    decl.add_method(
      sel!(onSuspend:),
      on_suspend as extern "C" fn(&Object, Sel, *mut c_void),
    );
    decl.add_method(
      sel!(onResume:),
      on_resume as extern "C" fn(&Object, Sel, *mut c_void),
    );
    POWER_MONITOR_CLASS = decl.register();
  });

  unsafe { POWER_MONITOR_CLASS }
}
