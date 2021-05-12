use std::collections::HashMap;
fn main(){
    let list_of_numbers: Vec<u32> = vec![1,1,1,1,4,4,4,4,4,4,4,4,4,4,4,2,3,4];
    let mean_value = get_mean_value(&list_of_numbers);
    let median_value: u32 = get_median_value(&list_of_numbers);
    //let modal_value: i32 = get_modal_value(&list_of_numbers);
    println!("vector => {:?}", list_of_numbers);
    println!("mean value {}", mean_value);
    println!("median value {}", median_value);
    println!("modal value ");
    get_modal_value(&list_of_numbers);
}
fn get_mean_value(vector: &Vec<u32>)-> f32{
    let mut sum: u32 = 0;
    for n in vector.iter(){
        sum += n;
    }
    sum as f32 / vector.len() as f32
}
fn is_even(len: &usize)->bool{
    if len % 2 == 0 {
        true
    }else{
        false
    }
}
fn get_median_value(vector: &Vec<u32>)-> u32{
    let mut inner_vector: Vec<u32> = vector.clone();
    inner_vector.sort();
    let len: usize = inner_vector.len();
    let mid_index: usize = (len / 2) as usize;
    if is_even(&len){
        (inner_vector[mid_index - 1] + inner_vector[mid_index]) / 2
    }else{
        inner_vector[mid_index]
    }
}
fn get_modal_value(vector: &Vec<u32>){
    let mut my_map = HashMap::new();
    for number in vector.iter(){
        let count = my_map.entry(number).or_insert(0);
        *count += 1;
    }
    let mut max_value: i32 = 0;
    for (_, value) in &my_map{
        if *value > max_value{
            max_value = *value;
        }
    }
    for (key, value) in my_map{
        println!("{} : {}", key, value);
    }
}