use wasm_bindgen::prelude::*;
use web_sys::{WebSocket, MessageEvent, CloseEvent, ErrorEvent};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketMessage {
    // Mensajes del chat
    ChatMessage {
        id: String,
        content: String,
        user_id: String,
        timestamp: u64,
    },
    // Estados del usuario
    UserTyping {
        user_id: String,
        is_typing: bool,
    },
    // Estado de conexión
    Connected {
        user_id: String,
        session_id: String,
    },
    // Presencia de usuarios
    UserPresence {
        user_id: String,
        status: String, // "online", "away", "offline"
    },
    // Respuestas del servidor
    ServerResponse {
        message_id: String,
        status: String,
        data: Option<String>,
    },
    // Errores
    Error {
        code: String,
        message: String,
    },
}

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error(String),
}

// Callback traits para diferentes eventos
type OnMessageCallback = Box<dyn FnMut(WebSocketMessage)>;
type OnStateChangeCallback = Box<dyn FnMut(ConnectionState)>;
type OnErrorCallback = Box<dyn FnMut(String)>;

pub struct WebSocketManager {
    websocket: Option<WebSocket>,
    url: String,
    connection_state: Rc<RefCell<ConnectionState>>,
    auto_reconnect: bool,
    reconnect_interval: u32, // milliseconds
    max_reconnect_attempts: u32,
    current_reconnect_attempts: u32,
    
    // Callbacks
    on_message: Option<OnMessageCallback>,
    on_state_change: Option<OnStateChangeCallback>,
    on_error: Option<OnErrorCallback>,
    
    // Session info
    user_id: Option<String>,
    session_id: Option<String>,
}

impl WebSocketManager {
    pub fn new(url: String) -> Self {
        Self {
            websocket: None,
            url,
            connection_state: Rc::new(RefCell::new(ConnectionState::Disconnected)),
            auto_reconnect: true,
            reconnect_interval: 3000, // 3 seconds
            max_reconnect_attempts: 5,
            current_reconnect_attempts: 0,
            on_message: None,
            on_state_change: None,
            on_error: None,
            user_id: None,
            session_id: None,
        }
    }

    // Configurar callbacks
    pub fn on_message<F>(&mut self, callback: F) 
    where 
        F: FnMut(WebSocketMessage) + 'static 
    {
        self.on_message = Some(Box::new(callback));
    }

    pub fn on_state_change<F>(&mut self, callback: F) 
    where 
        F: FnMut(ConnectionState) + 'static 
    {
        self.on_state_change = Some(Box::new(callback));
    }

    pub fn on_error<F>(&mut self, callback: F) 
    where 
        F: FnMut(String) + 'static 
    {
        self.on_error = Some(Box::new(callback));
    }

    // Conectar al WebSocket
    pub async fn connect(&mut self, user_id: String) -> Result<(), JsValue> {
        self.user_id = Some(user_id.clone());
        self.set_connection_state(ConnectionState::Connecting);
        
        web_sys::console::log_1(&format!("Connecting to WebSocket: {}", self.url).into());
        
        // Crear WebSocket
        let ws = WebSocket::new(&self.url)
            .map_err(|e| JsValue::from_str(&format!("Failed to create WebSocket: {:?}", e)))?;
        
        // Configurar event listeners
        self.setup_event_listeners(&ws).await?;
        
        self.websocket = Some(ws);
        Ok(())
    }

