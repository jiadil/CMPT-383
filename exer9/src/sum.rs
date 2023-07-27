pub fn sum_loop_index(vector: &Vec<i64>) -> i64 {
    // TODO: counting from 0 to v.len()-1 and index into the vector in a for loop (i.e. C-style).
    let mut sum = 0;
    for i in 0..vector.len() {
        sum += vector[i];
    }
    sum
}

pub fn sum_loop_iter(vector: &Vec<i64>) -> i64 {
    // TODO: iterate the vector in a for loop (for value in vector) and add the values.
    let mut sum = 0;
    for value in vector {
        sum += value;
    }
    sum
}

pub fn sum_fold(vector: &Vec<i64>) -> i64 {
    // TODO: using an iterator over the vector (vector.iter() or vector.into_iter()) and its .fold method.
    vector.iter().fold(0, |acc, x| acc + x)
}

pub fn sum_method(vector: &Vec<i64>) -> i64 {
    // TODO: using an iterator over the vector and its .sum method.
    vector.iter().sum()
}
