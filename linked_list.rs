enum List<T> {
    Node(T, Box<List<T>>),
    Null
}

impl<T> List<T> {
    fn length(&self) -> int {
        match *self {
            Null => 0,
            Node(_, ref next) => 1i + next.length(),
        }
    }
}

fn main() {
    let empty:List<int> = Null;
    let one = Node(8i, box Null);

    assert!(empty.length() == 0);
    assert!(one.length() == 1);
    println!("Hurray!")
}
