use std::io;

fn main() {
    println!("Hello, welcome to the calculator :D!");



    loop{
    let mut input1 = String::new();
    let mut oper = String::new();
    let mut input2 = String::new();
        println!("What is the first number?");
        
        io::stdin().read_line(&mut input1)
            .expect("need a number");

        let input1: u32 = match input1.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("What is the operator?");
        io::stdin().read_line(&mut oper)
            .expect("need a number");

        let oper = oper.trim_end();
        
        println!("What is the second number?");

        io::stdin().read_line(&mut input2)
            .expect("need a number");

        let input2: u32 = match input2.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if oper == "+"{
            let new = input1 + input2;
            println!("{}",new);
        }

        if oper == "-"{
            let new = input1 - input2;
            println!("{}",new);
        }

        if oper == "*"{
            let new = input1 * input2;
            println!("{}",new);
        }

        if oper == "/"{
            let new = input1 / input2;
            println!("{}",new);
        }
    }
}
