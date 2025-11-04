# Grace Chat SDK

[![Build and Deploy](https://github.com/CharlyEstudio/grce-sdk/actions/workflows/release.yml/badge.svg)](https://github.com/CharlyEstudio/grce-sdk/actions/workflows/release.yml)

SDK de chat en Rust compilado a WebAssembly para integración fácil en sitios web. **Ahora con soporte WebSocket para chat en tiempo real!**

## 🚀 CDN Usage

### Modo HTTP (Tradicional)
```html
<script type="module" src="https://CharlyEstudio.github.io/grce-sdk/grace-chat-loader.js"></script>

<grace-chat 
    api-key="tu-api-key" 
    welcome="¡Hola! ¿Cómo puedo ayudarte?"
    theme="light">
</grace-chat>
```

### Modo WebSocket (Tiempo Real) 🆕
```html
<script type="module" src="https://CharlyEstudio.github.io/grce-sdk/grace-chat-loader.js"></script>

<grace-chat 
    api-key="tu-api-key"
    mode="websocket"
    user-id="usuario-123"
    welcome="¡Chat en tiempo real!"
    theme="dark">
</grace-chat>
```

### Modo Híbrido (Mejor de ambos) 🔄
```html
<grace-chat 
    api-key="tu-api-key"
    mode="hybrid"
    user-id="usuario-123"
    welcome="¡Fallback automático!"
    theme="light">
</grace-chat>
```

## ⚡ Características WebSocket

- **💬 Chat en Tiempo Real**: Mensajes instantáneos sin polling
- **✍️ Indicadores de Escritura**: Ve cuando otros usuarios están escribiendo
- **👥 Presencia de Usuarios**: Estado online/offline en tiempo real
- **🔄 Auto-Reconexión**: Reconexión automática en caso de pérdida de red
- **📊 Estados de Conexión**: Monitoring completo del estado de conexión
- **🏷️ Mensajes Tipados**: Soporte para diferentes tipos de mensaje

## 📋 Atributos del Widget

### Básicos (Todos los modos)
- `api-key`: Clave de API (requerido)
- `welcome`: Mensaje de bienvenida (opcional)
- `theme`: Tema visual - "light" o "dark" (opcional, default: "light")

### WebSocket (Modo websocket/hybrid)
- `mode`: Modo de operación - "http", "websocket", "hybrid" (opcional, default: "http")
- `user-id`: ID único del usuario (requerido para websocket/hybrid)

**Nota**: La URL del WebSocket es interna y se configura como variable de entorno por seguridad.

## 🔧 Configuración del Servidor WebSocket

### Ejemplo con Node.js + Socket.IO
```javascript
const io = require('socket.io')(server);

io.on('connection', (socket) => {
    console.log('Usuario conectado:', socket.id);
    
    // Mensajes de chat
    socket.on('chat_message', (data) => {
        socket.emit('chat_response', {
            id: generateId(),
            content: processMessage(data.content),
            timestamp: Date.now()
        });
    });
    
    // Indicadores de escritura
    socket.on('typing', (data) => {
        socket.broadcast.emit('user_typing', {
            user_id: data.user_id,
            is_typing: data.is_typing
        });
    });
});
```

### Tipos de Mensaje WebSocket
```typescript
// Mensaje de chat
interface ChatMessage {
    type: "ChatMessage";
    id: string;
    content: string;
    user_id: string;
    timestamp: number;
}

// Indicador de escritura
interface UserTyping {
    type: "UserTyping";
    user_id: string;
    is_typing: boolean;
}

// Presencia de usuario
interface UserPresence {
    type: "UserPresence";
    user_id: string;
    status: "online" | "away" | "offline";
}
```

## 📦 Releases

Para crear un nuevo release y activar el build automático:

1. Crea un tag: `git tag v1.0.0`
2. Push el tag: `git push origin v1.0.0`
3. Ve a GitHub y crea un Release desde el tag
4. GitHub Actions automáticamente compilará y desplegará a GitHub Pages

## 🔧 Development

### Prerrequisitos
- [Rust](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Setup Local
```bash
# Clonar el repositorio
git clone https://github.com/CharlyEstudio/grce-sdk.git
cd grce-sdk

# Build con script local (configuración automática)
./build-local.sh

# O manualmente:
cp .env.example .env  # Configura WEBSOCKET_URL
export WEBSOCKET_URL="wss://tu-servidor.com/ws"
wasm-pack build --target web --out-dir pkg
cargo run --bin generate_loader

# Servir demo completo
cd web && python3 -m http.server 8080
```

### Variables de Entorno
El SDK requiere configurar la URL del WebSocket en tiempo de compilación:

```bash
# Desarrollo local (.env)
WEBSOCKET_URL=ws://localhost:3000/chat

# Producción (GitHub Actions)
WEBSOCKET_URL=wss://api.gracechat.dev/ws
```

## Estado actual

- ✅ **HTTP Mode**: Validación de API Key, chat con NewsAPI
- ✅ **WebSocket Support**: Estructura completa implementada
- ✅ **Multi-Mode**: HTTP, WebSocket, Hybrid
- ✅ **Widget responsivo**: Temas claro/oscuro, minimizar/maximizar
- 🧪 **WebSocket Demo**: Implementación base funcional
- 🚧 **Servidor WebSocket**: Requiere implementación backend
- � **Auto-reconexión**: Lógica implementada, requiere testing

## Próximas funcionalidades

- [ ] Servidor WebSocket de ejemplo completo
- [ ] Sistema de rooms/canales
- [ ] Notificaciones push
- [ ] Mensajes multimedia (imágenes, archivos)
- [ ] Historial de mensajes persistente
- [ ] Moderación automática de contenido