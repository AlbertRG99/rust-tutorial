/* FUNCIONES AUXILIARES */

fn obtener_numero(n:i32)->i32 {
    n
}


fn main() {
    /*
    // Declaración de variables
    let edad: i32=30;
    let mensaje: &str="Hola";
    let precio: f64=19.99;
    let activa: bool=true;

    // Binding (ownership)
    let nombre: String=String::from("Juan");
    let dato: String=nombre;
    println!("{}", dato);
    println!("{}", nombre); // ya no se va a imprimir porque ya no tiene el ownership
    
    // Variables inmutables/mutables/constantes
    let mut contador: i32=0;
    contador=contador+1;
    println!("Contador después de incrementar: {}", contador);
    let resultado: i32=obtener_numero(10);
    println!("{}", resultado)
    
    // Borrowing
    let saludo: String=String::from("hola");
    let referencia: &String=&saludo;    // variable copiada (&)
    println!("Original: {}", saludo);
    println!("Préstamo: {}", referencia);
    std::mem::drop(saludo); // elimino la original
    println!("Préstamo: {}", referencia); // ERROR: La variable a la que hacía referencia ya no existe
    
    // Lifetime de variables y referencias
    let r;
    { //bloque de código
        let s=String::from("Hola");
        r = &s; // fuera de este bloque, "s" no vive
    }
    println!("{}", r);  // puntero colgante (en C++ se puede hacer pero RUST no lo compila)
    
    // String [para modificar] vs &str [para leer]
    let s: String=String::from("Hola"); // 's' SÍ tiene el ownership (SE puede modificar/mutar)
    let t: &str="Adios";                // 't' NO tiene el ownership (NO se puede modificar y vive mientras lo haga la variable que lo apunta)
    
    // Slices
    let s: String=String::from("Hola alumnos");
    let saludo: &str=&s[0..4];
    println!("{}", saludo);

    let numeros: [i32; 5] = [1, 2, 3, 4, 5];
    let parte: &[i32]=&numeros[1..4];
    println!("{:?}", parte); // hay que poner {:?} para estructuras que no sean String
    */
    // Creación y uso de vectores


}
