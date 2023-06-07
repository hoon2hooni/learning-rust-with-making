struct BmiRange {
    min: f64,
    max: f64,
    label: &'static str,
}

fn main() {
    let bmi: f64 = 172.2 / 55.0;
    let bmi_list = vec![
        BmiRange {
            min: 0.0,
            max: 18.5,
            label: "저체중",
        },
        BmiRange {
            min: 18.5,
            max: 23.0,
            label: "저체중",
        },
        BmiRange {
            min: 23.0,
            max: 25.0,
            label: "저체중",
        },
    ];
    let mut result = "계산불가";
    for range in bmi_list {
        if range.min < bmi && bmi < range.max {}
    }
    println!("{}", result);
}
