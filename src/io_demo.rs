use std::{env, error::Error, fs, process};

#[derive(Debug)]
struct ConfigParam {
    quary: String,
    path: String,
}

impl ConfigParam {
    fn get_config_params(params: &[String]) -> Result<ConfigParam, &'static str> {
        if params.len() < 3 {
            return Err("缺少命令行参数！");
        }
        Ok(ConfigParam {
            quary: params[1].clone(),
            path: params[2].clone(),
        })
    }
}

fn read_file_form_path(config_param: &ConfigParam) -> Result<(), Box<dyn Error>> {
    let file_info = fs::read_to_string(&config_param.path)?;
    for line in search(&config_param.quary, &file_info) {
        println!("search_vec_item = {}", line);
    }
    Ok(())
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
// 第一个例子
pub fn run_io_demo1() {
    let args: Vec<String> = env::args().collect();
    let config_param = ConfigParam::get_config_params(&args).unwrap_or_else(|err| {
        println!("问题原因: {err}");
        process::exit(1);
    });
    dbg!(&args[1]);
    if let Err(e) = read_file_form_path(&config_param) {
        println!("打开文件出错: {e}");
        process::exit(1);
    }
}
