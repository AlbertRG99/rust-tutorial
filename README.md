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
- **Compuestos**:
    - Tuplas
    - Arrays

## 1. Declaración de variables

Las variables son inmutables por defecto, es decir, no se pueden cambiar.

```rust
fn main() {

    let edad: i32=30;
    
    let mensaje: &str="Hola";
    
    let precio: f64=19.99;
    
    let activa: bool=true;

}
```

Las variables no se crean en memoria como tal, sino que se almacena en una dirección de memoria y se asocia el nombre a esa dirección ("binding"):

![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260714124628.png?raw=true)

No se puede cambiar algo de nombre s**in eliminar la variable original** porque ese nombre original es **dueño** de ese valor. Habría que usar borrowing.
![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260714125144.png?raw=true)

```rust
let nombre: String=String::from("Juan");
let dato: String=nombre;

println!("{}", dato);
println!("{}", nombre); // ya no se va a imprimir porque ya no tiene el ownership
```

## 2. Variables inmutables vs mutables vs constantes

![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260714143037.png?raw=true)

- Se pueden declarar **fuera de una función**: let/const
- No se pueden declarar **fuera de una función**: let mut

Solo se deben poner constantes para algo que no va a variar nunca en el programa.
Para manejar variables entre funciones no va a funcionar const, solo **let**.

## 3. Borrowing

Mecanismo mediante el cual una variable permite que otra parte del programa acceda a su valor sin transferir la propiedad (ownership).

Lo hace creando una referencia a él (mutable o inmutable), permitiendo leerlo o modificarlo temporalmente sin cambiar el dueño real.

Con esto se aumenta la seguridad. Así evitas lo que suele pasar en C++: Punteros colgantes, accesos simultáneos o modificaciones no controladas.

![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260714151830.png?raw=true)

Si intento copiar una variable en otra, y elimino la primera, me va a dar error porque la nueva variable no apunta a la posición de memoria sino a la variable original:

```rust
let saludo: String=String::from("hola");
let referencia: &String=&saludo; // variable copiada (&)
println!("Original: {}", saludo);
println!("Préstamo: {}", referencia);

std::mem::drop(saludo); // elimino la original

println!("Préstamo: {}", referencia); // ERROR: La variable a la que hacía referencia ya no existe
```

- **Importante**: Cuando una variable original **mutable** está prestada a otra variable, no se puede leer desde otra variable. UNA LOCURA!
![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260714184215.png?raw=true)

#### Problemas del ownership/borrowing

En el ownership, cualquier variables que declares, le va a pertenecer a un bloque, y ese bloque va a ser el dueño de esa variable.
De esta forma, cuando deja de ejecutarse ese bloque, saca la variable de memoria.

```rust
struct Tupla {
    a: i64,
    b: i64,
}

fn main() {
    if ... {
        let tupla = Tupla { a:4, b:2 }; // la tupla pertenece al scoope del IF.
    }
}
```

- **Traspasar el ownership**:

Una variable pertenece a la última función donde se haya introducido/declarado. Si llamo a una función B desde otra función A con una variable originalmente declarada en A, ahora pertenecerá a B y no se podrá volver a usar en A. Es como una pelota que se pasa de función en función.

```rust
struct Tupla {
    a: i64,
    b: i64,
}

fn sumar(tpl: Tupla) -> i64 {
    tpl.a + tpl.b
}

fn restar(tpl: Tupla) -> i64 {
    tpl.a - tpl.b
}

fn main() {
    let tupla = Tupla { a:4, b:2 }; // tupla pertenece a main()
    let suma = sumar(tupla); // AHORA tupla pertenece a suma() y deja de funcionar para main()!!
    let resta = restar(tupla); // ERROR: No compila porque el ownership de tupla ya no está en la función main()
}
```

**Solución**: Pasar por referencia (&{var}):

