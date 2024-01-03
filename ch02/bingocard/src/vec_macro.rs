fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", nums);

    let mut nums = Vec::new();
    nums.push(1);
    nums.push(2);
    nums.push(3);
    println!("{:?}", nums);

    let a_vec: Vec<u32> = vec![100, 200, 300];
    for i in a_vec {
        println!("{}", i);
    }

    let s_vec: Vec<&str> = vec!["one", "two", "three"];
    for i in s_vec {
        println!("{}", i);
    }
}