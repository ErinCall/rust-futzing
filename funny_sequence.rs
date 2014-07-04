fn main (){
    let mut x = 5u;
    loop {
        x += x - 3;
        if x == 3 { break }
        println!("{}", x);
    }
}