    // Configurar event listeners
    async fn setup_event_listeners(&mut self, ws: &WebSocket) -> Result<(), JsValue> {
        let state_ref = self.connection_state.clone();
        
        // OnOpen
        {
            let state_ref = state_ref.clone();
            let user_id = self.user_id.clone();
            let ws_clone = ws.clone();
            
            let onopen_callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                web_sys::console::log_1(&"WebSocket connection opened".into());
                
                // Cambiar estado a conectado
                *state_ref.borrow_mut() = ConnectionState::Connected;
                
                // Enviar mensaje de conexión si tenemos user_id
                if let Some(ref user_id) = user_id {
                    let connect_msg = WebSocketMessage::Connected {
                        user_id: user_id.clone(),
                        session_id: js_sys::Date::now().to_string(), // Simple session ID
                    };
                    
                    if let Ok(json) = serde_json::to_string(&connect_msg) {
                        let _ = ws_clone.send_with_str(&json);
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            onopen_callback.forget();
        }

        // OnMessage
        {
            let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
                web_sys::console::log_1(&"WebSocket message received".into());
                
                if let Ok(text) = event.data().dyn_into::<js_sys::JsString>() {
                    let message_str = text.as_string().unwrap_or_default();
                    
                    // Intentar parsear el mensaje
                    match serde_json::from_str::<WebSocketMessage>(&message_str) {
                        Ok(ws_message) => {
                            web_sys::console::log_1(&format!("Parsed message: {:?}", ws_message).into());
                            // Aquí llamaríamos al callback on_message si estuviera configurado
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("Failed to parse message: {}", e).into());
                        }
                    }
                } else {
                    web_sys::console::log_1(&"Received non-text message".into());
                }
            }) as Box<dyn FnMut(_)>);
            
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();
        }

        // OnError
        {
            let state_ref = state_ref.clone();
            
            let onerror_callback = Closure::wrap(Box::new(move |event: ErrorEvent| {
                let error_msg = format!("WebSocket error: {:?}", event);
                web_sys::console::log_1(&error_msg.clone().into());
                
                *state_ref.borrow_mut() = ConnectionState::Error(error_msg);
            }) as Box<dyn FnMut(_)>);
            
            ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            onerror_callback.forget();
        }

        // OnClose
        {
            let state_ref = state_ref.clone();
            let auto_reconnect = self.auto_reconnect;
            
            let onclose_callback = Closure::wrap(Box::new(move |event: CloseEvent| {
                let close_msg = format!("WebSocket closed: code={}, reason={}", event.code(), event.reason());
                web_sys::console::log_1(&close_msg.into());
                
                *state_ref.borrow_mut() = ConnectionState::Disconnected;
                
                // Auto-reconnect logic aquí si es necesario
                if auto_reconnect && event.code() != 1000 { // 1000 = normal closure
                    web_sys::console::log_1(&"Will attempt to reconnect...".into());
                    // Implementar lógica de reconexión
                }
            }) as Box<dyn FnMut(_)>);
            
            ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
            onclose_callback.forget();
        }

        Ok(())
    }

    // Enviar mensaje
    pub fn send_message(&self, message: WebSocketMessage) -> Result<(), JsValue> {
        if let Some(ref ws) = self.websocket {
            match ws.ready_state() {
                WebSocket::OPEN => {
                    let json = serde_json::to_string(&message)
                        .map_err(|e| JsValue::from_str(&format!("Failed to serialize message: {}", e)))?;
                    
                    ws.send_with_str(&json)
                        .map_err(|e| JsValue::from_str(&format!("Failed to send message: {:?}", e)))?;
                    
                    web_sys::console::log_1(&"Message sent successfully".into());
                    Ok(())
                }
                _ => {
                    Err(JsValue::from_str("WebSocket is not connected"))
                }
            }
        } else {
            Err(JsValue::from_str("WebSocket not initialized"))
        }
    }

    // Enviar mensaje de chat
    pub fn send_chat_message(&self, content: String) -> Result<(), JsValue> {
        if let Some(ref user_id) = self.user_id {
            let message = WebSocketMessage::ChatMessage {
                id: format!("{}-{}", user_id, js_sys::Date::now()),
                content,
                user_id: user_id.clone(),
                timestamp: js_sys::Date::now() as u64,
            };
            
            self.send_message(message)
        } else {
            Err(JsValue::from_str("User ID not set"))
        }
    }

    // Indicar que el usuario está escribiendo
    pub fn send_typing_indicator(&self, is_typing: bool) -> Result<(), JsValue> {
        if let Some(ref user_id) = self.user_id {
            let message = WebSocketMessage::UserTyping {
                user_id: user_id.clone(),
                is_typing,
            };
            
            self.send_message(message)
        } else {
            Err(JsValue::from_str("User ID not set"))
        }
    }

    // Desconectar
    pub fn disconnect(&mut self) -> Result<(), JsValue> {
        if let Some(ref ws) = self.websocket {
            ws.close().map_err(|e| JsValue::from_str(&format!("Failed to close WebSocket: {:?}", e)))?;
            self.websocket = None;
            self.set_connection_state(ConnectionState::Disconnected);
        }
        Ok(())
    }

    // Obtener estado de conexión
    pub fn get_connection_state(&self) -> ConnectionState {
        self.connection_state.borrow().clone()
    }

    // Cambiar estado de conexión
    fn set_connection_state(&mut self, state: ConnectionState) {
        *self.connection_state.borrow_mut() = state.clone();
        
        // Llamar callback si existe
        if let Some(ref mut callback) = self.on_state_change {
            callback(state);
        }
    }

    // Verificar si está conectado
    pub fn is_connected(&self) -> bool {
        matches!(*self.connection_state.borrow(), ConnectionState::Connected)
    }

    // Configurar opciones de reconexión
    pub fn set_reconnect_options(&mut self, auto_reconnect: bool, interval_ms: u32, max_attempts: u32) {
        self.auto_reconnect = auto_reconnect;
        self.reconnect_interval = interval_ms;
        self.max_reconnect_attempts = max_attempts;
    }
}

// Drop trait para limpiar recursos
impl Drop for WebSocketManager {
    fn drop(&mut self) {
        let _ = self.disconnect();
    }
}