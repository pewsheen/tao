use crate::platform_impl::PowerMonitor as PowerMonitorImpl;

pub enum PowerState {
  Unknown,
  Suspend,
  Resume,
}

pub struct PowerMonitor(pub PowerMonitorImpl);

impl PowerMonitor {
  pub fn new() -> Self {
    Self(PowerMonitorImpl::new())
  }
}

impl Default for PowerMonitor {
  fn default() -> Self {
    Self::new()
  }
}
