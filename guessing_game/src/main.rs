// rust por defecto ya trae algunas funciones de la libreria estandar al scope del programa (prelude)
// como aparentemente io no esta dentro del prelude se debe traer explicitamente
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // thread_rng genera el seed aleatorio usando el sistema operativo
    // gen usa como parametro la range expression start..=end
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {

        println!("Please input your guess.");
        
        let mut guess = String::new();
    
        // en rust las variables y las referencias son inmutables por defecto. Si se quiere lograr lo contrario
        // hay que especificarlo
    
        // en rust el simbolo de referencia es: &. Si se quiere mutar la referencia a memoria se debe poner &mut
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // se puede imprimir de dos formas, una es con {var_name} y la otra es ("{}",var_name)
        println!("You guessed: {guess}");
    
        // concepto de shadowing variables o reusar el nombre
    
        // trim quita espacios y saltos de linea, parse convierte el valor en numero
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // cmp compara dos valores y retorna un Ordering enum
        // match: condicional con la que decidimos que hacer con la salida (se me hace como un switch)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    
}


// cargo build para construir proyecto. Cargo update para actualizar dependencias