use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct GraceChatConfig {
    pub api_key: String,
    pub welcome_message: String,
    pub theme: String,
}

impl GraceChatConfig {
    pub fn new(api_key: String, welcome_message: String, theme: String) -> Self {
        Self {
            api_key,
            welcome_message,
            theme,
        }
    }

    // Validación ficticia del API Key por ahora
    pub async fn validate_api_key(&self) -> Result<bool, JsValue> {
        // Simulamos una validación - en el futuro esto hará una llamada real
        let is_valid = self.api_key.starts_with("pk_test_") || self.api_key.starts_with("pk_live_");
        
        if is_valid {
            web_sys::console::log_1(&"API Key validation: SUCCESS".into());
        } else {
            web_sys::console::log_1(&"API Key validation: FAILED".into());
        }
        
        Ok(is_valid)
    }
}