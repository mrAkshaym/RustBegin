fn main(){
    //println!("Result of div is {}",func(13,3));
   // println!("Result of div is {}",func(13,0));
}

fn func(num:i32, den:i32) -> Result<i32, String>{
    if den ==0{
        return Err(String::from("Denominator cannot be zero"));
    }

    return Ok(num/den);
}