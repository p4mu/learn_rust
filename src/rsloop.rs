pub fn run(){
    // infinite loop
    let mut count = 0;
    loop{
        count+=1;
        println!("{}", count);
        if count==13{
            break;
        }
    }
    // while loop
    count = 0;
    while count <= 100{
        println!{"{}", count}
        count+=1;
    }
}