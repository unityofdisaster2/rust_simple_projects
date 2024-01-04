fn main() {
    // las tuplas pueden tener distintos tipos de dato. No pueden crecer o disminuir su tamano
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // para acceder al valor de la tupla se usa destructure o el operador punto mas el indice

    let (x,y,z) = tup;

    let x1 = tup.0;

    // los arreglos son diferentes a otro lenguaje, aqui son de tamano fijo y una vez definidos no pueden
    // cambiar su tamano, es decir tampoco se pueden agregar o quitar elementos
    // tambien es evidente que todos los valores deben ser del mismo tipo
    // para definir el tipo se debe poner [&tipo_dato; numero_elementos]
    let days_of_week: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];


    // asi se declaran arreglos multidimensionales
    // let multidimensional = [[1,1,1],[2,2,2],[3,3,3]];
    

    // si se quiere inicializar un arreglo con el mismo valor n veces se hace de la siguiente forma
    let a = [3;5]; // [3, 3, 3, 3, 3];
    

    println!("{x} {y} {z}"); 
    println!("{x1}");

    // para acceder a los elementos del arreglo es de la misma forma que en otros lenguajes 
    println!("imprime cuarto dia de la semana {}", days_of_week[3]);
    println!("{}", a[2]);
}
