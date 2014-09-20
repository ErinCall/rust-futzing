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

    fn head<'a>(&'a self) -> Option<&'a T> {
        match *self {
            Null => None,
            Node(ref x, _) => Some(x),
        }
    }

    fn tail<'a>(&'a self) -> &'a List<T> {
        match *self {
            Null => self,
            Node(_, box ref next) => next,
        }
    }
}

fn main() {
    let empty:List<int> = Null;
    let one = Node(8i, box Null);
    let two = Node(4i, box Node(8i, box Null));

    assert!(empty.length() == 0);
    assert!(one.length() == 1);

    assert!(empty.head() == None);
    assert!(one.head() == Some(&8i));
    assert!(two.head() == Some(&4i));

    assert!(empty.tail().head() == None);
    assert!(one.tail().head() == None);
    assert!(two.tail().head() == one.head());
    println!("Hurray!")
}
