use rust_learn;
use std::{env, error::Error, fs, process};

#[derive(Debug)]
struct ConfigParam {
    quary: String,
    path: String,
    ignore_case: bool,
}

impl ConfigParam {
    fn get_config_params(params: &[String]) -> Result<ConfigParam, &'static str> {
        if params.len() < 3 {
            return Err("缺少命令行参数！");
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(ConfigParam {
            quary: params[1].clone(),
            path: params[2].clone(),
            ignore_case,
        })
    }
}

fn read_file_form_path(config_param: &ConfigParam) -> Result<(), Box<dyn Error>> {
    let file_info = fs::read_to_string(&config_param.path)?;
    let results = if config_param.ignore_case {
        rust_learn::search_case_insensitive(&config_param.quary, &file_info)
    } else {
        rust_learn::search(&config_param.quary, &file_info)
    };
    for line in results {
        println!("search_vec_item = {}", line);
    }
    Ok(())
}

// 第一个例子
pub fn run_io_demo1() {
    let args: Vec<String> = env::args().collect();
    let config_param = ConfigParam::get_config_params(&args).unwrap_or_else(|err| {
        eprintln!("问题原因: {err}");
        process::exit(1);
    });
    if let Err(e) = read_file_form_path(&config_param) {
        eprintln!("打开文件出错: {e}");
        process::exit(1);
    }
}
