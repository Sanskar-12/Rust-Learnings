// if we are passing vector of f64 type then we have to change the type of the parameter so we can use generic where we can pass any type inside the function

// ---------------------- Generic Type -----------------------------//

// fn largest_i32<T: std::cmp::PartialOrd>(list:&[T]) -> &T {
//     let mut largest= &list[0];

//     for i in list {
//         if i > largest {
//             largest = i;
//         }
//     }

//     return largest;
// }

// fn main() {
//     let list_1=vec![1,2,3,4,5];
//     let list_2=vec![1.1,2.1,3.1,4.1,5.1];
//     let res= largest_i32(&list_2);
//     println!("The largest number is {res}");
// }

// -------------------------Generic with struct---------------//

// we can also take multiple generics
struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U> Point<T, U> {
    fn new(x:T, y:U) -> Self {
        Self {x,y}
    }
}

fn main() {
    let point1 = Point::new(2, 3);

    let point2 = Point { x: 23.3, y: 34 };
}
