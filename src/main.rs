fn _largest_i32(list: &[i32]) -> &i32{
    let mut largest = &list[0];

    for item in list{
        if item > largest {largest = item;}
    }

    &largest
}
fn _largest_char(list: &[char]) -> &char{
    let mut largest = &list[0];

    for item in list{
        if item > largest {largest = item;}
    }

    &largest
}

fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd
{
    let mut largest = &list[0];

    for item in list{
        if item > largest {largest = item;}
    }

    &largest
}

fn main() {
    let number_list = vec![1, 2, 3, 4, 5];

    let result = largest(&number_list);
    println!("The largest number is {result}");
    
    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    
    let result = largest(&char_list);
    println!("The largest number is '{result}'");
    
}
