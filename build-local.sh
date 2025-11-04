#!/bin/bash

# Grace Chat SDK - Build Script para Desarrollo Local
# Este script configura las variables de entorno necesarias para compilar

# Verificar si existe .env, si no, crear desde .env.example
if [ ! -f .env ]; then
    echo "🔧 Creando .env desde .env.example..."
    cp .env.example .env
    echo "✅ Archivo .env creado. Modifica las URLs según tu configuración local."
fi

# Cargar variables de entorno desde .env
if [ -f .env ]; then
    echo "📄 Cargando variables de entorno desde .env..."
    export $(cat .env | grep -v '^#' | xargs)
fi

# Verificar que WEBSOCKET_URL está configurada
if [ -z "$WEBSOCKET_URL" ]; then
    echo "❌ Error: WEBSOCKET_URL no está configurada"
    echo "💡 Configura WEBSOCKET_URL en tu archivo .env"
    exit 1
fi

echo "🌐 WebSocket URL: $WEBSOCKET_URL"

# Compilar WASM
echo "🚀 Compilando SDK a WebAssembly..."
wasm-pack build --target web --out-dir pkg

if [ $? -eq 0 ]; then
    echo "✅ Compilación exitosa!"
    
    # Generar loader
    echo "📦 Generando loader..."
    cargo run --bin generate_loader
    
    echo "🎉 ¡Listo! Puedes servir con:"
    echo "   python3 -m http.server 8080"
    echo "   Luego visita: http://localhost:8080/websocket-demo.html"
else
    echo "❌ Error en la compilación"
    exit 1
fi