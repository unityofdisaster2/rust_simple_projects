
// para mostrar el contenido del struct al depurar hay que agregar el atributo derive

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// los structs pueden tener metodos que se definen con la palabra reservada impl + el nombre del struct
// el primer parametro de los metodos siempre es "self" que representa la instancia del struct del que
// el metodo esta siendo llamado. Nota &self es una forma corta de escribir (self: &Self)
// el compilador acepta tambien que haya multiples bloques impl, pero no es necesario 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // es valido nombrar un metodo con el mismo nombre de un campo del struct

    fn width(&self) -> bool {
        self.width > 0
    }

    // los metodos tambien pueden recibir otros parametros aparte del self
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width >= rectangle.width && self.height >= rectangle.height
    }

    // se pueden agregar funciones asociadas que no usen el parametro self
    // la funcion square seria una especie de constructor para transformar el
    // rectangulo en un cuadrado
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    // funcion asociada, por lo general se utiliza para crear nuevas instancias
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}


fn main() {
    let rect1 = Rectangle::new(30, 50);

    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&rect1)
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // para llamar a una funcion asociada se usa la sintaxis de ::
    println!("Create square: {}", Rectangle::square(5).area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));

    // para mostrar el output de un elemento que haya sido marcado para depurar hay que usar
    // {:?} o {:#?}
    println!("rect1 is {:?}", rect1);

    // otra forma de mostrar el contenido de un elemento en modo depuracion es la macro dbg!
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}