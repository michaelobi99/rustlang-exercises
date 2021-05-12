fn main(){
    let mut list_of_numbers: Vec<u32> = vec![4,4,4,4,5,2,3,3,3,5,6,6,6,5,5,5,1,2,6,6,6,7];
    let mean_value = get_mean_value(&list_of_numbers);
    let median_value: u32 = get_median_value(&list_of_numbers);
}
fn get_mean_value(vector: &Vec<u32>)-> u32{
    let mut sum: u32 = 0;
    for n in vector.iter(){
        sum += n;
    }
    sum / vector.len()
}
fn get_median_value(vector: &Vec<u32>)-> u32{
    let mut inner_vector: Vec<u32> = vector.clone();
    inner_vector.sort();
    let
}