```rust
struct Tupla {
    a: i64,
    b: i64,
}

fn sumar(tpl: &Tupla) -> i64 { // paso por referencia
    tpl.a + tpl.b
}

fn restar(tpl: &Tupla) -> i64 { // paso por referencia
    tpl.a - tpl.b
}

fn main() {
    let tupla = Tupla { a:4, b:2 }; // tupla pertenece a main()
    let suma = sumar(&tupla); // Paso por referencia (mantiene ownership)
    let resta = restar(&tupla); // Paso por referencia (mantiene ownership)
}
```

"tupla" sigue siendo del main() y se **puede únicamente leer**.

- Qué hacer si quiero pasar por **COPIA**? -> Especifico ```#[derive(Clone)]``` al principio.

```rust
#[derive(Clone)]

struct Tupla {
    a: i64,
    b: i64,
}

fn sumar(tpl: &Tupla) -> i64 { // paso por referencia
    tpl.a + tpl.b
}

fn restar(tpl: &Tupla) -> i64 { // paso por referencia
    tpl.a - tpl.b
}

fn main() {
    let tupla = Tupla { a:4, b:2 }; // tupla pertenece a main()
    let suma = sumar(tupla.clone()); // Paso por copia
    let resta = restar(tupla.clone()); // Paso por copia
}
```


## 4. Lifetime de variables y referencias | String vs &str

En Rust no se permiten los punteros colgantes (en C++, sí).
Esto quiere decir que si se hace referencia a algo que ya no existe o existía dentro de un bloque de código,  va a dar error al compilar porque sabe que 's' se crea y muere dentro de ese bloque y que si intentas hacer referencia a la variable desde fuera, va a dar error. Apuntaría a una variable que ya no existe.

```rust
let r;

{ //bloque de código
    let s=String::from("Hola");
    r = &s; // fuera de este bloque, "s" no vive
}

println!("{}", r);
```

La diferencia entre String y &str es que el String tiene un propietario (se puede modificar) mientras que &str no lo tiene. &str **muere** cuando ninguna variable esté apuntando a ella.

Por norma general:
- String: Para strings que quiero modificar.
- &str: Para variables que solo quiero leer.

```rust
let s: String=String::from("Hola"); // 's' SÍ tiene el ownership (SE puede modificar/mutar)

let t: &str="Adios"; // 't' NO tiene el ownership (NO se puede modificar y vive mientras lo haga la variable que lo apunta)
```

![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260715153848.png?raw=true)

## 5. Slices

Referencia a parte de una coleccioón de datos que ya existe.
Permite **ver parte de los datos sin copiarlos y sin tener el ownership**.
El slice tiene borrowing y lifetime.

Para qué sirven?
- Para tener mayor eficiencia y seguridad:
    - Evita copias innecesarias.
    - Pasa datos a funciones de forma seguira.
    - Permite APIs más flexibles.

El slice se crea con una referencia (a lo que apunta) y un rango (lo que quiero obtener):

```rust
let variable: &{tipo} = &{var.apunta}[inicio..fin];
```

```rust
let s: String=String::from("Hola alumnos");
let saludo: &str=&s[0..4];
```

![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260715155437.png?raw=true)

```rust
let numeros: [i32; 5] = [1, 2, 3, 4, 5];
let parte: &[i32]=&numeros[1..4];

println!("{}", parte); // ERROR: Array no contiene Display
```

- **IMPORTANTE**: Rust no es capaz de imprimir todo por pantalla. Todo tiene que ser de tipo String. Los arrays no son capaces de mostrarse en consola porque no implementan Display. Hay que usar **{:?}**: 

```rust 
println!("{:?}", array);
```

```rust
let numeros: [i32; 5] = [1, 2, 3, 4, 5];
let parte: &[i32]=&numeros[1..4];

println!("{:?}", parte); // Ahora sí que imprime
```

## 6. Creación y uso de vectores

Qué es un Vec?:
- Un `Vec<T>` es una colección de elementos del mismo tipo (T). 
- Se almacenan en el heap.
- Pueden cambiar de tamaño de forma dinámica.
- Permite agregar o quitar elementos dinámicamente.

Qué guarda en memoria un Vec?
- En el stack guarda una cabecera con 3 datos:
    - **ptr**-> puntero al heap donde están los elementos.
    - **len**-> cuántos elementos hay actualmente.
    - **cap**-> cuántos elementos caben sin realocar ("mudanza" a otro vector para agrandarlo).

