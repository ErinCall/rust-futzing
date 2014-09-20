enum List<T> {
    Node(Box<T>, Box<List<T>>),
    Null
}

// impl<T> List<T> {
//     fn cons <T>(&self, next: &T) -> &List<T> {
//         &Node(next, self)
//     }
// }

fn main() {

}
