use std::rc::Rc;
use std::cell::RefCell;
use core::borrow::{BorrowMut};

type HeapNode<T> = Rc<RefCell<Box<Node<T>>>>;

pub struct Node<T>{
    data: Option<T>,
    next: Option<HeapNode<T>>
}
pub struct SimpleLinkedList<T> {
    head: Option<HeapNode<T>>
}

impl<T> Node<T>{
    pub fn make_rc_node(data: T)->Option<HeapNode<T>>{
        Some(                                     //Option to make it none
            Rc::new(                              //Option to let multiple references
                RefCell::new(                //Option to allow interior mutability
                    Box::new(                //Allocating memory on heap
                        Node{                   //Actual data
                            data: Some(data),
                            next: None
                        }
                    )
                )
            )
        )
    }
    pub fn len(&self)->usize{
        match &self.next{
            None => 1,
            Some(rc_node)=>{
                1 + rc_node
                    .as_ref()
                    .borrow_mut()
                    .len()
            }
        }
    }

    pub fn append(&mut self, data: T){
        match &self.next{
            None => self.next = Node::make_rc_node(data),
            Some(rc_node) => {
                rc_node //Rc<RefCell<Box<Node<T>>>>
                    .as_ref() //&RefCell<Box<Node<T>>>
                    .borrow_mut() //&Box<Node<T>>
                    .append(data);
            }
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{head: None}
    }

    pub fn len(&self) -> usize {
        match &self.head{
            None => 0,
            Some(rc_node) => {
                rc_node
                    .as_ref()
                    .borrow_mut()
                    .len()
            }
        }
    }

    pub fn push(&mut self, _element: T) {
        match &self.head{
            None => self.head = Node::make_rc_node(_element),
            Some(rc_node)=>{
                rc_node
                    .as_ref()
                    .borrow_mut()
                    .append(_element)
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        unimplemented!()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
