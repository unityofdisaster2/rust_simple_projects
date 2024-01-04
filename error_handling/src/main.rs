// errores recuperables: aun se puede hacer algo para resolverlo
// ejemplo: file not found

// errores irrecuperables: no hay ninguna alternativa
// ejemplo: index beyond array bounds

use std::fs;

use std::io;


fn main() {
    // macro panic! se utiliza para errores irrecuperables
    // panic!("Houston,  we've had a problem");

    // let countdown = [5, 4, 3, 2, 1, 0];

    // for count in countdown.iter() {
    //     println!{"T-minus {}", count};
    //     // error division entre cero
    //     let x = 1 / count;
    // }

    read_file(String::from("the_ultimate_question.txt"));
}

fn read_file(path: String) {
    // potencialmente puede fallar si no existe el archivo
    // Se puede utilizar el metodo spec en un Result para personalizar el mensaje
    // de error
    // let file_content = fs::read_to_string(path)
    // .expect("nobody knows the ultimate question!");


    // una mejor forma para manejar posibles errores de un Result es utilizar un match
    // para tener una logica para cada caso posible
    let result = fs::read_to_string(path);

    let content = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            // el enum ErrorKind tiene una lista de errores comunes de I/O 
            io::ErrorKind::NotFound => String::from ("File not found"),
            io::ErrorKind::PermissionDenied => String::from ("Permission denied"),
            _ => panic!("Another type or error {:?}", error)

        }
    };

    // si la funcion retorna un Result o un Option se puede usar el operador "?"
    // como una version corta del match

    
    println!("content is: {:?}", content);
}
