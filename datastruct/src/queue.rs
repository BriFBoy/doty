use crate::linked_list::{LinkedList, LinkedListIterator};

pub struct Queue<T: Clone> {
    list: LinkedList<T>,
}

impl<T: Clone> Queue<T> {
    pub fn take(&mut self) -> Option<T> {
        let lenght = self.list.iter().count();
        self.list.remove(lenght - 1).map(|v| v.get_value().clone())
    }

    pub fn append(&mut self, value: T) {
        self.list.add_to_head(value)
    }

    pub fn new(value: T) -> Queue<T> {
        Queue {
            list: LinkedList::new(value),
        }
    }
    pub fn iter(&self) -> LinkedListIterator<'_, T> {
        self.list.iter()
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
            queue.iter().map(|f| *f.get_value()).collect::<Vec<isize>>(),
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

        queue.take();
        array.remove(3);

        assert_eq!(
            queue.iter().map(|f| *f.get_value()).collect::<Vec<isize>>(),
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

        queue.take();
        queue.take();
        array.remove(3);
        array.remove(2);

        assert_eq!(
            queue.iter().map(|f| *f.get_value()).collect::<Vec<isize>>(),
            array
        );
    }
}
