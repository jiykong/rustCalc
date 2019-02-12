use std::io;

fn main() {
    println!("Hello, welcome to the calculator :D!");

    let mut input1 = String::new();
    let mut oper = String::new();
    let mut input2 = String::new();

    loop{

        println!("What is the first number?");
        io::stdin().read_line(&mut input1)
            .expect("need a number");
        let input1: u32 = match input1.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };


    }
}
