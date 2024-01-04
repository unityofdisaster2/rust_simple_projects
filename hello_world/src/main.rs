fn main() {
    
    // signed int 8 bits
    // variables are not mutable by default so you need to add mut if you want to change the content later
    let mut x: i8 = -10;
    // unsigned int 8 bits
    let mut y: u8 = 255;
    println!("x is {}", x);
    // compila, pero hay overflow en esta variable
    y -=1;
    println!("y is {}", y);

    //floating point variables

    let mut fl1: f32 = 4.55;
    let mut fl2: f64 = 5.55;

    // nota: por lo general las macro (funciones que terminan en !) son funciones built-in de rust

}
