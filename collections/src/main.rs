
// los hashmap no estan en el prelude, por lo tanto se debe importar
use std::collections::HashMap;

fn main() {

    // vectores tienen tamano dinamico (se guardan en el heap)

    let mut astronauts:Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("astronauts is {:?}", astronauts);

    // se debe usar el operador de referencia para asignar el valor a una variable
    // independiente, de lo contrario se pierde el ownership en el vector
    // eso o implementar el trait Copy o usar funcion get()
    //let third = &astronauts[2];
    let third = astronauts.get(2);

    println!("astronauts is {:?}", third);

    // si queremos inicializar un vector con valores predeterminados 
    // se puede utilizar la macro vec! y un arreglo con los valores
    let countdown = vec![5,4,3,2,1];

    println!("countdown is {:?}", countdown);


    // Hashmaps

    let mut mission_flown = HashMap::new();
    // los valores no estan ordenados secuencialmente
    mission_flown.insert("Hadfield", 3);
    mission_flown.insert("Hurley", 3);
    mission_flown.insert("Barron", 0);

    println!("missions flown is: {:?}", mission_flown);

    // obtener valor 
    let barron_missions = mission_flown.get("Barron");

    println!("Barron mission is: {:?}", barron_missions);

    // actualizar

    mission_flown.insert("Barron", 2);

    // solo inserta si no existe la llave

    mission_flown.entry("Stone").or_insert(10);
    let kayla = mission_flown.entry("Barron").or_insert(0);
    // otra forma de actualizar es obtener la referencia del entry y operar sobre el deferrence
    *kayla += 1;
    

    println!("new missions flown is: {:?}", mission_flown);

}
