// Web Component wrapper para el elemento grace-chat
class GraceChatElement extends HTMLElement {
    constructor() {
        super();
        this.chatInstance = null;
    }

    async connectedCallback() {
        try {
            // Importar el módulo WASM
            const wasmModule = await import('../pkg/grace_sdk.js');
            await wasmModule.default();
            
            // Inicializar el SDK
            wasmModule.init_grace_chat();
            
            // Crear instancia del chat
            this.chatInstance = new wasmModule.GraceChatElement();
            this.chatInstance.connected_callback(this);
            
        } catch (error) {
            console.error('Error initializing Grace Chat:', error);
            this.showError('Error al cargar el chat. Por favor, intenta de nuevo.');
        }
    }

    disconnectedCallback() {
        if (this.chatInstance) {
            // Cleanup si es necesario
            this.chatInstance = null;
        }
    }

    showError(message) {
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
                <strong>Error:</strong> ${message}
            </div>
        `;
    }
}

// Registrar el Web Component
if (!customElements.get('grace-chat')) {
    customElements.define('grace-chat', GraceChatElement);
}

export default GraceChatElement;