
// value is one of a possible set of values
// enum IpAddrKind {
//     V4,
//     V6,
// }

// se pueden agregar datos dentro de cada variante de un enum haciendo mas facil
// el manejo de datos sin tener que crear un struct

enum IpAddrKind {
    V4(String),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// los structs tambien pueden tener una cantidad distinta de datos asociados, por ejemplo se puede
// usar 4 parametros u8 para representar la direccion de ipV4 y un string para V6
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

// en los enums tambien se pueden definir metodos
impl Message {
    fn call(&self) {
        // TODO
    }
}


fn main() {
    // crear instancias de elementos de un enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    // let home = IpAddrKind::V4(127, 0, 0, 1));
    // let loopback = IpAddrKind::V6(String::from("::1"));
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    value_in_cents(Coin::Quarter((UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // aunque match es exhaustivo se puede usar un parametro para cachar todas las opciones
        // que no se hayan considerado en un ultimo valor, similar pero no igual al default en la operacion
        // switch
        other => move_player(other),
        // tambien hay otro patron para el catch call en caso de que no queramos usar el valor (ignorar)
        // para esto se debe usar el caracter "_"
        // _ => reroll()
    }

    let config_max = Some(3u8);
    // acceder al valor interno de some de forma rapida e ignorando el caso donde es posible que el valor
    // sea none
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}" , max);
    }
    // se puede agregar else a este tipo de estructura que seria equivalente a poner  "_ =>" en el match 
}


// los enums son utiles en conjunto con la operacion match, ya que se puede definir un resultado
// sobre cada una de las variantes del enum (algo que se debe tener claro es que la operacion arms)
// match es exhaustiva y debe cubrir todos casos posibles con ese struct, no solo unos cuantos
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // en la expresion match se puede agregar un parametro a las variantes para obtener el valor
        // interno de una de las variantes
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// se puede usar la operacion match para extraer valores del enum Option<T> 
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // en esta operation se retorna el valor que hay en el Some en otro some pero sumado con uno
        Some(i) => Some(i + 1)
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}