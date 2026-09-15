#[cfg(any(target_os = "linux", test))]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
mod linux;
#[cfg(any(target_os = "macos", test))]
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
mod macos;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("whylag supports Mac and Linux");

#[derive(Clone, Default)]
pub struct Network {
    pub name: Option<String>,
    pub signal: Option<i32>,
    pub router: Option<std::net::IpAddr>,
}
