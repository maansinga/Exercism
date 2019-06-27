pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0{
        None
    }else if array[array.len()/2] == key {
        Some(array.len()/2)
    }else{
        if array[array.len()/2] < key {
            match find(&array[(array.len()/2 + 1) ..], key){
                Some(pos) => Some(array.len()/2 + 1 + pos),
                None => None
            }
        }else{
            match find(&array[.. (array.len()/2)], key){
                Some(pos) => Some(pos),
                None => None
            }
        }
    }
}
