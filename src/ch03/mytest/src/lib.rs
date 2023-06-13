pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(100 * 2, 200);
    }

    #[test]
    fn calc_test2() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(100, 200), 300);
    }

    #[test]
    fn array_test() {
        let a1 = [1, 2, 3];
        let a2 = [1, 2, 3];
        assert_eq!(a1, a2);

        let a3 = ["사과".to_string(), "배".to_string()];
        let a4 = [String::from("사과"), String::from("배")];
        assert_eq!(a3, a4);
    }

    #[test]
    fn vec_test() {
        let v1 = vec!["apple", "banna", "mango"];
        let mut v2: Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banna");
        v2.push("mango");
        assert_eq!(v1, v2);
    }

    #[test]
    fn struct_test() {
        let user1 = User {
            name: String::from("홍길동"),
            age: 20,
        };
        let user2 = User {
            name: String::from("홍길동"),
            age: 20,
        };
        assert_eq!(user1.age, user2.age);
        assert_eq!(user1, user2);
    }
}
