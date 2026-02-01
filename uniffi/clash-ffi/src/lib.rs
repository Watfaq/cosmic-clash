use clash_lib::app::dns;

use clash_lib::app::dns::config::{DNSListenAddr, DNSNetMode, NameServer};
use clash_lib::config::def::{DNSMode, Port};
use clash_lib::{
    Config,
    config::{config::Controller, def::LogLevel},
    start,
};
use std::path::PathBuf;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Once,
};

use log::init_logger;
use tokio::{sync::broadcast, task::JoinHandle};
use tracing::{error, info};

use clash_lib::config::def::Config as ConfigDef;

pub mod controller;
pub mod log;
pub mod util;

// Error type that can be used with uniffi
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ClashError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

impl From<eyre::Error> for ClashError {
    fn from(err: eyre::Error) -> Self {
        ClashError::RuntimeError(format!("{:#}", err))
    }
}

impl From<std::io::Error> for ClashError {
    fn from(err: std::io::Error) -> Self {
        ClashError::IoError(err.to_string())
    }
}

impl From<std::net::AddrParseError> for ClashError {
    fn from(err: std::net::AddrParseError) -> Self {
        ClashError::ParseError(err.to_string())
    }
}

impl From<ipnet::PrefixLenError> for ClashError {
    fn from(err: ipnet::PrefixLenError) -> Self {
        ClashError::ParseError(err.to_string())
    }
}

impl From<ipnet::AddrParseError> for ClashError {
    fn from(err: ipnet::AddrParseError) -> Self {
        ClashError::ParseError(err.to_string())
    }
}

impl From<serde_json::Error> for ClashError {
    fn from(err: serde_json::Error) -> Self {
        ClashError::ParseError(err.to_string())
    }
}

impl From<clash_lib::Error> for ClashError {
    fn from(err: clash_lib::Error) -> Self {
        ClashError::RuntimeError(format!("{:#}", err))
    }
}

#[derive(uniffi::Record)]
pub struct ProfileOverride {
    #[uniffi(default = false)]
    pub allow_lan: bool,

    #[uniffi(default = 7890)]
    pub mixed_port: u16,
    
    #[uniffi(default = None)]
    pub http_port: Option<u16>,
    
    #[uniffi(default = None)]
    pub socks_port: Option<u16>,
    
    #[uniffi(default = false)]
    pub fake_ip: bool,

    #[uniffi(default = "198.18.0.2/16")]
    pub fake_ip_range: String,

    #[uniffi(default = true)]
    pub ipv6: bool,
}

#[derive(uniffi::Record, Default)]
pub struct FinalProfile {
    #[uniffi(default = 7890)]
    pub mixed_port: u16,
}

#[uniffi::export]
pub fn init_clash(log_level: String) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let level = match log_level.to_lowercase().as_str() {
            "trace" => LogLevel::Trace,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warning,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        };

        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1");
        }
        
        init_logger(level.into());
        color_eyre::install().unwrap();

        // Install aws-lc-rs as the default crypto provider
        if let Err(e) = rustls::crypto::aws_lc_rs::default_provider().install_default() {
            error!("Failed to install default crypto provider: {:?}", e);
        } else {
            info!("Successfully installed aws-lc-rs crypto provider");
        }
        info!("Clash initialized with log level: {:?}", level);
    });
}

#[uniffi::export]
fn verify_config(config_path: &str) -> Result<String, ClashError> {
    let _config = Config::File(config_path.to_string()).try_parse()?;
    Ok(format!("Configuration loaded successfully for: {}", config_path))
}

#[uniffi::export(async_runtime = "tokio")]
async fn run_clash(
    config_path: String,
    work_dir: String,
    over: ProfileOverride,
) -> Result<FinalProfile, ClashError> {
    std::env::set_current_dir(&work_dir)?;
    let mut final_profile = FinalProfile::default();
    let mut config_def = ConfigDef::try_from(PathBuf::from(config_path.clone()))?;
    final_profile.mixed_port = config_def.mixed_port.get_or_insert(Port(over.mixed_port)).0;
    config_def.port = config_def.port.or_else(|| over.http_port.map(Port));
    config_def.socks_port = config_def.socks_port.or_else(|| over.socks_port.map(Port));

    let mut config = Config::Def(config_def).try_parse()?;

    config.general.geosite = Some("geosite.dat".to_string());
    config.general.mmdb = Some("Country.mmdb".to_string());
    config.general.asn_mmdb = None;

    config.general.controller = Controller {
        external_controller_ipc: Some(format!("{work_dir}/clash.sock")),
        ..Default::default()
    };

    config.general.ipv6 = over.ipv6;

    let nameserver = if config.dns.nameserver.is_empty() {
        vec![
            NameServer {
                net: DNSNetMode::DoH,
                address: "223.5.5.5:443".to_string(),
                interface: None,
                proxy: None,
            },
            NameServer {
                net: DNSNetMode::DoH,
                address: "223.6.6.6:443".to_string(),
                interface: None,
                proxy: None,
            },
            NameServer {
                net: DNSNetMode::DoH,
                address: "120.53.53.53:443".to_string(),
                interface: None,
                proxy: None,
            },
            NameServer {
                net: DNSNetMode::DoH,
                address: "1.12.12.12:443".to_string(),
                interface: None,
                proxy: None,
            },
        ]
    } else {
        config.dns.nameserver.clone()
    };
    
    let default_nameserver = if config.dns.default_nameserver.is_empty() {
        vec![
            NameServer {
                net: DNSNetMode::Udp,
                address: "223.6.6.6:53".to_string(),
                interface: None,
                proxy: None,
            },
            NameServer {
                net: DNSNetMode::Udp,
                address: "8.8.8.8:53".to_string(),
                interface: None,
                proxy: None,
            },
        ]
    } else {
        config.dns.default_nameserver.clone()
    };

    config.dns = dns::Config {
        enable: true,
        ipv6: over.ipv6,
        listen: DNSListenAddr {
            udp: Some(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                53553,
            )),
            ..config.dns.listen
        },
        nameserver,
        default_nameserver,
        ..config.dns
    };
    
    if over.fake_ip {
        config.dns.enhance_mode = DNSMode::FakeIp;
        config.dns.fake_ip_range = over.fake_ip_range.parse()?;
    } else {
        config.dns.enhance_mode = DNSMode::Normal;
    }

    info!("Config path: {config_path}");

    let _: JoinHandle<eyre::Result<()>> = tokio::spawn(async {
        let (log_tx, _) = broadcast::channel(100);
        info!("Starting clash-rs");
        if let Err(err) = start(config, work_dir, log_tx).await {
            error!("clash-rs start error: {:#}", eyre::eyre!(err));
        }

        info!("Quitting clash-rs");
        Ok(())
    });
    
    Ok(final_profile)
}

#[uniffi::export]
fn shutdown() {
    clash_lib::shutdown();
    info!("clash shutdown");
}

uniffi::setup_scaffolding!("clash_ffi");
