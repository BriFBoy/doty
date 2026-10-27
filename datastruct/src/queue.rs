use crate::linked_list::LinkedList;

pub struct Queue<T: Clone> {
    list: LinkedList<T>,
}

impl<T: Clone> Queue<T> {
    pub fn take() {}

    pub fn append(self, value: T) -> Queue<T> {
        Queue {
            list: self.list.add_to_head(value),
        }
    }
}
