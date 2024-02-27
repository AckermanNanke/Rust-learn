// 测试库
#[cfg(test)]
mod Test {
    use super::*;
    #[test]
    fn get_param() {
        let query = "燃尽";
        let contents = "\
        纵使黑云蔽日，我也要燃尽天空，带你找到回家的路！
        纵使长夜漫漫永劫无期，
        我也要化身星辰点燃希望，
        你的誓言由我来实现!";

        assert_eq!(
            vec!["纵使黑云蔽日，我也要燃尽天空，带你找到回家的路！"],
            search(query, contents)
        );
    }
    fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        for item in contents.lines() {
            if item.contains(query) {
                results.push(item)
            }
        }
        results
    }
    #[test]
    fn case_insensitive() {
        let q = "ruST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(q, contents)
        );
    }
    fn search_case_insensitive<'a>(q: &'a str, c: &'a str) -> Vec<&'a str> {
        let q = q.to_uppercase();
        let mut results = Vec::new();
        for item in c.lines() {
            if item.to_uppercase().contains(&q) {
                results.push(item)
            }
        }
        results
    }
}
