//! Configuração embutida do fork Getel Suporte.
//!
//! Reaproveita o mecanismo de "custom client" do RustDesk, sem a verificação de assinatura.

use std::collections::HashMap;

pub const ENABLED: bool = true;
pub const APP_NAME: &str = "Getel Suporte";
pub const SERVER: &str = "10.1.2.107";
pub const KEY: &str = "pWxQpuP4wMUlWS0+RVeicGAm7RtIDD5FXjy4McN+kJU=";

pub fn config() -> HashMap<String, serde_json::Value> {
    let value = serde_json::json!({
        "app-name": APP_NAME,
        "conn-type": "incoming",
        "override-settings": {
            "custom-rendezvous-server": SERVER,
            "relay-server": SERVER,
            "key": KEY,
            "hide-network-settings": "Y",
            "hide-server-settings": "Y"
        }
    });
    match value {
        serde_json::Value::Object(map) => map.into_iter().collect(),
        _ => HashMap::new(),
    }
}

pub fn apply() {
    crate::common::apply_custom_client_config(config());
}

#[cfg(test)]
mod tests {
    use super::*;
    use hbb_common::config::{self, Config};

    #[test]
    fn apply_sets_getel_client_config() {
        apply();
        assert_eq!(*config::APP_NAME.read().unwrap(), "Getel Suporte");
        assert!(config::is_incoming_only());
        assert_eq!(Config::get_option("custom-rendezvous-server"), "10.1.2.107");
        assert_eq!(Config::get_option("relay-server"), "10.1.2.107");
        assert_eq!(Config::get_option("key"), "pWxQpuP4wMUlWS0+RVeicGAm7RtIDD5FXjy4McN+kJU=");
        assert_eq!(crate::common::get_builtin_option("hide-network-settings"), "Y");
        assert_eq!(crate::common::get_builtin_option("hide-server-settings"), "Y");
        assert!(crate::common::is_custom_client());
    }

    #[test]
    fn user_cannot_override_server() {
        apply();
        Config::set_option("custom-rendezvous-server".into(), "outro.exemplo.com".into());
        Config::set_option("key".into(), "outra-key".into());
        assert_eq!(Config::get_option("custom-rendezvous-server"), "10.1.2.107");
        assert_eq!(Config::get_option("key"), "pWxQpuP4wMUlWS0+RVeicGAm7RtIDD5FXjy4McN+kJU=");
    }
}
