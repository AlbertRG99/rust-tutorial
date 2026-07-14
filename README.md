# Tutorial de Rust

## Compilación y ejecución

- cargo new proyecto
- cargo run
- cargo clean

## Teoría

- **Macros**: Son funciones que generan código en tiempo de compilación. Las macros no reciben valores sino código. Usan "!" al final.
 
```rust
fn main() {
    println!("Hello, world")
}
```

### Tipos de datos

- **Escalares**:
  - Enteros
    - i8
    - u8
    - u16
    - i32 (por defecto en inferencia de tipos)
    - u32
    - i64
    - u64
    - i128
    - u128
    - isize/usize (depende de la arquitectura del CPU)
  - Floats
    - f32
    - f64
  - Booleanos
    - bool
  - Caracteres
    - char
- **Compuestos**:
  - Tuplas
  - Arrays