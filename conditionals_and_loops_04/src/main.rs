fn main() {
    let ans:bool = is_even(10);
    
    if ans{
        println!("Number is even");
    }
    else{
        println!("Number is odd");
    }

    let _i: i32;
    for i in 0..10 {
        print!("{} ", i);
    } 
}

fn is_even(x: i32) -> bool{
    return x%2 == 0;
}