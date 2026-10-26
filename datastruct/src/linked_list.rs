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
    pub fn add_to_head(self, value: T) -> LinkedList<T> {
        LinkedList {
            value,
            next: Some(Box::new(self)),
        }
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

        assert_eq!(list.value, 3);
        assert!(list.next.is_none());
    }

    #[test]
    fn add_new_head() {
        let list = LinkedList::new(3);
        let list = list.add_to_head(5);

        assert_eq!(list.value, 5);
        assert!(list.next.is_some());
    }
    #[test]
    fn add_new_multiple_to_tail() {
        let array = [3, 5, 6, 7, 8, 9];
        let mut list = LinkedList::new(array[0]);
        list.add_to_tail(array[1]);
        list.add_to_tail(array[2]);
        list.add_to_tail(array[3]);
        list.add_to_tail(array[4]);
        list.add_to_tail(array[5]);

        assert_eq!(
            list.iter().map(|f| f.value).collect::<Vec<_>>(),
            array.to_vec()
        );
    }
    #[test]
    fn adding_heads_preserves_existing_nodes() {
        let list = LinkedList::new(3).add_to_head(5).add_to_head(7);

        let values: Vec<_> = list.iter().map(|node| node.value).collect();
        assert_eq!(values, vec![7, 5, 3]);
    }
    #[test]
    fn works_with_non_copy_values() {
        let mut list = LinkedList::new(String::from("first"));
        list.add_to_tail(String::from("second"));

        let values: Vec<_> = list.iter().map(|node| node.value.as_str()).collect();

        assert_eq!(values, vec!["first", "second"]);
    }
}
