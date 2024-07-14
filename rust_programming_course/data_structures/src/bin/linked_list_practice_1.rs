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
        Linklist { head: None }
    }

    fn add(&mut self, element: i32) {
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

    fn peek(&self) -> Option<i32> {
        match &self.head {
            Some(previous_head) => {
                Some(previous_head.as_ref().element)
            }
            None => None
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
    let mut list = Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    println!("{:?}", list.peek());
}
