use crate::linked_list::{LinkedList, LinkedListIterator};

pub struct Queue<T: Clone> {
    list: Option<LinkedList<T>>,
}

impl<T: Clone> Queue<T> {
    pub fn take(&mut self) -> Option<T> {
        let Some(list) = &mut self.list else {
            return None;
        };
        let length = list.iter().count();
        if length == 1 {
            let node = self.list.take()?;
            Some(node.get_value().clone())
        } else {
            list.remove(length - 1).map(|v| v.get_value().clone())
        }
    }

    pub fn append(&mut self, value: T) {
        match &mut self.list {
            Some(l) => l.add_to_head(value),
            None => self.list = Some(LinkedList::new(value)),
        }
    }

    pub fn new(value: Option<T>) -> Queue<T> {
        match value {
            Some(value) => Queue {
                list: Some(LinkedList::new(value)),
            },
            None => Queue { list: None },
        }
    }
    pub fn iter(&self) -> Option<LinkedListIterator<'_, T>> {
        self.list.as_ref().map(|l| l.iter())
    }
}

#[cfg(test)]
mod test {

    use crate::queue::Queue;

    #[test]
    pub fn append_three() {
        let mut array = [1, 2, 3, 4];
        let mut queue = Queue::new(array[0]);
        queue.append(array[1]);
        queue.append(array[2]);
        queue.append(array[3]);

        array.reverse();
        assert_eq!(
            queue
                .iter()
                .unwrap()
                .map(|f| *f.get_value())
                .collect::<Vec<isize>>(),
            array
        );
    }

    #[test]
    pub fn take_one() {
        let mut array = [1, 2, 3, 4].to_vec();
        let mut queue = Queue::new(array[0]);
        queue.append(array[1]);
        queue.append(array[2]);
        queue.append(array[3]);
        array.reverse();

        let take_one = queue.take();
        array.remove(3);

        assert_eq!(take_one.unwrap(), 1);
        assert_eq!(
            queue
                .iter()
                .unwrap()
                .map(|f| *f.get_value())
                .collect::<Vec<isize>>(),
            array
        );
    }
    #[test]
    pub fn take_two() {
        let mut array = [1, 2, 3, 4].to_vec();
        let mut queue = Queue::new(array[0]);
        queue.append(array[1]);
        queue.append(array[2]);
        queue.append(array[3]);
        array.reverse();

        let take_one = queue.take();
        let take_two = queue.take();
        array.remove(3);
        array.remove(2);

        assert_eq!(take_one.unwrap(), 1);
        assert_eq!(take_two.unwrap(), 2);
        assert_eq!(
            queue
                .iter()
                .unwrap()
                .map(|f| *f.get_value())
                .collect::<Vec<isize>>(),
            array
        );
    }
    #[test]
    pub fn take_more_than_queue_length() {
        let array = [1, 2].to_vec();
        let mut queue = Queue::new(array[0]);
        queue.append(array[1]);

        queue.take();
        queue.take();
        let ret = queue.take();

        assert!(ret.is_none());
        assert!(queue.iter().is_none());
    }
}
