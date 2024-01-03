
#[cfg(test)]
mod tests {
    #[test]
    fn array_test() {
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2);

        let a3 = ["リンゴ".to_string(), "スイカ".to_string(), "メロン".to_string()];
        let a4 = [String::from("リンゴ"), String::from("スイカ"), String::from("メロン")];
        assert_eq!(a3, a4);
    }

    #[test]
    fn vec_test() {
        let v1 = vec!["apple", "banana", "orange"];
        let mut v2:Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("orange");
        assert_eq!(v1, v2);
    }
}