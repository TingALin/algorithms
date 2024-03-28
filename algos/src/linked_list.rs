use crate::linked_list::List::*;

#[allow(dead_code)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

#[allow(dead_code)]
impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn reverse_list_two_pointer(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut cur = head;
    let mut pre = None;
    while let Some(mut node) = cur.take() {
        cur = node.next;
        node.next = pre;
        pre = Some(node);
    }
    pre
}

fn reverse_list_recursive(head: Option<Box<Node>>) -> Option<Box<Node>> {
    fn rev(mut head :Option<Box<Node>>, mut pre :Option<Box<Node>>) -> Option<Box<Node>>{
        if let Some(mut node) = head.take(){
            let cur = node.next;
            node.next = pre;
            pre = Some(node);
        }
        pre
    }
    rev(head, None)
}

#[cfg(test)]
mod linked_list_tests {
    use super::*;

    #[test]
    fn linked_list_base() {
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        assert_eq!(list.len(), 3);
    }
}
