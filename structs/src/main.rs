struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// tambien existen structs parecidos a tuplas llamados tuple structs
// en estos no es necesario nombrar las propiedades, solo poner el tipo
// para acceder a sus valores es similar a las tuplas, es decir usando
// el .0, .1 etc
struct Color(i32, i32, i32);

// tambien se pueden definir structs sin campos!
// estos se llaman unit-like structs

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    // crear instancias de otras instancias, es parecido usar el object spread en javascript

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0,0,0);

    let subject = AlwaysEqual;

    user1.email = String::from("anotheremail@gmail.com");
}


fn build_user(email: String, username: String) -> User {
    // si el ultimo statement de la funcion es un struct se utilizara como valor de retorno

    // se puede usar el field shorthand como en javascript, es decir si el nombre del campo 
    // es igual al nombre del parametro se puede usar solo el nombre del campo y omitir el otro
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}