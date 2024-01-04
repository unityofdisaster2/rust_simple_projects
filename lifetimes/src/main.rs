// cuando se utilizan valores prestados y se intenta retornar alguno, no se tiene la
// certidumbre de si en ese momento aun estaran disponibles en memoria por el scope en el que
// se hayan declarado las variables x o y. Para asegurar que ambas variables estan en el mismo
// lifetime, se debe utilizar una anotacion que inicie con apostrofe mas una palabra
// en este caso: 'a. Agregando esta anotacion se asegura que ambas variables estan en el mismo lifetime
// y no habra problema si se intenta retornar cualquiera de las dos. En el caso de que una de ellas
// este en un scope diferente, se prioriza a la que este en el scope mas cercano, pero aun con eso
// fallara la funcion al intentar ejecutarla. Evidentemente se puede agregar n anotaciones para poder
// trabajar con variables de diferentes lifetimes
fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// si se utilizan referencias en un struct es necesario agregar anotaciones de lifetime
struct Shuttle<'a> {
    name: &'a str
}

impl<'a> Shuttle<'a> {
    // dependiendo de que valor se retorne se determina si es necesario agregar explicitamente
    // anotaciones de lifetime. Por ejemplo si se utiliza el valor del self, no es necesario
    // pero en este caso si se intenta retornar el valor de msg, es necesario agregar una segunda
    // anotacion de lifetime
    fn send_transmission(&self, msg: &str) -> &str {
        println!("Transmitting message: {}", msg);
        &self.name
    }
}



fn main() {
    // las variables anotadas con el lifetime 'static estaran disponibles durante
    // toda la duracion del programa
    let s: &'static str = "Greeting from Neptune";
    // esta variable vive en este scope y cualquier otro anidado
    let propelant: &String;
    // los lifetimes se determinan por el scope en el que se utiliza alguna variable
    {
        // rp1 solo vive dentro de este scope
        let rp1 = String::from("RP-1");
        propelant = &rp1;
        println!("propelant is {}",propelant);
    }

    // esto no se puede porque esta fuera del lifetime de rp1
    // println!("propelant is {}",propelant);


    let result;
    let propelant1 = String::from("RP-1");
    // {
    //     let propelant2 = String::from("LNG");
    //     result = best_fuel(&propelant1, &propelant2);
    // }
    let propelant2 = String::from("LNG");
    result = best_fuel(&propelant1, &propelant2);
    println!("result is: {}", result);
}
