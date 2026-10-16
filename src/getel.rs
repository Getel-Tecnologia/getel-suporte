//! Configuração embutida do fork Getel Suporte.
//!
//! Reaproveita o mecanismo de "custom client" do RustDesk, sem a verificação de assinatura.

use std::collections::HashMap;

pub const ENABLED: bool = true;
/// Nome interno (serviço, pastas, registro, exe). Só letras e números: o RustDesk monta
/// comandos do Windows com ele sem aspas. O nome exibido ("Getel Suporte") fica nos
/// metadados do exe.
pub const APP_NAME: &str = "GetelSuporte";
pub const SERVER: &str = "10.1.2.107";
pub const KEY: &str = "pWxQpuP4wMUlWS0+RVeicGAm7RtIDD5FXjy4McN+kJU=";

pub fn config() -> HashMap<String, serde_json::Value> {
    let value = serde_json::json!({
        "app-name": APP_NAME,
        "conn-type": "incoming",
        // O servidor gratuito não tem login.
        "disable-account": "Y",
        "override-settings": {
            "custom-rendezvous-server": SERVER,
            "relay-server": SERVER,
            "api-server": "",
            "key": KEY,
            "hide-network-settings": "Y",
            "hide-server-settings": "Y"
        }
    });
    let serde_json::Value::Object(map) = value else {
        unreachable!("a configuração Getel deve ser um objeto JSON");
    };
    map.into_iter().collect()
}

pub fn apply() {
    crate::common::apply_custom_client_config(config());
}

#[cfg(test)]
mod tests {
    use super::*;
    use hbb_common::config::{self, Config};

    /// Devolve os estáticos do processo ao estado de fábrica, antes de qualquer configuração.
    fn reset_to_factory() {
        *config::APP_NAME.write().unwrap() = "RustDesk".to_owned();
        config::HARD_SETTINGS.write().unwrap().clear();
        config::OVERWRITE_SETTINGS.write().unwrap().clear();
        config::BUILTIN_SETTINGS.write().unwrap().clear();
        assert!(!config::is_incoming_only());
        assert!(!crate::common::is_custom_client());
    }

    fn assert_getel_applied() {
        assert_eq!(*config::APP_NAME.read().unwrap(), APP_NAME);
        assert!(config::is_incoming_only());
        assert_eq!(Config::get_option("custom-rendezvous-server"), SERVER);
        assert_eq!(Config::get_option("relay-server"), SERVER);
        assert_eq!(Config::get_option("key"), KEY);
    }

    #[test]
    fn apply_sets_getel_client_config() {
        apply();
        assert_getel_applied();
        assert_eq!(crate::common::get_builtin_option("hide-network-settings"), "Y");
        assert_eq!(crate::common::get_builtin_option("hide-server-settings"), "Y");
        assert!(crate::common::is_custom_client());
        assert!(config::is_disable_account());
        assert_eq!(
            config::OVERWRITE_SETTINGS.read().unwrap().get("api-server"),
            Some(&String::new())
        );
        assert_eq!(Config::get_option("api-server"), "");
    }

    #[test]
    fn user_cannot_override_server() {
        apply();
        Config::set_option("custom-rendezvous-server".into(), "outro.exemplo.com".into());
        Config::set_option("key".into(), "outra-key".into());
        assert_eq!(Config::get_option("custom-rendezvous-server"), SERVER);
        assert_eq!(Config::get_option("key"), KEY);
    }

    #[test]
    fn load_custom_client_applies_getel_config() {
        reset_to_factory();
        crate::common::load_custom_client();
        assert_getel_applied();
    }

    #[test]
    fn read_custom_client_ignores_input_and_applies_getel_config() {
        reset_to_factory();
        crate::common::read_custom_client("lixo");
        assert_getel_applied();
    }

    #[cfg(windows)]
    #[test]
    fn exe_name_cannot_override_server() {
        std::env::set_var(
            crate::common::PORTABLE_APPNAME_RUNTIME_ENV_KEY,
            "getel-suporte-host=outro.exemplo.com,key=XYZ.exe",
        );
        let result = crate::platform::windows::get_license_from_exe_name();
        std::env::remove_var(crate::common::PORTABLE_APPNAME_RUNTIME_ENV_KEY);
        assert!(result.is_err());
    }
}
