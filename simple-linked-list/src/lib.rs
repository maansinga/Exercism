use core::borrow::BorrowMut;

pub struct Node<T>{
    data: Option<T>,
    next: Option<Box<Node<T>>>
}
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{head: None}
    }

    pub fn len(&self) -> usize {
        let mut h = &self.head;
        let mut count = 0;
        loop{
            match h {
                None => break,
                Some(box_node) => {
                    h = &box_node.next;
                    count += 1;
                }
            }
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(
            Box::new(
                Node{
                    data: Some(_element),
                    next: self.head.take()
                }
            )
        );
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none(){
            None
        }else{
            let uhead = self.head.take().unwrap();
            self.head = uhead.next;
            uhead.data
        }

    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_none() {
            None
        }else{
            self.head.as_ref().unwrap().data.as_ref()
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut ret = SimpleLinkedList::new();
        let mut head = &self.head;
        while head.is_some() {
            let uhead = head.as_ref().unwrap();
            ret.push(uhead.data.clone().unwrap());
            head = &uhead.next;
        }

        ret
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut ret = SimpleLinkedList::new();
        for k in _item.iter(){
            ret.push(k.clone())
        }

        ret
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut ret = vec![];
        let mut head = self.head;
        while head.is_some() {
            let uhead = head.unwrap();
            ret.push(uhead.data.unwrap());
            head = uhead.next;
        }

        ret.reverse();
        ret
    }
}
