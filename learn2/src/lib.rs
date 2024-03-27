pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 使用迭代器版本
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    // let mut results = Vec::new();
    // for item in contents.lines() {
    //     if item.contains(query) {
    //         results.push(item)
    //     }
    // }
    // results
}

pub fn search_case_insensitive<'a>(q: &'a str, c: &'a str) -> Vec<&'a str> {
    let q = q.to_uppercase();
    let mut results = Vec::new();
    for item in c.lines() {
        if item.to_uppercase().contains(&q) {
            results.push(item)
        }
    }
    results
}
