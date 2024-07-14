#[derive(Debug)]
struct Linklist {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

impl Linklist {
    fn new() -> Linklist {
        Linklist {
            head: None,
        }
    }

    fn add(&mut self, element: i32) {
        // match self.head {
        //     None => {
        //         let new_node = Some(Box::new(Node {
        //             element,
        //             next: None,
        //         }));
        //     }
        //     Some(previous_head) => {
        //         let new_node = Some((Box::new(Node {
        //             element,
        //             next: Some(previous_head)
        //         })));
        //         self.head = new_node;
        //     }
        // }
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}

fn main() {
    let list = Node {
        element: 1,
        next: None,
    };

    let list = Node {
        element: 1,
        next: Some(
            Box::new(Node {
                element: 2,
                next: None,
            })
        ),
    };

    let list = Linklist {
        head: Some(
            Box::new(
                Node {
                    element: 1,
                    next: None,
                }
            )
        )
    };

    let list = Linklist {
        head: Some(
            Box::new(
                Node {
                    element: 1,
                    next: Some(Box::new(Node {
                        element: 2,
                        next: Some(Box::new(Node {
                            element: 3,
                            next: None,
                        })),
                    })),
                }
            )
        )
    };

    println!("{:?}", &list.head);

    let mut list = Linklist::new();
    for i in 1..5 {
        list.add(i * 5);
    }
    list.remove();
    list.print();
}
