use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// Constante para el endpoint de chat
const CHAT_ENDPOINT: &str = "https://newsapi.org/v2/everything";

#[derive(Debug, Clone)]
pub struct GraceChatConfig {
    pub api_key: String,
    pub welcome_message: String,
    pub theme: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub text: String,
    pub is_user: bool,
    pub timestamp: String,
}

#[derive(Deserialize)]
struct NewsApiResponse {
    status: String,
    #[serde(rename = "totalResults")]
    total_results: Option<i32>,
    articles: Option<Vec<Article>>,
    message: Option<String>,
}

#[derive(Deserialize)]
struct Article {
    title: String,
    description: Option<String>,
    url: String,
}

// Estructura para manejar respuestas HTTP de forma centralizada
struct HttpHandler;

impl HttpHandler {
    // Hacer una petición HTTP GET
    async fn get_request(url: &str) -> Result<Response, JsValue> {
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts)
            .map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        
        Ok(resp)
    }

    // Manejar respuestas de validación de API Key
    fn handle_validation_response(status: u16, error_text: Option<String>) -> (bool, String) {
        match status {
            200 => (true, "API Key validation: SUCCESS".to_string()),
            400 => (false, "API Key validation: FAILED - Bad Request".to_string()),
            401 => {
                let message = if let Some(text) = error_text {
                    if text.contains("apiKeyInvalid") {
                        "API Key validation: FAILED - Invalid API Key"
                    } else if text.contains("apiKeyMissing") {
                        "API Key validation: FAILED - Missing API Key"
                    } else if text.contains("apiKeyDisabled") {
                        "API Key validation: FAILED - API Key Disabled"
                    } else if text.contains("apiKeyExhausted") {
                        "API Key validation: FAILED - API Key Exhausted"
                    } else {
                        "API Key validation: FAILED - Unauthorized"
                    }
                } else {
                    "API Key validation: FAILED - Unauthorized"
                };
                (false, message.to_string())
            },
            429 => (false, "API Key validation: FAILED - Rate Limited".to_string()),
            500 => (false, "API Key validation: FAILED - Server Error".to_string()),
            _ => (false, format!("API Key validation: FAILED - Unexpected status: {}", status)),
        }
    }

    // Manejar respuestas de chat (para el usuario) - Unificado con validación
    fn handle_chat_response(status: u16, error_text: Option<String>) -> String {
        match status {
            200 => "Success".to_string(), // Este caso se maneja diferente en chat
            400 => "Lo siento, tu pregunta no es válida. ¿Podrías reformularla?".to_string(),
            401 => {
                // Podríamos hacer análisis similar al de validación si fuera necesario
                if let Some(text) = error_text {
                    if text.contains("apiKeyInvalid") || text.contains("apiKeyMissing") {
                        "Tu API key no es válida. Por favor, contacta al administrador.".to_string()
                    } else if text.contains("apiKeyDisabled") || text.contains("apiKeyExhausted") {
                        "Tu API key ha sido deshabilitada o agotada. Contacta al administrador.".to_string()
                    } else {
                        "Hay un problema con la configuración del chat. Por favor, contacta al administrador.".to_string()
                    }
                } else {
                    "Hay un problema con la configuración del chat. Por favor, contacta al administrador.".to_string()
                }
            },
            429 => "Demasiadas consultas en este momento. Por favor, espera un momento e intenta de nuevo.".to_string(),
            500 => "El servicio no está disponible en este momento. Por favor, intenta más tarde.".to_string(),
            _ => format!("Error inesperado (código {}). Por favor, intenta de nuevo.", status),
        }
    }
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
        // URL para validar la API Key con NewsAPI
        let url = format!("{}?q=bitcoin&apiKey={}", CHAT_ENDPOINT, self.api_key);
        
        let resp = HttpHandler::get_request(&url).await?;
        let status = resp.status();
        
        let (is_valid, message) = if status == 401 {
            // Para 401, intentamos leer el cuerpo de la respuesta para obtener el error específico
            let text_promise = resp.text().map_err(|_| JsValue::from_str("Failed to read response"))?;
            let text = JsFuture::from(text_promise).await?;
            let error_text = text.as_string();
            
            HttpHandler::handle_validation_response(status, error_text)
        } else {
            HttpHandler::handle_validation_response(status, None)
        };
        
        web_sys::console::log_1(&message.into());
        Ok(is_valid)
    }

    // Procesar mensaje del usuario y obtener respuesta del chat
    // GET https://newsapi.org/v2/everything?q=USER_MESSAGE&apiKey=API_KEY
    pub async fn process_chat_message(&self, user_message: &str) -> Result<String, JsValue> {
        // Validar que tenemos API key
        if self.api_key.is_empty() {
            return Err(JsValue::from_str("API key is required"));
        }

        // Crear URL con el mensaje del usuario como query
        let encoded_message = js_sys::encode_uri_component(user_message);
        let url = format!(
            "{}?q={}&apiKey={}", 
            CHAT_ENDPOINT, 
            encoded_message.as_string().unwrap_or_default(),
            self.api_key
        );
        
        web_sys::console::log_1(&format!("Chat request URL: {}", url).into());

        let resp = HttpHandler::get_request(&url).await?;
        let status = resp.status();
        
        match status {
            200 => {
                // Leer el cuerpo de la respuesta
                let text_promise = resp.text().map_err(|_| JsValue::from_str("Failed to read response"))?;
                let text = JsFuture::from(text_promise).await?;
                let json_text = text.as_string().unwrap_or_default();
                
                // Parsear la respuesta JSON
                match serde_json::from_str::<NewsApiResponse>(&json_text) {
                    Ok(news_response) => {
                        if news_response.status == "ok" {
                            self.format_news_response(&news_response)
                        } else {
                            Ok(format!("Lo siento, hubo un problema: {}", 
                                news_response.message.unwrap_or("Error desconocido".to_string())))
                        }
                    },
                    Err(_) => {
                        web_sys::console::log_1(&format!("Failed to parse JSON response: {}", json_text).into());
                        Ok("Lo siento, no pude procesar la respuesta del servidor.".to_string())
                    }
                }
            },
            _ => {
                // Usar el handler centralizado para manejar errores de chat
                Ok(HttpHandler::handle_chat_response(status, None))
            }
        }
    }

    // Formatear la respuesta de noticias en un mensaje amigable
    fn format_news_response(&self, news_response: &NewsApiResponse) -> Result<String, JsValue> {
        let total_results = news_response.total_results.unwrap_or(0);
        
        if total_results == 0 {
            return Ok("No encontré noticias relacionadas con tu consulta. ¿Podrías probar con otros términos?".to_string());
        }

        let empty_vec = vec![];
        let articles = news_response.articles.as_ref().unwrap_or(&empty_vec);
        let limited_articles = articles.iter().take(3); // Mostrar solo las primeras 3 noticias
        
        let mut response = format!("Encontré {} noticias relacionadas. Aquí están las más relevantes:\n\n", total_results);
        
        for (index, article) in limited_articles.enumerate() {
            response.push_str(&format!(
                "{}. **{}**\n{}\n[Leer más]({})\n\n",
                index + 1,
                article.title,
                article.description.as_ref().unwrap_or(&"Sin descripción disponible.".to_string()),
                article.url
            ));
        }
        
        if total_results > 3 {
            response.push_str(&format!("Y {} noticias más...", total_results - 3));
        }
        
        Ok(response)
    }
}