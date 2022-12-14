use core::fmt::{Debug, Formatter};
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    println!("The array is: {:?}", &arr);
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            println!("i is {}", i);
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut ve1);
        assert!(is_sorted(&ve1));
    }

    #[test]
    fn ascending() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }
}

fn main() {
    let mut vec1 = vec![3, 4, 2, 4, 1, 3, 19];
    bubble_sort(&mut vec1);
}
