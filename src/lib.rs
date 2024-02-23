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
}
