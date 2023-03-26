use std::io;
fn main() {
   println!("welcome to pascal triangle generator");
   loop {
    println!("insert number you want to get pascal triangle for");
    let mut value = String::new();

    io::stdin()
               .read_line(&mut value)
               .expect("could not process your input");
    let value: i32 = match value.trim().parse() {
        Ok(num)=> match num{
            x if x > 0 => x,
            _ => {
                println!("insert a positive number");
                continue
            },
        },
        Err(_)=>continue,
    };
    looper(value);
    break
}
}

fn looper(mut value: i32){
    let first = vec![1];
    let second = vec![1,1];
    match value{
     x if x == 1 =>  println!("{:?}", first),
     x if x == 2 =>  println!("{:?}", second),
     x if x > 2 =>  {
         println!("{:?}", first);
         println!("{:?}", second);
         let mut current_set = second;
         while value > 2{
             let result = processor(current_set);
             println!("{:?}", &result);
             current_set = result;
             value -=1;
         }  
     },
     _ => () 
    }
}

fn processor(array: Vec<i32>) -> Vec<i32>{
    let mut current_index = 0;
    let mut processing_set:Vec<i32> = vec![1];
    let mut length = array.len() as i32;
    while length -1 > 0{
        processing_set.push(&array[current_index] + &array[current_index + 1]);
        current_index += 1;
        length -=1;
      }
      processing_set.push(1);
      processing_set
}