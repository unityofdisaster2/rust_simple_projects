// use core::ops::arith::Add;

fn sum_boxes<T: std::ops::Add<Output = T>>(first_box: Box<T>, second_box: Box<T>) -> Box<T> {
    Box::new(*first_box + *second_box)
}


fn main() {
    let first_number = Box::new(10);
    let second_number = Box::new(120);

    println!("numero: {}", *sum_boxes(first_number, second_number));
}
