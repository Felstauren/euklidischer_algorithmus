fn main() {
    
    let a:i32 = 768;
    let b:i32 = 912;
    
    let result:i32 = division(a, b);
    
    println!("{}", result);
}

fn division(a:i32 , b:i32) -> i32 {
    if b == 0{
        return a;
    }
    let koffizent:i32 = a / b;
    let rest:i32 = a - (koffizent*b);

    let result:i32 = division(b, rest);
    return result
}
