// pub mod rational;
// pub mod sort;

use std::any::type_name;
use std::mem::size_of;


fn main() {
    // nothing is required here, but you may want to use it for testing.
    let numbers = vec![1, 2, 3, 4, 5];
    let fruits = vec!["apple", "banana", "orange"];

    let iter_numbers = numbers.iter();
    let iter_fruits = fruits.iter();

    // Use type_name to print the type of the iterator elements
    println!("Type of iter_numbers: {}", type_name::<&i32>());
    println!("Type of iter_fruits: {}", type_name::<&&str>());

    // Size of the iterator elements
    // println!("Size of iter_numbers element: {} bytes", size_of::<&i32>());
    // println!("Size of iter_fruits element: {} bytes", size_of::<&&str>());

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    let mut v2 = Vec::from([1, 6, 2, 4, 8, 2, 1, 4]);
    // sort::quicksort(&mut v2);
    println! {"{:?}",v2};
    print_type_of(&v2);
    let mut iterator = v2.iter();
    println!("Type of iterator: {}", type_name::<&i32>());
    println!("{:?}", iterator);
    print_type_of(&iterator);

    let v3 = Vec::from([1, 6, 2, 4, 8, 2, 1, 4]);
    let inte = v3.into_iter();
    println!("Type of inte: {}", type_name::<std::vec::IntoIter<i32>>());
    print_type_of(&inte);
    println!("{:?}", inte);

    let mut v4 = Vec::from([1, 6, 2, 4, 8, 2, 1, 4]);
    let mut inte2 = v4.iter_mut();
    println!("Type of inte2: {}", type_name::<std::slice::IterMut<i32>>());
    print_type_of(&inte2);
    println!("{:?}", inte2);

}
