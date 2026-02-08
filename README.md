# Arquitectura-monolito

## Reto de Latencia Mínima: Solución con Monolito
Este proyecto es una implementación en Rust de un sistema de estímulo-respuesta con una latencia inferior a un milisegundo. 
La solución se basa en una arquitectura Monolitica comunicación en memoria entre hilos.

## Arquitectura
La arquitectura elegida consiste en un único proceso, donde el "cliente" y el "servidor" son simplemente hilos (threads) que se comunican a través de canales en memoria.

## Estructura de Archivos
El proyecto está organizado como un workspace de Cargo:
```
.
├── Cargo.toml
└── src/
    └── main.rs
```

##  Prerrequisitos
### Instalación Rust
Para compilar y ejecutar este proyecto, necesitas tener instalado Rust. 
Ejecuta el siguiente comando que descarga y ejecuta el script de instalación oficial de Rust.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Presiona Enter para elegir la opción por defecto (1) Proceed with installation (default).
Cierra tu terminal y vuelve a abrirla.

Verifica la instalación:
```
cargo --version
```

### Cómo Compilar y Ejecutar
Ve a la raíz del proyecto (Monolito/).

1. Compilar el Proyecto
```
cargo run --release
```

```
## Resultados Esperados
Ver file cargado en la carpeta `/resultados` 

----------------------------------------------------
