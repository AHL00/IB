use ib::inputln;

fn main() {
    let n: i32 = inputln("Enter n: ");

    // This is a rust feature called "list comprehension"
    // It is equivalent to the following:
        // let mut arr: Vec<i32> = Vec::new();
        // for i in 0..n {
        //     arr.push(fibonacci(i));
        // }
    // The collect() method is used to store the values in a vector

    // 0..n returns what is called a range
    // map() is a method that takes a closure (a function that takes no arguments and returns a value)
    // and applies it to every element in the range, then returns what is called an iterator
    // collect() is a method that takes an iterator and "runs" it, and returns the values to an iterator

    // This is useful for more complex operations in very few lines of code
    // FYI: This is not only a rust feature, also exists in python and some other

    let arr: Vec<i32> = (0..n).map(|x| fibonacci(x)).collect();    

    println!("Fibonacci sequence: {:?}", arr);
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}