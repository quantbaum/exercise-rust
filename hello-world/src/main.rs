mod lib;
use lib::{sublist, Comparison};

fn recurring_values_unequal() {
    let arr1 = [1, 2, 1, 2, 3];
    let arr2 = [1, 2, 3, 1, 2, 3, 2, 3, 2, 1];
    assert_eq!(
        Comparison::Unequal,
        sublist(&arr1, &arr2)
    );
}

fn equal(){
    let a = [1, 2];
    let b = [1, 2];
    println!("{}", a == b);
}
fn main(){
    // recurring_values_unequal()
    equal()
}