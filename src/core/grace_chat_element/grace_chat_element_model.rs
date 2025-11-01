use wasm_bindgen::prelude::*;
use web_sys::*;
use crate::core::grace_chat_config::grace_chat_config_model::GraceChatConfig;

// Web Component principal
#[wasm_bindgen]
pub struct GraceChatElement {
    element: HtmlElement,
    config: Option<GraceChatConfig>,
    initialized: bool,
}

#[wasm_bindgen]
impl GraceChatElement {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GraceChatElement, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.create_element("div")?.dyn_into::<HtmlElement>()?;
        
        Ok(GraceChatElement {
            element,
            config: None,
            initialized: false,
        })
    }

    pub fn connected_callback(&mut self, element: HtmlElement) -> Result<(), JsValue> {
        self.element = element;
        self.extract_attributes()?;
        self.init_chat()?;
        Ok(())
    }

    fn extract_attributes(&mut self) -> Result<(), JsValue> {
        let api_key = self.element.get_attribute("api-key").unwrap_or_default();
        let welcome = self.element.get_attribute("welcome").unwrap_or("¡Hola! ¿En qué te puedo ayudar?".to_string());
        let theme = self.element.get_attribute("theme").unwrap_or("light".to_string());

        if api_key.is_empty() {
            return Err(JsValue::from_str("API Key is required"));
        }

        self.config = Some(GraceChatConfig::new(api_key, welcome, theme));
        Ok(())
    }

    fn init_chat(&mut self) -> Result<(), JsValue> {
        if let Some(config) = &self.config {
            // Inyectar estilos
            self.inject_styles()?;
            
            // Crear la estructura HTML del chat
            self.create_chat_structure(config)?;
            
            // Validar API Key de forma asíncrona
            self.validate_and_show_chat();
            
            self.initialized = true;
        }
        Ok(())
    }

    fn inject_styles(&self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let head = document.head().unwrap();

        // Verificar si ya existen los estilos
        if document.get_element_by_id("grace-chat-styles").is_some() {
            return Ok(());
        }

        let style_element = document.create_element("style")?;
        style_element.set_id("grace-chat-styles");
        
        // CSS embebido minificado con nombres de Grace
        let css = r#"
:root{--grace-primary-color:#007bff;--grace-success-color:#28a745;--grace-error-color:#dc3545;--grace-dark-bg:#2c3e50;--grace-dark-text:#ecf0f1;--grace-light-bg:#ffffff;--grace-light-text:#333333;--grace-border-radius:12px;--grace-shadow:0 4px 20px rgba(0,0,0,0.15);--grace-animation-duration:0.3s;--grace-z-index:999999}
.grace-chat-container{position:fixed;bottom:20px;right:20px;width:350px;max-width:calc(100vw - 40px);max-height:calc(100vh - 40px);border-radius:var(--grace-border-radius);box-shadow:var(--grace-shadow);font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,'Helvetica Neue',Arial,sans-serif;z-index:var(--grace-z-index);overflow:hidden;transition:all var(--grace-animation-duration) ease-in-out;background-color:var(--grace-light-bg);color:var(--grace-light-text);border:1px solid #e1e8ed}
.grace-chat-container--minimized{height:60px}
.grace-chat-container--minimized .grace-chat-body,.grace-chat-container--minimized .grace-chat-status,.grace-chat-container--minimized .grace-chat-input-container{display:none}
.grace-chat-container--minimized .grace-chat-minimize{transform:rotate(180deg)}
.grace-chat-container.grace-chat--dark{background-color:var(--grace-dark-bg);color:var(--grace-dark-text);border-color:#34495e}
.grace-chat-container.grace-chat--dark .grace-chat-header{background-color:#263544}
.grace-chat-container.grace-chat--dark .grace-chat-message--bot .grace-chat-message-content{background-color:#3e5771;color:var(--grace-dark-text)}
.grace-chat-header{display:flex;align-items:center;justify-content:space-between;padding:15px 20px;background-color:var(--grace-primary-color);color:white;font-weight:600}
.grace-chat--dark .grace-chat-header{background-color:#263544}
.grace-chat-title{font-size:clamp(14px,2.5vw,16px);line-height:1.4;font-weight:600}
.grace-chat-minimize{background:none;border:none;color:white;font-size:20px;font-weight:bold;cursor:pointer;padding:5px 8px;border-radius:4px;transition:all 0.2s ease}
.grace-chat-minimize:hover{background-color:rgba(255,255,255,0.1)}
.grace-chat-minimize:focus{outline:2px solid rgba(255,255,255,0.3);outline-offset:2px}
.grace-chat-body{padding:20px;min-height:200px;max-height:400px;overflow-y:auto}
.grace-chat-messages{min-height:100%}
.grace-chat-body::-webkit-scrollbar{width:6px}
.grace-chat-body::-webkit-scrollbar-track{background:transparent}
.grace-chat-body::-webkit-scrollbar-thumb{background:rgba(0,0,0,0.2);border-radius:3px}
.grace-chat-body::-webkit-scrollbar-thumb:hover{background:rgba(0,0,0,0.3)}
.grace-chat--dark .grace-chat-body::-webkit-scrollbar-thumb{background:rgba(255,255,255,0.2)}
.grace-chat--dark .grace-chat-body::-webkit-scrollbar-thumb:hover{background:rgba(255,255,255,0.3)}
.grace-chat-message{margin-bottom:15px;animation:fadeInUp var(--grace-animation-duration) ease-out}
.grace-chat-message--bot .grace-chat-message-content{background-color:#f8f9fa;color:var(--grace-light-text);border-radius:var(--grace-border-radius);padding:12px 16px;max-width:85%;font-size:clamp(14px,2.5vw,16px);line-height:1.4}
.grace-chat-message--user{text-align:right}
.grace-chat-message--user .grace-chat-message-content{background-color:var(--grace-primary-color);color:white;border-radius:var(--grace-border-radius);padding:12px 16px;max-width:85%;margin-left:auto;font-size:clamp(14px,2.5vw,16px);line-height:1.4}
.grace-chat-input-container{display:flex;align-items:center;padding:15px 20px;border-top:1px solid #e1e8ed;gap:10px}
.grace-chat--dark .grace-chat-input-container{border-top-color:#34495e}
.grace-chat-input{flex:1;padding:10px 15px;border:1px solid #e1e8ed;border-radius:20px;font-size:14px;outline:none;background-color:var(--grace-light-bg);color:var(--grace-light-text);transition:border-color 0.2s ease}
.grace-chat-input:focus{border-color:var(--grace-primary-color)}
.grace-chat--dark .grace-chat-input{background-color:#34495e;border-color:#4a6174;color:var(--grace-dark-text)}
.grace-chat--dark .grace-chat-input:focus{border-color:#5dade2}
.grace-chat-send-btn{padding:10px 15px;border:none;border-radius:20px;background-color:var(--grace-primary-color);color:white;cursor:pointer;font-size:16px;font-weight:bold;transition:all 0.2s ease;min-width:44px;display:flex;align-items:center;justify-content:center}
.grace-chat-send-btn:hover{background-color:#0056b3;transform:scale(1.05)}
.grace-chat-send-btn:active{transform:scale(0.95)}
.grace-chat-send-btn:disabled{background-color:#6c757d;cursor:not-allowed;transform:none}
.grace-chat-typing-indicator{font-size:12px;color:#6c757d;font-style:italic;padding:10px 0;animation:fadeInUp var(--grace-animation-duration) ease-out}
.grace-chat-status{padding:10px 20px;border-top:1px solid #e1e8ed;font-size:12px;text-align:center;transition:all var(--grace-animation-duration) ease}
.grace-chat--dark .grace-chat-status{border-top-color:#34495e}
.grace-chat-status--success{background-color:#d4edda;color:#155724}
.grace-chat--dark .grace-chat-status--success{background-color:#0f5132;color:#75b798}
.grace-chat-status--error{background-color:#f8d7da;color:#721c24}
.grace-chat--dark .grace-chat-status--error{background-color:#58151c;color:#ea868f}
.grace-chat-status-text{font-weight:500}
@keyframes fadeInUp{from{opacity:0;transform:translateY(10px)}to{opacity:1;transform:translateY(0)}}
@media (max-width:480px){.grace-chat-container{width:calc(100vw - 20px);bottom:10px;right:10px;left:10px}.grace-chat-container--minimized{height:50px}.grace-chat-header{padding:12px 15px}.grace-chat-body{padding:15px;min-height:150px;max-height:calc(100vh - 200px)}.grace-chat-title{font-size:14px}.grace-chat-input-container{padding:12px 15px}}
@media (max-width:320px){.grace-chat-container{width:calc(100vw - 10px);bottom:5px;right:5px;left:5px}.grace-chat-message-content{font-size:13px!important;padding:10px 12px}.grace-chat-input{font-size:13px}.grace-chat-send-btn{min-width:40px;font-size:14px}}
@media (prefers-reduced-motion:reduce){.grace-chat-container,.grace-chat-message,.grace-chat-minimize,.grace-chat-send-btn{animation:none;transition:none}}
@media (prefers-contrast:high){.grace-chat-container{border:2px solid currentColor}.grace-chat-message-content{border:1px solid currentColor}.grace-chat-input{border:2px solid currentColor}}
        "#;
        
        style_element.set_text_content(Some(css));
        head.append_child(&style_element)?;
        Ok(())
    }

    fn create_chat_structure(&self, config: &GraceChatConfig) -> Result<(), JsValue> {
        let theme_class = format!("grace-chat--{}", config.theme);
        
        let html = format!(
            r#"
            <div class="grace-chat-container {}">
                <div class="grace-chat-header">
                    <span class="grace-chat-title">Grace</span>
                    <button class="grace-chat-minimize" type="button">−</button>
                </div>
                <div class="grace-chat-body">
                    <div class="grace-chat-messages">
                        <div class="grace-chat-message grace-chat-message--bot">
                            <div class="grace-chat-message-content">
                                {}
                            </div>
                        </div>
                    </div>
                </div>
                <div class="grace-chat-input-container">
                    <input 
                        type="text" 
                        class="grace-chat-input" 
                        placeholder="Escribe tu mensaje..."
                        maxlength="500"
                    />
                    <button class="grace-chat-send-btn" type="button">
                        <span>→</span>
                    </button>
                </div>
                <div class="grace-chat-status">
                    <span class="grace-chat-status-text">Validando...</span>
                </div>
            </div>
            "#,
            theme_class,
            config.welcome_message
        );

        self.element.set_inner_html(&html);
        self.setup_event_listeners()?;
        
        Ok(())
    }

    fn setup_event_listeners(&self) -> Result<(), JsValue> {
        let minimize_btn = self.element.query_selector(".grace-chat-minimize")?;
        
        if let Some(btn) = minimize_btn {
            let container = self.element.query_selector(".grace-chat-container")?;
            
            if let Some(container_elem) = container {
                let closure = Closure::wrap(Box::new(move |_: Event| {
                    let class_list = container_elem.class_list();
                    if class_list.contains("grace-chat-container--minimized") {
                        let _ = class_list.remove_1("grace-chat-container--minimized");
                    } else {
                        let _ = class_list.add_1("grace-chat-container--minimized");
                    }
                }) as Box<dyn FnMut(_)>);
                
                btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
                closure.forget();
            }
        }

        // Setup para el envío de mensajes
        self.setup_message_sending()?;
        
        Ok(())
    }

    fn setup_message_sending(&self) -> Result<(), JsValue> {
        let send_btn = self.element.query_selector(".grace-chat-send-btn")?;
        let input = self.element.query_selector(".grace-chat-input")?;
        
        if let (Some(btn), Some(input_elem)) = (send_btn, input) {
            let element_clone = self.element.clone();
            let config_clone = self.config.clone();
            
            // Manejar click del botón enviar
            let click_closure = Closure::wrap(Box::new(move |_: Event| {
                let input_element = input_elem.dyn_ref::<HtmlInputElement>().unwrap();
                let message = input_element.value().trim().to_string();
                
                if !message.is_empty() {
                    // Limpiar input
                    input_element.set_value("");
                    
                    // Enviar mensaje
                    if let Some(config) = &config_clone {
                        Self::send_message_static(element_clone.clone(), config.clone(), message);
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            btn.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
            click_closure.forget();
            
            // Manejar Enter en el input
            let input_clone = input_elem.clone();
            let element_clone2 = self.element.clone();
            let config_clone2 = self.config.clone();
            
            let keypress_closure = Closure::wrap(Box::new(move |event: Event| {
                let keyboard_event = event.dyn_ref::<KeyboardEvent>().unwrap();
                if keyboard_event.key() == "Enter" {
                    let input_element = input_clone.dyn_ref::<HtmlInputElement>().unwrap();
                    let message = input_element.value().trim().to_string();
                    
                    if !message.is_empty() {
                        input_element.set_value("");
                        
                        if let Some(config) = &config_clone2 {
                            Self::send_message_static(element_clone2.clone(), config.clone(), message);
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            input_elem.add_event_listener_with_callback("keypress", keypress_closure.as_ref().unchecked_ref())?;
            keypress_closure.forget();
        }
        
        Ok(())
    }

    fn send_message_static(element: HtmlElement, config: GraceChatConfig, message: String) {
        // Agregar mensaje del usuario
        Self::add_message_to_chat(&element, &message, true);
        
        // Mostrar typing indicator
        Self::show_typing_indicator(&element);
        
        // Procesar mensaje de forma asíncrona
        wasm_bindgen_futures::spawn_local(async move {
            match config.process_chat_message(&message).await {
                Ok(response) => {
                    // Ocultar typing indicator
                    Self::hide_typing_indicator(&element);
                    
                    // Agregar respuesta del bot
                    Self::add_message_to_chat(&element, &response, false);
                }
                Err(error) => {
                    Self::hide_typing_indicator(&element);
                    
                    let error_msg = error.as_string().unwrap_or("Error procesando mensaje".to_string());
                    Self::add_message_to_chat(&element, &error_msg, false);
                    
                    web_sys::console::log_1(&format!("Error processing message: {}", error_msg).into());
                }
            }
        });
    }

    fn add_message_to_chat(element: &HtmlElement, message: &str, is_user: bool) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        if let Ok(Some(messages_container)) = element.query_selector(".grace-chat-messages") {
            let message_div = document.create_element("div").unwrap();
            let message_class = if is_user { "grace-chat-message grace-chat-message--user" } else { "grace-chat-message grace-chat-message--bot" };
            message_div.set_class_name(message_class);
            
            let content_div = document.create_element("div").unwrap();
            content_div.set_class_name("grace-chat-message-content");
            content_div.set_text_content(Some(message));
            
            message_div.append_child(&content_div).unwrap();
            messages_container.append_child(&message_div).unwrap();
            
            // Scroll hacia abajo
            messages_container.set_scroll_top(messages_container.scroll_height());
        }
    }

    fn show_typing_indicator(element: &HtmlElement) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        if let Ok(Some(messages_container)) = element.query_selector(".grace-chat-messages") {
            let typing_div = document.create_element("div").unwrap();
            typing_div.set_class_name("grace-chat-typing-indicator");
            typing_div.set_id("grace-typing-indicator");
            typing_div.set_text_content(Some("Escribiendo..."));
            
            messages_container.append_child(&typing_div).unwrap();
            messages_container.set_scroll_top(messages_container.scroll_height());
        }
    }

    fn hide_typing_indicator(element: &HtmlElement) {
        if let Ok(Some(typing_indicator)) = element.query_selector("#grace-typing-indicator") {
            typing_indicator.remove();
        }
    }

    fn validate_and_show_chat(&self) {
        if let Some(config) = &self.config {
            let config_clone = GraceChatConfig::new(
                config.api_key.clone(),
                config.welcome_message.clone(),
                config.theme.clone()
            );
            
            let element = self.element.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match config_clone.validate_api_key().await {
                    Ok(is_valid) => {
                        let status_elem = element.query_selector(".grace-chat-status-text").unwrap();
                        if let Some(status) = status_elem {
                            if is_valid {
                                status.set_text_content(Some("Conectado"));
                                if let Some(status_container) = element.query_selector(".grace-chat-status").unwrap() {
                                    let _ = status_container.class_list().add_1("grace-chat-status--success");
                                }
                            } else {
                                status.set_text_content(Some("API Key inválido"));
                                if let Some(status_container) = element.query_selector(".grace-chat-status").unwrap() {
                                    let _ = status_container.class_list().add_1("grace-chat-status--error");
                                }
                            }
                        }
                    }
                    Err(_) => {
                        web_sys::console::log_1(&"Error validating API key".into());
                    }
                }
            });
        }
    }
}