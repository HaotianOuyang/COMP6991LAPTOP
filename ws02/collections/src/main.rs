use std::collections::{HashMap, LinkedList, VecDeque};

const MAX_ITER: i32 = 300000;

fn main() {
    // Vectors
    vec_operations();

    // VecDeque
    vec_deque_operations();

    // TODO: your code here, for linked list insertions
    linked_list_operations();
    // TODO: your code here, for hashmap insertions
    hashmap_operations();
    // TODO: your text explanation to the questions in the spec
    //As the result from Terminal The VecDeque can be the fastest in these collections
    //I think this is due to the special structure of the VecDeque. It just need to remove the head,
    //and push to the back. 
    //There are significant difference between Vec and VecDeuqe deletion
    //Because the push back and pop_front function in Vector will just appends an element to the back of or remove to the head of the deque
    //But the remove in vector will remove and retutrn the element and shifting all elemnts after it to the left which is very consuming.
    //I might use VecDeque over vec when: I need a queue, double-ended queue and a Vec that need more efficient insertion at both ends of the 
    //sequence to improve the performance to my program
    //I might use LinkedList over vec when: I want the vec and vecDeque for unknown size, and can't tolerate amorization
    //also effciently split and append lists, absolutely want a doubly linked list.
    //The result didn't suprise me because the data structure I've learnt before, remove in some structure is always time consuming. Because it takes index
    //sometimes it needs to traverse all the other element. 
}       

/// measure the insertion and removal
/// operations of a vector
fn vec_operations() {
    let mut vec = Vec::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec.push(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Vector ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec.remove(0);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

/// measure the insertion and removal
/// operations of a VecDeque
fn vec_deque_operations() {
    let mut vec_deque = VecDeque::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec_deque.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== VecDeque ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec_deque.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

fn linked_list_operations(){
    let mut linked_list = LinkedList::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER{
        linked_list.push_back(i);
    }
    let time_end = std::time::Instant::now();
    println!("==== LinkedList ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER{
        linked_list.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}


fn hashmap_operations(){
    let mut hash_map = HashMap::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER{
        hash_map.insert(i, i);
    }
    let time_end = std::time::Instant::now();
    println!("==== HashMap ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER{
        hash_map.remove(&i);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}