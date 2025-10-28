use cfg_if::cfg_if;

pub mod auth;
pub mod credential;
pub mod error;
pub mod gateway;
pub mod gp_params;
pub mod portal;
pub mod process;
pub mod service;
pub mod utils;

#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "clap")]
pub mod clap;

#[cfg(debug_assertions)]
pub const GP_API_KEY: &[u8; 32] = &[0; 32];

pub const GP_USER_AGENT: &str = "PAN GlobalProtect";
pub const GP_SERVICE_LOCK_FILE: &str = "/var/run/gpservice.lock";
pub const GP_CALLBACK_PORT_FILENAME: &str = "gpcallback.port";

cfg_if! {
    if #[cfg(feature = "gp_binaries_on_path")] {
        pub const GP_CLIENT_BINARY: &str = "gpclient";
        pub const GP_SERVICE_BINARY: &str = "gpservice";
        pub const GP_GUI_BINARY: &str = "gpgui";
        pub const GP_GUI_HELPER_BINARY: &str = "gpgui-helper";
        pub(crate) const GP_AUTH_BINARY: &str = "gpauth";
    } else if #[cfg(debug_assertions)] {
        pub const GP_CLIENT_BINARY: &str = env!("GP_CLIENT_BINARY");
        pub const GP_SERVICE_BINARY: &str = env!("GP_SERVICE_BINARY");
        pub const GP_GUI_BINARY: &str = env!("GP_GUI_BINARY");
        pub const GP_GUI_HELPER_BINARY: &str = env!("GP_GUI_HELPER_BINARY");
        pub(crate) const GP_AUTH_BINARY: &str = env!("GP_AUTH_BINARY");
    } else {
        pub const GP_CLIENT_BINARY: &str = "/usr/bin/gpclient";
        pub const GP_SERVICE_BINARY: &str = "/usr/bin/gpservice";
        pub const GP_GUI_BINARY: &str = "/usr/bin/gpgui";
        pub const GP_GUI_HELPER_BINARY: &str = "/usr/bin/gpgui-helper";
        pub(crate) const GP_AUTH_BINARY: &str = "/usr/bin/gpauth";
    }
}
