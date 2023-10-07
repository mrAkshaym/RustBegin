
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

  /*let a:i32 = 3;
  let a_complement:i32 = !a;

  println!("Complement of {} is {}", a, a_complement);

  let a_ones_complement:i32 = !a+1;
  
  println!("Ones complement of {} is {}", a, a_ones_complement);*/

/*Output 
  Complement of 3 is -4
Ones complement of 3 is -3 */

// Testing logical operators
   //log_oper();

  // log_oper2();

  bit_oper();
}

fn log_oper(){
  let a:bool = true;
  let b:bool = false;

  println!("a and b is {}", a && b);
  println!("a or b is {}", a || b);
  println!("not a is {}", !a);
  println!("not b is {}", !b);
}

fn log_oper2(){
  let ten: i32 = 10;
  let twenty: i32 = 20;

  println!("Is ten > twenty {}", ten > twenty);
  println!("Is ten < twenty {}", ten < twenty);
  println!("Is ten >= twenty {}", ten >= twenty);
  println!("Is ten <= twenty {}", ten <= twenty);
}

fn bit_oper(){  
  let sixteen : i32 = 16;

  println!(" the value of {} is {}", sixteen, sixteen);

  println!("the value of !{} is {}", sixteen, !sixteen);
  println!("the value of one's complement of {} is {}", sixteen, !sixteen+1);

  println!("the value of one left shift of {} is {}", sixteen, sixteen << 1);
  println!("the value of one right shift of {} is {}", sixteen, sixteen >> 1);

  let bit1 : i32 = 1;
  println!(" the value of {}&{} is {}", sixteen, bit1, sixteen & bit1);
  println!("the value of {}|{} is {}", sixteen, bit1, sixteen | bit1);

  /*
  Output
  
   the value of 16 is 16
the value of !16 is -17
the value of one's complement of 16 is -16
the value of one left shift of 16 is 32
the value of one right shift of 16 is 8
 the value of 16&1 is 0
the value of 16|1 is 17
   */
}