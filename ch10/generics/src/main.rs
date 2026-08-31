struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    
}

enum Fake_Result<T, E> {
    Ok(T),
    Err(E),
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list{
        if number > largest{
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest{
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest{
            largest = item;
        }
    }

    largest
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest element is: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point {x: 5, y: 10};
    let float = Point { x: 1.0, y: 4.0};
    let mixed = Point {x: 1.0, y: 5.0};

    let p = Point { x: 5, y: 10};
    
    println!("p.x = {}", p.x());
}
