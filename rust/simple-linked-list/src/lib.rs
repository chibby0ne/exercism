#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T>
where
    T: std::fmt::Debug,
{
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut n = self.head.as_ref();
        while n.is_some() {
            if let Some(val) = n {
                count += 1;
                n = val.next.as_ref();
            }
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        dbg!(self.head.as_ref());
        let x = self.head.take();
        dbg!(x.as_ref());
        dbg!(self.head.as_ref());
        let new_node = Some(Box::new(Node {
            data: _element,
            next: x,
        }));
        dbg!(new_node.as_ref());
        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            // Update the head by popping the current head
            let head = self.head.take().unwrap();
            self.head = head.next;
            // Return the old head's value
            Some(head.data)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(b) => Some(&b.data),
            None => None,
        }
    }
}

impl<T: Clone> SimpleLinkedList<T>
where
    T: std::fmt::Debug,
{
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut linked_list = SimpleLinkedList::new();
        let mut n = self.head.as_ref();
        loop {
            match n {
                Some(val) => {
                    linked_list.push(val.data.clone());
                    n = val.next.as_ref();
                }
                None => {
                    break;
                }
            }
        }
        linked_list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T>
where
    T: std::fmt::Debug,
{
    fn from(_item: &[T]) -> Self {
        let mut linked_list = SimpleLinkedList::new();
        for elem in _item {
            linked_list.push(elem.clone());
        }
        linked_list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: std::fmt::Debug,
{
    fn into(self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();
        let mut n = self.head;
        loop {
            match n {
                Some(val) => {
                    vec.insert(0, val.data);
                    n = val.next;
                }
                None => {
                    break;
                }
            }
        }
        vec
    }
}
