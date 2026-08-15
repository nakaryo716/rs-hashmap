use rs_hashmap::HashMap;

fn main() {
    let mut map = HashMap::new(32);

    map.insert("rust", "cargo");
    map.insert("java", "maven");
    map.insert("python", "uv");

    assert_eq!(map.get(&"rust"), Some(&"cargo"));
    assert_eq!(map.get(&"java"), Some(&"maven"));
    assert_eq!(map.get(&"python"), Some(&"uv"));
}
