use algorithms::search::binary_search;
use algorithms::sort::quick_sort;
fn main() {
    let a = [10, 20, 90, 90, 100, 200];
    let res = binary_search(&a, &100);
    match res {
        Some(res) => {println!(" The target value found at index: {}", res)},
        None => {print!("Value not found")}
    }
    let mut a = [10, 90000, 90,70, 10, 200];
    quick_sort(&mut a);
    for el in a{
        println!("{}", el);
    }
}
