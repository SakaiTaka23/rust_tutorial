use crate::ListNode::Node;

#[derive(Debug)]
enum ListNode<T> {
    Node(T, Option<Box<ListNode<T>>>),
}

fn main() {
    let list = Node(
        1, Some(Box::new(
            Node(
                2, Some(Box::new(
                    Node(
                        3, Some(Box::new(
                            Node(
                                4, None)
                        )),
                    )
                )
                ),
            )
        )),
    );
    println!("{:?}", list);
}