Por qué usar Vec en lugar de simples arrays?. Cuando no sabes de antemano:

- Cuántos datos vas a tener.
- Cuántos usuarios.
- Cuántos resultados de una búsqueda.
- Cuántas líneas tiene un archivo.
- Cuántos registros devuelve una DB.

```rust
// Vectores fijos
let num: Vec<i32>=vec![1,2,3];
println!("{:?}", num);

// Vectores dinámicos (vacío)
let mut numeros: Vec<i32>=Vec::new(); // nuevo vector vacío
numeros.push(10); // agregar elementos
numeros.push(20);
numeros.push(30);
println!("{:?}", numeros);

// Vectores dinámicos con reserva mínima (para no realocar)
let mut numeros_cap: Vec<i32>=Vec::with_capacity(500);
numeros_cap.push(10);
println!("{:?}", numeros_cap);
```

Reservar con `with_capacity` no significa establecer un máximo de elementos para ese vector sino evitar que reserve menos memoria de la necesaria al principio si tenemos un previsión de cuantos elementos queremos meter en ese array. Es una forma de eficiencia.

## 6.1. Creación y uso de vectores II (Eliminación y acceso a elementos)

Vectores dinámicos (Vec):
- Eliminar elementos.
- Acceso a elementos con `[]` y `get()`

Para eliminar elementos de un array se usa `array.pop()`, que retorna un `Option<T>`.
Un `Option<T>` significa que podría eliminar algo o no eliminarlo.
Rust cree que podrías estar intentando eliminar un elemento que no existe y en otros lenguajes daría error, por lo que tendrías que intentar controlarlo con un `try/except` (python).

Para eliminar un elemento mientras se lo asignas a otra variable hay que suar un Option, que va a retornar `Some(elemento)` si existe, y `None`, si no existe:

```rust
let ultimo: Option<i32>=numeros.pop();

println!("{:?}", ultimo); // existe: `Some(30)`, no existe: `None`
```

```rust

let mut ultimo: Option<i32>=numeros.pop();
println!("{:?}", ultimo); // retorna `Some(30)`

ultimo=numeros.pop();
println!("{:?}", ultimo); // retorna `Some(20)`

ultimo=numeros.pop();
println!("{:?}", ultimo); // retorna `Some(10)`

ultimo=numeros.pop();
println!("{:?}", ultimo); // retorna `None`
```

Acceso a elementos (como en todos los lenguajes, con `[ ]`):

```rust
println!("{:?}", numeros[0]); // 10
println!("{:?}", numeros[1]); // 20
println!("{:?}", numeros[2]); // 30
println!("{:?}", numeros[3]); // _ERROR_: "index out of bounds: the len is 3 but the index is 3"
```

Pero se puede acceder de una forma más segura con `get()`:

```rust
println!("{:?}", numeros.get(0)); // 10
println!("{:?}", numeros.get(1)); // 20
println!("{:?}", numeros.get(2)); // 30
println!("{:?}", numeros.get(3)); // `None`
```

RESUMEN:

- `Some()` variante de `Option`.
- `pop()` retorna `Option<T>`.
- `Some(x)` indica que hay valor.
- `None` nos indica que no hay valor.
- Rust evita `Null`.
- `get()` mejor que `[ ]`.

## 6.2. Creación y uso de vectores III (recorrer vectores con bucle FOR)

Vectores dinámicos:
- Iterar con bucle for un vector.

```rust
let numeros: Vec<i32>=vec![1,2,3];

for n: i32 in numeros{ // ownership del array transferido al iterador del bucle
    println!("{:?}", n);
}
for n: i32 in numeros{ // ERROR: ownership del array no existe
    println!("{:?}", n);
}
```

El problema es que cuando se recorre un array, el **ownership cambia al iterador** que se crea dentro del bucle for. Se debe a que no hay recolector de basura como en otros lenguajes como Java.

**Solución**: Usar referencias (borrowing).

