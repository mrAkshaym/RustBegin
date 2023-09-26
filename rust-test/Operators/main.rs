
fn main(){
   /*
    let square_int_int: i32 = i32::pow(3,2);
    println!("Square of 3 is {}", square_int_int);

    let square_float_int : f64 = f64::powi(3.14,2);
    println!("Square of 3.14 is {}", square_float_int);

    let square_float_float : f64 = f64::powf(3.14,2.2);
    println!(" 3.14 raised to 2.2 is {}", square_float_float);

    //let square_int_float : f64 = f64::powf(3,2.2); // mismatched types error
    let square_int_float : f64 = f64::powf(3.0,2.2);
    println!(" 3.0 raised to 2.2 is {}", square_int_float);
    */

    /*
    Output
    
    Square of 3 is 9
Square of 3.14 is 9.8596
 3.14 raised to 2.2 is 12.394962744865618
 3.0 raised to 2.2 is 11.211578456539659
  */

  // trying typecasting

  /*let a:i32 = 3;
  let b:f64 = 2.2;

  //let sq:f64 = f64::powf(a, b); //error[E0308]: mismatched types
  let sq:f64 = f64::powf(a as f64, b);
  println!("{} raised to {} is {}", a, b, sq);*/
  /* Output:
  3 raised to 2.2 is 11.211578456539659
   */

  let a:i32 = 3;
  let a_complement:i32 = !a;

  println!("Complement of {} is {}", a, a_complement);

  let a_ones_complement:i32 = !a+1;
  println!("Ones complement of {} is {}", a, a_ones_complement);
/*Output */
  Complement of 3 is -4
Ones complement of 3 is -3 */
}