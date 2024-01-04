use std::env;
use std::fs;
use std::io::Write;
mod test;

fn main() {
    if env::args().len() <= 2 {
        println!("input must have at least two arguments!");
        return;
    }

    let path = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();

    println!("archivo: {}, nombre: {}", path, name);

    is_in_roster(path, name);

    // for (index, argument) in env::args().enumerate() {
    //     println!("argument {} is {}", index, argument);
    // }

    // let arg2 = env::args().nth(2).unwrap();

    // println!("the 2nd argument is: {}", arg2);

    // read_from_files();
    // test::pruebas();


}

fn read_from_files() {
    let file_content = fs::read_to_string("src/planets.txt").unwrap();
    println!("file content is: {}", file_content);

    for line in file_content.lines() {
        println!("line is {}", line);
    }

    let file_content_bytes = fs::read("src/planets.txt").unwrap();

    println!("reading file in byte mode: {:?}", file_content_bytes);
    println!("writing a file...");
    write_to_file();
}

fn write_to_file() {
    let mut speech = String::new();

    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");
    fs::write("speech.txt", speech);

    let mut planet_file = fs::OpenOptions::new().append(true).open("src/planets.txt").unwrap();

    planet_file.write(b"\nPluto");

}

fn is_in_roster(path: String, name: String) {
    let list_of_names = fs::read_to_string(path).unwrap();
    for line in list_of_names.lines() {
        if name == line {
            println!("{} is in the list", name);
            return;
        }
    }
    println!("{} is not in the list", name);

}