```rust
let numeros: Vec<i32>=vec![1,2,3];

for n: &i32 in &numeros{ // NO hay transferencia de ownership (OK)
    println!("{:?}", n);
}
for n: &i32 in &numeros{ // NO hay transferencia de ownership (OK)
    println!("{:?}", n);
}
```

Modificación de elementos del vector:

Si se modifican los elementos del vector dentro del bucle for, va a dar error:
Requisitos:
- Vector mutable.
- for mutable y referenciado.
- Desreferenciación:  `*n` (acceso al valor de la referencia).

```rust
let mut numeros = vec![1, 2, 3]; // 1. vector mutable

for n in &mut numeros {      // 2. bucle mutable y referenciado
    println!("{:?}", *n+=1); // 3. desreferenciado (*)
}
```

## 6.3. Creación y uso de vectores IV (iteradores)

Vectores dinámicos:
- Iteradores.

Los iteradores no mueven el vector sino que lo prestan (borrowing).
Es más sencillo, en lugar de usar la sintaxis anterior, usar `iter_mut()`.

Solo es necesario:
- iter/iter_mut() en el for.
- Desreferenciación.

```rust
let mut numeros = vec![1, 2, 3]; // 1. vector mutable

for n in numeros.iter_mut() {      // 2. iter_mut()
    println!("{:?}", *n+=1);       // 3. desreferenciado (*)
}
```

```rust
let numeros = vec![1, 2, 3]; // 1. vector inmutable

for n in numeros.iter() {          // 2. iter()
    println!("{:?}", *n+=1);       // 3. desreferenciado (*)
}
```

![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260716022025.png?raw=true)

## 6.4. Creación y uso de vectores V (Map(), filter, collect())

Vectores dinámicos (Vec):
- Iteradores:
    - **filter()**
    - **map()**
    - **collect()**

![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260716022437.png?raw=true)

Imagina que quiero recorrer un vector, tomar solo los valores pares, multiplicarlos y mostrarlo por consola:

```rust
//Forma tradicional
let numeros = vec![1, 2, 3, 4, 5, 6];
let mut multip: i32=1;

for n in numeros.iter() {
    if (n%2==0) {
        multip = multip*n;
    }
}

println!("{}", multip);
```

```rust
//Forma funcional (|funciones anónimas|)
let numeros = vec![1, 2, 3, 4, 5, 6];
let mut multip: i32=1;

numeros
    .iter()
    .filter(|n|*n%2==0)
    .map(|multip: i32|*n);

println!("{}", multip);
```

![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260716024755.png?raw=true)

#### APARTADO: Closures / funciones anónimas

Son funciones anónimas que pueden capturar variables de su entorno.
Parecidos a las lambda en otros lenguajes.

`|parámetros| expresión`
`|parámetros| {código resultado}`

```rust
let sumar = |a,b| a+b;

println!("{}", sumar(3, 4));
```

### APARTADO: Sacar el valor del Some(x)

Para sacar el valor interno de, por ejemplo `Some(10)` se usa `.unwrap()`.

## 6.5. Creación y uso de vectores VI (funciones útiles para cálculos)


Vectores dinámicos (Vec):
- Iteradores:
    - **sum()** -> Retorna el tipo que le indiques
    - **find()** -> Hay que desreferenciar. Retorna un `Option<&i32>`
    - **count()** -> Retorna un `unsize`.
    - **any()** -> Retorna un `bool`.
    - **all()** -> Retorna un `bool`.
    - **max()** -> Retorna un `Option<&i32>`. NO FUNCIONA SOBRE NÚMEROS DECIMALES.
    - **min()** -> Retorna un `Option<&i32>`. NO FUNCIONA SOBRE NÚMEROS DECIMALES.
    - **max_by()** -> 
    - **min_by()** -> 

