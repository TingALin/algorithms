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
