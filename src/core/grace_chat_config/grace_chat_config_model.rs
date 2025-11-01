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

    // Validación real del API Key usando NewsAPI
    // GET https://newsapi.org/v2/everything?q=bitcoin&apiKey=API_KEY
    // 200 - OK
    // 400 - Bad Request
    // 401 - Unauthorized
    // 429 - Too Many Requests
    // 500 - Server Error
    /*
        apiKeyDisabled - Your API key has been disabled.
        apiKeyExhausted - Your API key has no more requests available.
        apiKeyInvalid - Your API key hasn't been entered correctly. Double check it and try again.
        apiKeyMissing - Your API key is missing from the request. Append it to the request with one of these methods.
        parameterInvalid - You've included a parameter in your request which is currently not supported. Check the message property for more details.
        parametersMissing - Required parameters are missing from the request and it cannot be completed. Check the message property for more details.
        rateLimited - You have been rate limited. Back off for a while before trying the request again.
        sourcesTooMany - You have requested too many sources in a single request. Try splitting the request into 2 smaller requests.
        sourceDoesNotExist - You have requested a source which does not exist.
        unexpectedError - This shouldn't happen, and if it does then it's our fault, not yours. Try the request again shortly.
    */
    pub async fn validate_api_key(&self) -> Result<bool, JsValue> {
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        // URL para validar la API Key con NewsAPI
        let url = format!("https://newsapi.org/v2/everything?q=bitcoin&apiKey={}", self.api_key);
        
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &opts)
            .map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        
        let status = resp.status();
        
        match status {
            200 => {
                web_sys::console::log_1(&"API Key validation: SUCCESS".into());
                Ok(true)
            },
            400 => {
                web_sys::console::log_1(&"API Key validation: FAILED - Bad Request".into());
                Ok(false)
            },
            401 => {
                // Intentamos obtener el error específico del cuerpo de la respuesta
                let text_promise = resp.text().map_err(|_| JsValue::from_str("Failed to read response"))?;
                let text = JsFuture::from(text_promise).await?;
                let error_text = text.as_string().unwrap_or("Unknown error".to_string());
                
                if error_text.contains("apiKeyInvalid") {
                    web_sys::console::log_1(&"API Key validation: FAILED - Invalid API Key".into());
                } else if error_text.contains("apiKeyMissing") {
                    web_sys::console::log_1(&"API Key validation: FAILED - Missing API Key".into());
                } else if error_text.contains("apiKeyDisabled") {
                    web_sys::console::log_1(&"API Key validation: FAILED - API Key Disabled".into());
                } else if error_text.contains("apiKeyExhausted") {
                    web_sys::console::log_1(&"API Key validation: FAILED - API Key Exhausted".into());
                } else {
                    web_sys::console::log_1(&"API Key validation: FAILED - Unauthorized".into());
                }
                Ok(false)
            },
            429 => {
                web_sys::console::log_1(&"API Key validation: FAILED - Rate Limited".into());
                Ok(false)
            },
            500 => {
                web_sys::console::log_1(&"API Key validation: FAILED - Server Error".into());
                Ok(false)
            },
            _ => {
                web_sys::console::log_1(&format!("API Key validation: FAILED - Unexpected status: {}", status).into());
                Ok(false)
            }
        }
    }
}