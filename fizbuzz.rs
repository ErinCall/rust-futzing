use std::io::print;

fn main() {
    for x in range(1u, 100) {
        if x % 3 == 0 || x % 5 == 0 {
            if x % 3 == 0 {
                print("fizz");
            }
            if x % 5 == 0 {
                print("buzz");
            }
            print("\n");
        } else {
            println!("{}", x); // this doesn't seem like a great way to do the int->string conversion :\
        }
    }
}
