// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. 

// expresiones no deben tener punto y coma, de lo contrario se convierten en statements

fn main() {
    let my_number = 10;

    another_function(my_number);

    // las declaraciones pueden englobar bloques complejos para definir su contenido, ejemplo:
    let y = {
        let x  = 3;
        x + 1
    };

    println!("value of y {y}");
    println!("{}", multiply_by_10(100));

}

// definicion de funciones es muy similar a otros lenguajes
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}


fn multiply_by_10(x: i32) -> i32 {
    // en rust se suele retornar el ultimo valor que aparezca en la funcion (sin punto y coma)
    // pero si en algun caso se desea retornar de un punto en particular
    // se puede utilizar la palabra reservada return
    x * 10
}