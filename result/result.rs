fn test_return(i: i32) -> Result<(), String> {
    match i {
        0 => return Err("Error".to_owned()),
        _ => Ok(())
    }
}

fn body() -> Result<(), String> {
    let res = test_return(0);
    println!("text here {:?}", res); //=> text here Err("Error")
    Ok(())
}

fn main() {
    println!("Hello : {:?}",body()); //=> Hello : Err("Error")
}