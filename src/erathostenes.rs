fn main(){
    let mut prims: Vec<i32> = vec![1,2];
    let mut i = 0;
    while i<100{
        for x in prims.iter(){
            prims.push(i);
            if i%x!=0{
                prims.pop();
                break;
            }
        }
    }
    println!("{:?}", prims);
}