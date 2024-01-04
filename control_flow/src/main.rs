fn main() {
    let number = 6;

    // bloque if else no se envuelve entre parentesis
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // se puede usar if para asignaciones. Algo similar al operador ternario

    let condition = true;

    // obviamente el valor de if y else debe ser del mismo tipo
    let number  = if condition {5} else {6};

    println!("the value of the number is: {}", number);

    // tipos de estructura de repeticion

    // loop es un ciclo infinito. Se usa principalmente para reintentar operaciones que sabemos que pueden fallar
    // como validar si un hilo ya ha completado su trabajo. igual se puede usar para pasar un resultado

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is: {result}");

    // se puede desambiguar uso de multiples loops etiquetandolos con un nombre

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // rompe el loop mas cercano
                break;
            }
            if count == 2 {
                // rompe el loop etiquetado como counting_up
                break 'counting_up;
            }
            remaining -= 1;
        }
        count +=1;
    }

    println!("End count {count}");

    // ciclos while

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    // se puede usar while para recorrer arreglos

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // pero es mucho mejor hacerlo con un for

    for element in a {
        println!("element taken from for: {}", element);
    }


    println!("probando recursividad {}", factorial(5));
}


fn factorial(x: i64) -> i64 {
    if x == 1 {
        return 1;
    }
    x * factorial(x - 1)
}