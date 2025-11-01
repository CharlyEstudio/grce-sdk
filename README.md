# Grace Chat SDK

Widget de chat embebible compilado a WebAssembly con Rust.

## Estructura del proyecto

```
grace-sdk/
├── Cargo.toml          # Configuración de Rust
├── src/
│   └── lib.rs         # Código principal del SDK
└── pkg/               # Archivos generados por wasm-pack
    ├── grace_sdk.js   # Módulo WASM compilado
    └── grace_sdk_bg.wasm # Binario WebAssembly
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
```

## Uso

### Build y distribución

```bash
# Compilar y generar archivos CDN automáticamente
wasm-pack build --target web --out-dir pkg

# Para personalizar el CDN URL:
CDN_URL=https://mi-cdn.com wasm-pack build --target web --out-dir pkg
```

Esto genera automáticamente:
- `pkg/grace_sdk.js` - Módulo WASM
- `pkg/grace_sdk_bg.wasm` - Binario WASM  
- `pkg/grace-chat-loader.js` - Loader listo para CDN

### Usar el SDK (usuario final)

```html
<!DOCTYPE html>
<html>
<head>
    <title>Mi sitio web</title>
</head>
<body>
    <h1>Mi contenido</h1>
    
    <!-- Incluir el SDK desde CDN -->
    <script type="module" src="https://cdn.tu-sdk/grace-chat-loader.js"></script>
    
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