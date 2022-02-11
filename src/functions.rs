pub fn run(){
    greeting("Hello", "Jane");
    let get_sum = add(5,5);
    println!("{get_sum}")
}

fn greeting(greet: &str, name: &str){
    println!("{}, {}!\nNice to meet you!", greet, name)
}

fn add(n1:i32, n2: i32) -> i32 { //returns i32 value
    n1 + n2
}