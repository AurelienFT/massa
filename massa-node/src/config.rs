// Copyright (c) 2021 MASSA LABS <info@massa.net>

use api::ApiConfig;
use bootstrap::config::BootstrapConfig;
use communication::network::NetworkConfig;
use communication::protocol::ProtocolConfig;
use consensus::ConsensusConfig;
use pool::PoolConfig;
use serde::Deserialize;
use storage::StorageConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub logging: LoggingConfig,
    pub protocol: ProtocolConfig,
    pub network: NetworkConfig,
    pub consensus: ConsensusConfig,
    pub api: ApiConfig,
    pub storage: StorageConfig,
    pub bootstrap: BootstrapConfig,
    pub pool: PoolConfig,
}

impl Config {
    /// Deserializes config.
    pub fn from_toml(toml_str: &str) -> Result<Config, toml::de::Error> {
        toml::de::from_str(toml_str)
    }
}