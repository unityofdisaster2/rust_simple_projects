use std::fmt;
use std::any;

// se pueden utilizar varios traits predefinidos con derive
// ya sea para operaciones de comparacion, copia, clon, debug, etc.
// PartialEq permite comparar todos los campos de un struct y solo si todos
// son iguales la expresion sera verdadera
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64
}

impl Satellite {
    fn new(name: String, velocity: f64) -> Satellite{
        Satellite {
            name,
            velocity
        }
    }
}

struct SpaceStation {
    name: String,
    crew_size: u32,
    altitude: u32
}

impl SpaceStation {
    fn new(name: String, crew_size: u32, altitude: u32) -> SpaceStation {
        SpaceStation {
            name,
            crew_size,
            altitude
        }
    }
}

// los traits son como interfaces en otros lenguajes. Se declara una funcion que puede ser
// definida de varias formas
trait Description {
    // se puede dejar solo la declaracion o tambien se puede implementar una version generica (default)
    //fn describe(&self) -> String;
    fn describe(&self) -> String {
        String::from("an object flying though space!")
    }
}


impl Description for Satellite {
    fn describe(&self) -> String {
        format!("the {} flying at {} miles per second!", self.name, self.velocity)
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying {} miles high with {} crew members aboard", self.name, self.altitude, self.crew_size)
    }
}

// en algunos casos (principalmente con funciones genericas) se limitara el uso de los valores
// debido a que no se sabe que tipo de dato se esta utilizando y si ese tipo implementa alguna
// funcion en especifico como display o la habilidad de sumarse con otro valor. Para esto tambien
// podemos especificar o extender la funcion del tipo generico extendiendo traits
fn print_type<T: fmt::Display>(item: T) {
    println!("{} is {}", item, any::type_name::<T>());
}

// dependiendo de la complejidad de la funcion, se puede necesitar implementar mas de un trait
// hay dos formas de hacer esto, una es usando el operador + como separador entre traits y el
// otro es usar where

//fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U) 
    where T: fmt::Display + PartialEq + From<U>, 
          U: fmt::Display + PartialEq + Copy
    {
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is not equal to {}", a, b);
    }
}

// se puede limitar el retorno de una funcion a tipos que implementen un trait en particular

fn get_displayable() -> impl fmt::Display {
    // el retorno debe ser consistente, es decir, no se debe tener condiciones que
    // en un caso retornen un tipo entero, en otra un tipo flotante o en otra un tipo cadena
    12
    // este no funcionaria porque los arreglos no implementen Display
    //[12]
}

fn main() {
    let hubble = Satellite::new(String::from("Hubble Telescope"), 4.72);
    let gps = Satellite::new(String::from("GPS"), 2.42);
    let iss = SpaceStation::new(String::from("International Space Station"), 6, 254);

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());

    println!("is hubble the same as gps: {}", hubble == gps);
    println!("is hubble the same as gps: {}", hubble > gps);

    print_type(10);
    print_type(10.4);
    print_type("ten");
    // este fallaria ya que los arrays no tienen definido el trait Display
    // para este caso se tendria que usar Debug
    //print_type([10]);

    compare_and_print(10, 30);
    compare_and_print(10, 10);
}
