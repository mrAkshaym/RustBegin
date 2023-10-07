fn main(){
    // Adding code to demo code flow in rust

    // demo_ifelse();
     // demo_ifelse2();

     // demo_enum();

     demo_options()


}

fn demo_ifelse(){
    let a:i32 = 3;
    let b:i32 = 2;

    if a>b {
        println!("{} is greater than {}", a, b);
    } else {
        println!("{} is greater than {}", b, a);
    }
}


fn demo_ifelse2(){
    let mut name:String = String::new();

    println!("Enter you name");
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    
    if str::eq(&name.trim(), "Akshay"){
        println!("Hello Akshay, how are you?");
    } else if str::eq(&name.trim(), "Raj"){
        println!("Hello Raj, how are you?");
    }
    else{
        println!("Hello {}", name);
    }
    /*
    Output:
    PS C:\myRust\RustBegin\rust-test\ControlFlow> .\main.exe
Enter you name
Akshay
Hello Akshay, how are you?
PS C:\myRust\RustBegin\rust-test\ControlFlow> .\main.exe
Enter you name
Raj
Hello Raj, how are you?
PS C:\myRust\RustBegin\rust-test\ControlFlow> .\main.exe
Enter you name
Dinesh
Hello Dinesh
     */
}

enum Color{
    Red = 5,
    Green,
    Blue,
    White,
    Black
}
fn demo_enum() {
println!("Color Red is {}", Color::Red as u8);
println!("Color Green is {}", Color::Green as u8);
println!("Color Black is {}", Color::Black as u8);
}

fn demo_options(){
    let name:String = String::from("Akshay Mahajan");

    //let letter:Option<char> = name.chars().nth(14);
    //let letter2:char = name.chars().nth(1).unwrap();

    //println!("First letter is {}", letter.unwrap());
    //println!("Second letter is {}", letter2);
    /* Output
    1. 
    let letter:Option<char> = name.chars().nth(0);
    let letter2:char = name.chars().nth(1).unwrap();

    First letter is A
    Second letter is k

    2. 
    let letter:Option<char> = name.chars().nth(0);
    let letter2:char = name.chars().nth(1).unwrap();

    thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', .\main.rs:74:45
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */

     let letter:Option<char> = name.chars().nth(14);

     let value: String = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value")
     };

     println!("Value is {}", value);
     /*
     Output
     
     1.let letter:Option<char> = name.chars().nth(1);
    Value is k
    
     2.    let letter:Option<char> = name.chars().nth(14);
    Value is No value
      */
}

