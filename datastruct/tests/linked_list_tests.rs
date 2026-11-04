use datastruct::linked_list::LinkedList;

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
        list.iter().map(|f| *f.get_value()).collect::<Vec<_>>(),
        array.to_vec()
    );
}
#[test]
fn remove_middle() {
    let mut array = [3, 5, 6, 7, 8, 9].to_vec();
    let mut list = LinkedList::new(array[0]);
    list.add_to_tail(array[1]);
    list.add_to_tail(array[2]);
    list.add_to_tail(array[3]);
    list.add_to_tail(array[4]);
    list.add_to_tail(array[5]);

    list.remove(3);
    array.remove(3);

    assert_eq!(
        list.iter().map(|f| *f.get_value()).collect::<Vec<_>>(),
        array.to_vec()
    );
}
#[test]
fn remove_second() {
    let mut array = [3, 5, 6].to_vec();
    let mut list = LinkedList::new(array[0]);
    list.add_to_tail(array[1]);
    list.add_to_tail(array[2]);

    list.remove(1);
    array.remove(1);

    assert_eq!(
        list.iter().map(|f| *f.get_value()).collect::<Vec<_>>(),
        array.to_vec()
    );
}
#[test]
fn remove_returns_correct_value() {
    let mut array = [3, 5, 6].to_vec();
    let mut list = LinkedList::new(array[0]);
    list.add_to_tail(array[1]);
    list.add_to_tail(array[2]);

    let return_value = *list.remove(1).unwrap().get_value();
    let array_return = array.remove(1);

    assert_eq!(return_value, array_return);
    assert_eq!(
        list.iter().map(|f| *f.get_value()).collect::<Vec<_>>(),
        array.to_vec()
    );
}
#[test]
fn remove_first() {
    let mut array = [3, 5, 6].to_vec();
    let mut list = LinkedList::new(array[0]);
    list.add_to_tail(array[1]);
    list.add_to_tail(array[2]);

    list.remove(0);
    array.remove(0);

    assert_eq!(
        list.iter().map(|f| *f.get_value()).collect::<Vec<_>>(),
        array.to_vec()
    );
}
#[test]
fn remove_out_of_bound() {
    let array = [3, 5, 6].to_vec();
    let mut list = LinkedList::new(array[0]);
    list.add_to_tail(array[1]);
    list.add_to_tail(array[2]);

    list.remove(1322);

    assert_eq!(
        list.iter().map(|f| *f.get_value()).collect::<Vec<_>>(),
        array.to_vec()
    );
}
#[test]
fn adding_heads_preserves_existing_nodes() {
    let mut list = LinkedList::new(3);
    list.add_to_head(5);
    list.add_to_head(7);

    let values: Vec<_> = list.iter().map(|node| *node.get_value()).collect();
    assert_eq!(values, vec![7, 5, 3]);
}
#[test]
fn works_with_non_copy_values() {
    let mut list = LinkedList::new(String::from("first"));
    list.add_to_tail(String::from("second"));

    let values: Vec<_> = list.iter().map(|node| node.get_value().as_str()).collect();

    assert_eq!(values, vec!["first", "second"]);
}
