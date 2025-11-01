# Grace Chat SDK

Widget de chat embebible compilado a WebAssembly con Rust.

## Estructura del proyecto

```
grace-sdk/
├── Cargo.toml          # Configuración de Rust
├── src/
│   └── lib.rs         # Código principal del SDK
└── web/
    ├── js/            # Archivos JavaScript
    │   ├── grace-chat-element.js
    │   └── grace-chat-loader.js
    └── styles/
        └── chat.css   # Estilos CSS vanilla
```

## Desarrollo

### Prerrequisitos

- [Rust](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Xcode Command Line Tools (macOS)

```bash
# Instalar wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Aceptar licencia de Xcode (macOS)
sudo xcodebuild -license accept
```

### Build

```bash
# Compilar a WebAssembly
wasm-pack build --target web --out-dir pkg

# Servir archivos para desarrollo local
python -m http.server 8080
# o
python3 -m http.server 8080
```

### Ejemplo de uso

```html
<!DOCTYPE html>
<html>
<head>
    <title>Grace Chat Demo</title>
</head>
<body>
    <h1>Mi sitio web</h1>
    
    <!-- Incluir el loader del SDK -->
    <script type="module" src="http://localhost:8080/web/js/grace-chat-loader.js"></script>
    
    <!-- Widget de chat -->
    <grace-chat
        api-key="pk_test_1234567890abcdefXYZ"
        endpoint="https://api.tu-dominio.chat"
        welcome="¡Hola! ¿En qué te puedo ayudar?"
        theme="dark">
    </grace-chat>
</body>
</html>
```

## Distribución

Los archivos para CDN serán:
- `pkg/grace_sdk.js` - Módulo WASM compilado
- `pkg/grace_sdk_bg.wasm` - Binario WebAssembly  
- `web/js/grace-chat-loader.js` - Loader principal
- `web/js/grace-chat-element.js` - Web Component

## Atributos del widget

- `api-key`: Clave de API (requerido)
- `endpoint`: URL del endpoint (requerido)  
- `welcome`: Mensaje de bienvenida (opcional)
- `theme`: Tema visual - "light" o "dark" (opcional, default: "light")

## Estado actual (PoC)

- ✅ Validación ficticia de API Key
- ✅ Mensaje de bienvenida
- ✅ Temas claro/oscuro
- ✅ Widget responsivo
- ✅ Minimizar/maximizar
- 🚧 Envío de mensajes (pendiente)
- 🚧 Integración con endpoint real (pendiente)