/* FUNCIONES AUXILIARES */


fn obtener_numero(n:i32)->i32 {
    n
}


fn main() {
    
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
    
    // Creación y uso de vectores
    let num: Vec<i32>=vec![1,2,3];
    println!("{:?}", num);

    let mut numeros: Vec<i32>=Vec::new();   // nuevo vector vacío
    numeros.push(10);                       // agregar elementos
    numeros.push(20);
    numeros.push(30);
    println!("{:?}", numeros);

    let mut numeros_cap: Vec<i32>=Vec::with_capacity(500);  // Reserva espacio para 500 elementos (sin realocar en un nuevo vector)
    numeros_cap.push(10);
    
    // Creación y uso de vectores II (eliminación y acceso a elementos)
    let mut numeros: Vec<i32>=Vec::new(); 
    numeros.push(10);                       // agregar elementos
    numeros.push(20);
    numeros.push(30);
    //numeros.pop();                          // elimina el último elemento
    println!("{:?}", numeros);

    let mut ultimo: Option<i32>=numeros.pop();
    println!("{:?}", ultimo);   // retorna `Some(30)`

    ultimo=numeros.pop();
    println!("{:?}", ultimo);   // retorna `Some(20)`

    ultimo=numeros.pop();
    println!("{:?}", ultimo);   // retorna `Some(10)`

    ultimo=numeros.pop();
    println!("{:?}", ultimo);   // retorna `None`
    
    // Acceso a elementos de un vector
    let mut numeros: Vec<i32>=Vec::new(); 
    numeros.push(10);                    
    numeros.push(20);
    numeros.push(30);

    println!("{:?}", numeros[0]);
    println!("{:?}", numeros[1]);
    println!("{:?}", numeros[2]);
    println!("{:?}", numeros[3]); // ERROR: "index out of bounds: the len is 3 but the index is 3"
    
    // Para acceder de una forma más segura, usar `get()`
    let mut numeros: Vec<i32>=Vec::new(); 
    numeros.push(10);                    
    numeros.push(20);
    numeros.push(30);

    println!("{:?}", numeros.get(0));
    println!("{:?}", numeros.get(1));
    println!("{:?}", numeros.get(2));
    println!("{:?}", numeros.get(3)); // None
    
    // Creación y uso de vectores II (recorrer vectores con bucles FOR)
    let numeros: Vec<i32>=vec![1,2,3];

    for n in numeros{   // ownership del array se transfiere al iterador
        println!("{:?}", n);
    }
    for n in numeros{   // ahora el array no tiene el ownership (ERROR)
        println!("{:?}", n);
    }
    
    // Acceso al bucle for sin modificar el ownership (borrowing)
    let numeros = vec![1, 2, 3];

    for n in &numeros { // NO hay transferencia de ownership (OK)
        println!("{:?}", n);
    }
    for n in &numeros { // NO hay transferencia de ownership (OK)
        println!("{:?}", n);
    }
    
    // Modificación de elementos dentro del bucle for
    let mut numeros = vec![1, 2, 3];    // vector mutable

    for n in &mut numeros {             // for referenciado a mutable
        *n+=1;                          // desreferenciación
    }
    println!("{:?}", numeros);
    
    // Creación y uso de vectores IV (iteradores)
    // Mutables
    let mut numeros = vec![1, 2, 3];    

    for n in numeros.iter_mut() {             
        println!("{:?}", n);                                    
    }
    // Inmutables
    let numeros = vec![1, 2, 3];    

    for n in numeros.iter() {             
        println!("{:?}", n);                                    
    }
    
    // Iteradores IV (Map, Filter, Collect)

    // Ej: Tomar solo los valores pares y multiplicarlos
    
    // -> Forma tradicional
    let numeros = vec![1, 2, 3, 4, 5, 6];    
    let mut multip: i32=1;
    for n in numeros.iter() {             
        if (n%2==0) {
            multip = multip*n;
        }
    }
    println!("{}", multip);
    
    // -> Forma funcional
    let numeros = vec![1, 2, 3, 4, 5, 6];  
    let mut multip: i32=1;
    
    let resultado:Vec<i32> = 
        numeros
            .iter()
            .filter(|n|*n%2==0)
            .map(|n|n*2)
            .collect();

    println!("{:?}", resultado);
    
    println!("{}", multip);
    
    // Iteradores V (Funciones útiles para cálculos)

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
    // max()
    let resultado; 
    resultado = numeros.iter().max();
    println!("{:?}", resultado);
    // min()
    let resultado; 
    resultado = numeros.iter().min();
    println!("{:?}", resultado);
    
    // Ejemplo práctico final: Dado un array de notas (con decimales), queremos obtener las aprobadas, media y nota más alta
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
    //maximo = _notas.iter().max();   // OJO!: No se puede usar max() directamente sobre decimales
    maximo = _notas.iter().max_by(|a,b|a.partial_cmp(b).unwrap());
    println!("Máxima nota: {:?}", maximo.unwrap());
    
    // HashMap: Guardar datos con clave y valor
    use std::collections::HashMap;

    let mut notas=HashMap::new();

    notas.insert("Ana", 8.5);
    notas.insert("Luis", 4.2);
    notas.insert("Marta", 9.1);
    notas.insert("Pedro", 6.4);
    println!("{:#?}", notas);

    let nota_marta = notas.get("Marta");
    println!("Nota de Marta: {:#?}", nota_marta.unwrap());

    // Recorrerlo con un bucle for (k,v)
    for (alumno,nota) in &notas {
        println!("El alumno {:#?} tiene una nota de {:?}", alumno, nota);
    }


}
