use std::mem;

#[derive(Clone)]
pub struct LinkedList<T: Clone> {
    value: T,
    next: Option<Box<LinkedList<T>>>,
}
pub struct LinkedListIterator<'a, T: Clone> {
    next: Option<&'a LinkedList<T>>,
}

impl<'a, T: Clone> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a LinkedList<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        self.next = current.next.as_deref();
        Some(current)
    }
}
impl<T: Clone> LinkedList<T> {
    pub fn new(value: T) -> LinkedList<T> {
        LinkedList { value, next: None }
    }
    pub fn get_value(&self) -> &T {
        &self.value
    }
    pub fn add_to_head(&mut self, value: T) {
        let next = self.next.take();
        let old_value = mem::replace(&mut self.value, value);
        self.next = Some(Box::new(LinkedList {
            value: old_value,
            next,
        }))
    }

    pub fn remove(&mut self, index: usize) -> Option<LinkedList<T>> {
        if index == 0 {
            let returned_list_node = LinkedList::new(self.value.clone());
            *self = *self.next.take()?;
            return Some(returned_list_node);
        }
        let mut last_node = self;
        for i in 0..=index {
            if i >= (index - 1) {
                let removed_node = &mut last_node.next.take()?;
                let next = removed_node.next.take();

                last_node.next = next;

                return Some(*removed_node.clone());
            }
            last_node = last_node.next.as_deref_mut()?;
        }
        None
    }
    pub fn add_to_tail(&mut self, value: T) {
        match &mut self.next {
            Some(e) => e.add_to_tail(value),
            None => self.next = Some(Box::new(LinkedList::new(value))),
        }
    }
    pub fn iter(&self) -> LinkedListIterator<'_, T> {
        LinkedListIterator { next: Some(self) }
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn new_linked_list() {
        let list = LinkedList::new(3);

        assert_eq!(list.get_value(), &3);
        assert!(list.next.is_none());
    }

    #[test]
    fn add_new_head() {
        let mut list = LinkedList::new(3);
        list.add_to_head(5);

        assert_eq!(list.get_value(), &5);
        assert!(list.next.is_some());
    }
}
