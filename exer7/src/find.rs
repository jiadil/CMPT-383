pub fn find_elt<T: Eq>(values: &Vec<T>, elt: T) -> Option<usize> {
    let mut i = 0;
    for value in values {
        if *value == elt {
            return Some(i);
        }
        i += 1;
    }
    None
}
