fn main() {
    // esta definicion de literal es util cuando la cadena es estatica
    // pero si se quiere mutar es preferible usar el objeto String
    // en estas literales se sabe el contenido en tiempo de compilacion
    // y al no ser mutable la velocidad de ejecucion sera mayor
    let s = "Hello";

    // con los strings se debe reservar espacio en el heap
    // hay una funcion para liberar memoria una vez que termina el scope
    // donde se declaro un valor, la funcion se llama drop
    // rust llama a la funcion al terminar las llaves de una funcion
    let mut s_mut = String::from("Hello");

    s_mut.push_str(", World!");


    // ejemplo clasico de paso por valor y referencia

    //en este caso como es un dato primitivo simplemnte se hace una copia del valor conocido 5.
    let x = 5;
    let y = x;

    // en este otro en lugar de pasar una copia se esta pasando la referencia de memoria

    // las cadenas se representan de dos formas en la memoria, por un lado tienen un conjunto de
    // valores que se guardan en el stack. estos son: ptr, len, capacity. Referente al valor ptr
    // este apunta hacia la direccion de memoria en el heap donde se ubiquen los datos actuales del string

    let mut s1 = String::from("Hello reference");

    // cuando hacemos esta asignacion o copia de s1 a s2, lo que en realidad estamos haciendo es copiar
    // los valores del stack de s1, por lo tanto s2 va a tener el mismo valor del apuntador
    // !!!! uno de los problemas que surgen de este tipo de asignaciones es que rust al tratar
    // de limpiar la memoria una vez que termine el scope, va a tratar de limpiar la misma direccion
    // del heap que ya fue limpiado para s1, lo que llevara a un memory safety warning (double free error)
    // tambien esta asignacion hace que la referencia al stack se quede en s2 y se invalide en s1
    // let s2 = s1;

    let s2 = s1.clone();
    println!("{}", s_mut);
    println!("s1={}, s2={}", s1, s2);

    s1 = takes_and_gives_back(s1);

    // con & se manda una referencia de s1, pero no se delega el ownership, por lo tanto 
    // es posible seguir reutilizando la variable posterior a la ejecucion de esta funcion
    calculate_length(&s1);

    change(&mut s1);

    println!("{}",s1);


    takes_ownership(s1);

    let string_with_spaces = String::from("Hello World my friend");

    first_word(&string_with_spaces);

    println!("{}",first_word_improved(&string_with_spaces[..]));b

    // si tratamos de utilizar s1 desde este punto se mostraran errores porque se perdio el ownership


}


// en este punto se pone interesante el concepto de ownership, ya que el paso por referencia no funciona
// como en otros lenguajes. Si en Rust se pasa la referencia de un objeto como el String pasa algo curioso
// se delega la referencia a la funcion que consume el valor y por lo tanto se limpia con la funcion drop
// de la funcion, es decir cuando se llegue a la ultima llave de takesOwnership se va a limpiar lo que habia en la variable
// y si se intenta seguir usando en main despues de haberlo pasado como parametro, se mostrara un error
fn takes_ownership(some_string: String) {
    println!("{} has new owner", some_string);
}

// Si se quiere mantener el ownership del dueno original se debe devolver el valor modificado en el return
fn takes_and_gives_back(some_string: String) -> String {
    some_string
}


// para evitar tanto lio con el ownership al pasar objetos complejos y evitar tener que retornar el valor cada
// que un valor se use como parametro rust permite hacer paso por referencia con el simbolo & (algo parecido a C)
// esto es similar a los apuntadores, pero en rust si se asegura que la referencia apunte a un valor valido
// esta accion en particular es conocida como borrowing. En otras palabras solo se presta el valor
fn calculate_length(s: &String) -> usize {
    s.len()
}


// es importante aclarar que aqui no es como c y solo se presta el valor en modo lectura. Si se intenta escribir
// sobre "s" el compilador arrojara un error. Esto se puede arreglar o llegar a un acuerdo para que la funcion
// comprenda que se permite mutar el objeto. Para esto hay que usar &mut
// no se puede mutar mas de una vez un valor prestado, es decir no se puede pasar el valor mutable a ams de una variable al mismo tiempo
// a menos que se haga en un scope temporal la segunda asignacion
fn change(some_string: &mut String) {
    some_string.push_str(",modified by function");
}

// para acceder a segmentos de una cadena podemos usar slices. Con estos podemos crear un apuntador al final de cierto
// rango. Por ejemplo si queremos un fragmento de la cadena que vaya de un indice inicial a un indice final, se puede hacer
// con el rango [inicial..final] obteniendo como resultado un apuntador al byte "final" del arreglo
// en los ranges si ya sabemos que queremos empezar del valor 0 se puede omitir de la sintaxis y quedar como [..final]
// lo mismo para el caso opuesto, si queremos empezar de un punto y llegar hasta el final por default [inicial..]
// por ultimo, se pueden omitir ambos indices si se quiere tomar un slice del tamano de todo el arreglo [..]
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// nota: me falto agregar que si en un punto se hace una referencia inmutable y posteriormente se trata de hacer
// otra mutable, el compilador lanzara un error, por ejemplo en este caso si se pasa referencia inmutable a una funcion
// para obtener un slice del string y luego se intenta hacer un clear sobre la cadena (para hacer el clear se requiere mutar)
// se mostrara un error