use std::mem;

// con la literal T se pueden definir valores genericos, por lo tanto
// width y height pueden ser enteros, flotantes o cualquier otro tipo
// pero no se pueden combinar. Es decir, si en un principio alguna se define como
// u32 todas las demas propiedades bajo el tipo T deben coincidir
// En caso de que se quieran definir varios tipos genericos se puede ampliar la lista 
// en el vector de la estructura, como en este caso T, U hasta N tipos
#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U
}

struct Shuttle {
    name: String,
    crew_size: u8,
    propelant: f64
}

// metodo generico
impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

// estos metodos solo aplican cuando los parametros sean u8
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2*(self.width + self.height)
    }
}


// tambien se pueden definir funciones genericas
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// el tipo box (considerado smart pointer) guarda los datos en el heap en lugar del stack




fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u16
    };

    let rect_u8 = Rectangle {
        width: 1u8,
        height: 3u8
    };

    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 6,
        propelant: 83402.0
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect_u8.get_perimeter());

    println!("biggest is {}", get_biggest(120, 50));

    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));

    // con el box se pueden mover los datos de una estructura al heap. Esto no es una
    // copia, es decir, cuando se mueven los datos, el objeto vehicle pierde ownership y
    // las propiedades que estaban en el stack se mueven al heap
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);

    // esto solo muestra el tamano en memoria del apuntador
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    // usar deferrence para mostrar el heap
    println!("boxed_vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle));

    // si se quiere devolver el ownership a un struct, se utiliza el operador deferrence
    // de esta forma se mueve la informacion del heap al stack de nuevo
    let unboxed_vehicle: Shuttle = *boxed_vehicle; 
    println!("vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));


    
}
