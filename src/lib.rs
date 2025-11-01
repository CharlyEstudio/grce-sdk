use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod core;

// Configuración de panic para WASM
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

// Función para registrar el Web Component
#[wasm_bindgen]
pub fn register_grace_chat() -> Result<(), JsValue> {
    // Definir el Web Component
    let _ = Closure::wrap(Box::new(|| {
        web_sys::console::log_1(&"Grace Chat element created".into());
    }) as Box<dyn FnMut()>);
    
    // Por simplicidad, usaremos una función JavaScript auxiliar
    Ok(())
}

// Función principal de inicialización
#[wasm_bindgen]
pub fn init_grace_chat() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Grace Chat SDK initialized".into());
    register_grace_chat()?;
    Ok(())
}
