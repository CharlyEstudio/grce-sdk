// Loader principal que se incluirá en el CDN
(async function() {
    'use strict';
    
    // Verificar si ya está cargado
    if (window.GraceChatSDK) {
        console.warn('Grace Chat SDK already loaded');
        return;
    }

    // Marcar como cargado
    window.GraceChatSDK = {
        version: '0.0.1',
        loaded: true
    };

    try {
        // URL base del CDN (se configurará en build)
        const CDN_BASE_URL = 'https://cdn.tu-sdk'; // Se reemplazará en build
        
        // Cargar el Web Component
        const { default: GraceChatElement } = await import('./grace-chat-element.js');
        
        console.log('Grace Chat SDK loaded successfully');
        
        // Evento personalizado para notificar que el SDK está listo
        window.dispatchEvent(new CustomEvent('grace-chat-sdk:ready', {
            detail: { version: window.GraceChatSDK.version }
        }));
        
    } catch (error) {
        console.error('Failed to load Grace Chat SDK:', error);
        
        // Evento de error
        window.dispatchEvent(new CustomEvent('grace-chat-sdk:error', {
            detail: { error: error.message }
        }));
    }
})();