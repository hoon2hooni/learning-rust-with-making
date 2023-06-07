fn main() {
    //배열 길이 변경 불가능
    let points = [80, 50, 40];
    slice();
}

fn slice() {
    //slice는 참조자들이다.
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[0..3];
    println!("{:?}", a_slice[1]);
    let num = sum_slice(a_slice);
    println!("{}",num);
}

fn sum_slice(nums: &[i64]) -> i64 {
    let mut total: i64 = 0;
    for num in nums {
        total += num;
    }
    return total;
}
