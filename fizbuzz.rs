use std::io::print;
use std::io::println;

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
            println(x.to_str().as_slice());
        }
    }
}
