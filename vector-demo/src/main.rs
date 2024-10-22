use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use std::collections::VecDeque; 
use std::collections::LinkedList;
fn main() {
    let mut fruit = vec!["Orange", "Fig", "Apple", "Peach"];
    // Scramble (shuffle) the fruit
    let mut rng: ThreadRng = thread_rng();
    fruit.shuffle(&mut rng);

    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}\n ", item);
        } else {
            println!("{}", item);
        }
    }
    // VecDeque // O(1) time complexity for operations on both ends / efficient for front and back
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    fruit.push_front("Pomegranate");
    fruit.push_back("Cherry");
    println!("New Fruit Salad from VecDeque");
    for (i, item) in fruit.iter().enumerate() {

    if i != fruit.len() - 1 {
        print!("{}\n ", item);
    } else {
        println!("{}", item);
    }
}

    // LinkedList efficiency with insertion and deletion in list, without carrying about accessing
    // good for inserting or removing from the middle of the list.
    // Higher memory overhead and worse cache locality than Vec or VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();
    fruit.remove(2);
    println!("New Fruit Salad from LinkedList");
    for (i, item) in fruit.iter().enumerate() {

    if i != fruit.len() - 1 {
        print!("{}\n ", item);
    } else {
        println!("{}", item);
    }
}}
