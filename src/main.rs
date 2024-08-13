fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let filters: Vec<i32> = nums.into_iter().filter(|x| x % 2 ==0).collect();
    println!("{:?}", filters);
}