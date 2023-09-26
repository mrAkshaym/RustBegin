
fn main(){

  /* 
  // Scope of variable is limited to the block
    let name:String = "Akshay".to_string();

    println!("Hello, {}!", name);

    {
        let name:String = "Sonu".to_string();
        println!("Hello, {}!", name);
    }

    println!("Hello, {}!", name);
    */
    /* Output
    Hello, Akshay!
Hello, Sonu!
Hello, Akshay!
     */

    // Shadowing

    let name:String = "Akshay".to_string();

    println!("Hello, {}!", name);

    let name:String = "Sonu".to_string();

    println!("Hello, {}!", name);
    /*Output

    PS C:\myRust\RustBegin\rust-test\shadowing> rustc .\main.rs
PS C:\myRust\RustBegin\rust-test\shadowing> .\main.exe
Hello, Akshay!
Hello, Sonu!
    
     */
}