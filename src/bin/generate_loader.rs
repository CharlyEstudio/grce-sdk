use std::fs;

fn main() {
    let cdn_url = std::env::var("CDN_URL").unwrap_or_else(|_| "https://cdn.grace-sdk.com".to_string());
    let version = env!("CARGO_PKG_VERSION");

    let loader_content = format!(r#"// Grace Chat SDK Loader - Generated automatically
(async function() {{
    'use strict';
    
    // Verificar si ya está cargado
    if (window.GraceChatSDK) {{
        console.warn('Grace Chat SDK already loaded');
        return;
    }}

    // Marcar como cargado
    window.GraceChatSDK = {{
        version: '{}',
        loaded: true,
        cdnUrl: '{}'
    }};

    try {{
        // Cargar el módulo WASM
        const wasmModule = await import('{}/grace_sdk.js');
        await wasmModule.default();
        
        // Inicializar el SDK
        wasmModule.init_grace_chat();
        
        // Registrar el Web Component
        class GraceChatElement extends HTMLElement {{
            constructor() {{
                super();
                this.chatInstance = null;
            }}

            async connectedCallback() {{
                try {{
                    // Crear instancia del chat
                    this.chatInstance = new wasmModule.GraceChatElement();
                    this.chatInstance.connected_callback(this);
                    
                }} catch (error) {{
                    console.error('Error initializing Grace Chat:', error);
                    this.showError('Error al cargar el chat. Por favor, intenta de nuevo.');
                }}
            }}

            disconnectedCallback() {{
                if (this.chatInstance) {{
                    this.chatInstance = null;
                }}
            }}

            showError(message) {{
                this.innerHTML = `
                    <div style="
                        position: fixed;
                        bottom: 20px;
                        right: 20px;
                        background: #dc3545;
                        color: white;
                        padding: 15px;
                        border-radius: 8px;
                        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                        font-family: system-ui, -apple-system, sans-serif;
                        font-size: 14px;
                        max-width: 300px;
                        z-index: 999999;
                    ">
                        <strong>Error:</strong> ${{message}}
                    </div>
                `;
            }}
        }}

        // Registrar el Web Component
        if (!customElements.get('grace-chat')) {{
            customElements.define('grace-chat', GraceChatElement);
        }}
        
        console.log('Grace Chat SDK loaded successfully');
        
        // Evento personalizado para notificar que el SDK está listo
        window.dispatchEvent(new CustomEvent('grace-chat-sdk:ready', {{
            detail: {{ 
                version: window.GraceChatSDK.version,
                cdnUrl: window.GraceChatSDK.cdnUrl
            }}
        }}));
        
    }} catch (error) {{
        console.error('Failed to load Grace Chat SDK:', error);
        
        // Evento de error
        window.dispatchEvent(new CustomEvent('grace-chat-sdk:error', {{
            detail: {{ error: error.message }}
        }}));
    }}
}})();"#, version, cdn_url, cdn_url);

    fs::write("pkg/grace-chat-loader.js", loader_content).unwrap();
    println!("✅ Generated pkg/grace-chat-loader.js with CDN: {}", cdn_url);
}