```rust
let numeros: Vec<i32>=vec![1,2,3,4,5,6];

// sum() -> Suma los resultados
let resultado: i32;
resultado = numeros.iter().sum();
println!("{:?}", resultado);

//find() -> Encuentra los resultados que cumplan una condición
let resultado;
resultado = numeros.iter().find(|&&n|n>3);
println!("{:?}", resultado);

// count() -> Cuenta los resultados
let resultado;
resultado = numeros.iter().filter(|&&n| n%2==0).count();
println!("{:?}", resultado);

// any() -> Busca si hay al menos un resultado que cumpla una condición (bool)
let resultado;
resultado = numeros.iter().any(|n| n%2==0);
println!("{:?}", resultado);

// all() -> Comprueba si todos los elementos cumplen la condición deseada
let resultado;
resultado = numeros.iter().all(|n| n%2==0);
println!("{:?}", resultado);

// max() -> Elemento máximo de un array
let resultado;
resultado = numeros.iter().max();
println!("{:?}", resultado);

// min() -> Elemento mínimo de un array
let resultado;
resultado = numeros.iter().min();
println!("{:?}", resultado);
```

## 7. Ejemplo práctico final

Queremos crear un programa en Rust al que le vamos a proporcionar las notas de un alumno en forma de vector (en decimal) y tenemos que obtener los aprobados, media y la nota más alta.

```rust
let _notas: Vec<f32> = vec![
    8.75, 6.50, 9.20, 4.80, 7.35,
    5.90, 10.00, 3.45, 8.10, 9.95,
];

let aprobadas: Vec<f32>;
aprobadas = _notas.iter().filter(|n|**n>=5.0).copied().collect(); // .copied().collect(), necesario para pasar a un nuevo array
println!("Aprobadas: {:?}", aprobadas);


let mut media: f32;
media = _notas.iter().sum();
media = media/(_notas.iter().count() as f32); // importante que para dividir los dos números sean del mismo tipo
println!("Media: {:?}", media);


let maximo;
//maximo = _notas.iter().max(); // OJO!: No se puede usar max() directamente sobre decimales
maximo = _notas.iter().max_by(|a,b|a.partial_cmp(b).unwrap());
println!("Máxima nota: {:?}", maximo.unwrap());
```

Para comparar números no `int`:
```rust
max_by(|a,b|a.partial_cmp(b).unwrap());
```


## 8. HashMap: Guardar datos con clave y valor

Qué es un hashmap?
- Una colección que permite almacenar datos en forma de clave-valor.
- Un HashMap sirve para guardar datos asociados a una clave. En lugar de mencionar la posición para acceder a valores como ocurre con vectores, se mencionan las claves.
    - Ejemplo:
        - "Ana"->8,5
        - "Luis"->4,2
        - "Marta"->9,1
    - Importante: Si modificas un hashmap existente, se reescribe el original y se pierde el valor original.
- Un vector es ideal cuando tengo una lista ordenada de elementos. Un hashmap es más útil cuando quiero buscar un dat por una clave, como por ejemplo, un identificador, un código, una palabra, un usuario, ...
- Casos de uso del HashMap (mejor que vectores):
    - Caché de datos.
    - Configuración de una aplicación (ej: modo, idioma, tema, ...).
    - Agrupamiento de datos por categorías.
    - Sesiones de usuario (ID usuario -> Token usuario).

![](https://github.com/AlbertRG99/rust-tutorial/blob/main/media/Pasted%20image%20260716162210.png?raw=true)

- Para insertar elementos se usa `.insert(k:&str, v:f64)`.
- Para leer un elemento en base su clave se usa `.get(k)`.
- Para recorrerlo en un bucle for:

```rust
    for (key,value) in &hashmap {
        ...
    }
```

```rust
use std::collections::HashMap; // importar el HashMap

let mut notas=HashMap::new(); // crear un HashMap vacío

notas.insert("Ana", 8.5); // insertar elementos con `insert(k,v)`
notas.insert("Luis", 4.2);
notas.insert("Marta", 9.1);
notas.insert("Pedro", 6.4);

println!("{:#?}", notas);

let nota_marta = notas.get("Marta"); // obtener valor en base a su clave `.get()`
println!("Nota de Marta: {:#?}", nota_marta.unwrap());

// Recorrerlo con un bucle for (k,v)
for (alumno,nota) in &notas {
    println!("El alumno {:#?} tiene una nota de {:?}", alumno, nota);
}
```

