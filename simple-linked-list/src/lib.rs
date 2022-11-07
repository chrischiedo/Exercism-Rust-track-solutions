use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        match self.head {
            None => return true,
            _ => return false,
        }
    }

    pub fn len(&self) -> usize {
        let mut temp = &self.head;
        let mut count = 0;

        while let Some(element) = temp {
            count += 1;
            temp = &element.next;
        }
        count
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element: element,
            next: self.head.take(),
        });

        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.element
        })
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed_list = Self::new();
        let mut current_list = self;

        loop {
            match current_list.pop() {
                Some(elem) => reversed_list.push(elem),
                None => return reversed_list,
            }
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_linked_list = Self::new();
        for elem in iter {
            new_linked_list.push(elem)
        }
        new_linked_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = vec![];
        while let Some(elem) = linked_list.pop() {
            vec.insert(0, elem)
        }
        vec
    }
}
