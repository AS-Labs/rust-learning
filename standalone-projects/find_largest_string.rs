//
// Passing two strings and returning a result which contains the largest of the two
//

fn largest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let str1 = "thisasmallerstring";
    let str2 = "thisisthelargerstring";
    let result = largest(str1, str2);

    println!("The Largest string is: {}", result);
}
