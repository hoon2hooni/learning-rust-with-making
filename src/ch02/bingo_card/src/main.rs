use rand::seq::SliceRandom;
fn main() {
    println!("Hello, world!");
    let mut nums = [0; 75];
    for i in 1..75 {
        nums[i - 1] = i;
    }

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                print!("  *,");
            } else {
                print!("{:3},", nums[i])
            }
        }
        println!("");
    }
    mut_vac();
}

fn mut_vac() {
    let nums = vec![1, 2, 3];
    println!("{:?}", nums);

    let mut without_macro_nums: Vec<i32> = Vec::new();
    without_macro_nums.push(1);
    without_macro_nums.push(2);
    without_macro_nums.push(-5);

    let mut without_macro_strs: Vec<&str> = Vec::new();
    without_macro_strs.push("닭");
    without_macro_strs.push("고양이");
    without_macro_strs.push("돼지");
}

fn bingo_card_with_vec() {
    let mut nums = vec![];
    for i in 1..75 {
        nums.push(i)
    }
    
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                print!("  *,");
            } else {
                print!("{:3},", nums[i])
            }
        }
        println!("");
    }
}
