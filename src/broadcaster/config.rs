use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum BroadcasterConfig {
    Udp(UdpConfig),
    Com(ComConfig),
}

#[derive(Debug, Clone, Deserialize)]
pub struct UdpConfig {
    pub port: u16,
    pub netmask: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ComConfig {
    pub port: String,
    pub baud_rate: u32,
}
