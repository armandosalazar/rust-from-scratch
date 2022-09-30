// Recordar la importancia del stack y heap
fn main() {
    let x: u8 = 10;
    let y: String = String::from("Welcome");

    println!("{:p}", &x);
    // Si no clonara el valor daría un error en la línea 9 porque ahora el dueño sería la función add
    // por eso es que se utiliza el clone.
    add(x, y.clone());
    println!("{:p}", &x);
    add_ref(&x, y);
    println!("{:p}", &x);
    iter_arr();
}

fn add(x: u8, _y: String) {
    println!("{:p}", &x);
}

fn add_ref(x: &u8, _y: String) {
    println!("{:p}", x);
}

fn iter_arr() {
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        print!("{} ", number);
    }
}
