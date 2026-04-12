//Ridiculously basic hashmap implementation
//doesn't handle collisions, and can only store 10 values, cannot handle negative values



fn insert_hash(arr: &mut [i32], value: i32){

    let index = (value % 10) as usize;
    arr[index] = value;

} 

fn remove_hash(arr: &mut [i32], value: i32){
    let index = (value % 10) as usize;
    arr[index] = -1;
}

fn get_element(arr: &[i32], value: i32) -> i32 {
    let index = (value % 10) as usize;
    arr[index]
}

fn main() {
    let mut hm = [-1; 10];

    println!("{:?}", hm);

    insert_hash(&mut hm, 1);
    insert_hash(&mut hm, 7);

    println!("{:?}", hm);

    let test = get_element(&hm, 17);

    println!("{}", test);


}
