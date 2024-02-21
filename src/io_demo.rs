use std::{env, fs};

#[derive(Debug)]
struct ConfigParam {
    quary: String,
    path: String,
}

pub fn run_io_demo1() {
    let args: Vec<String> = env::args().collect();
    let a = ConfigParam::get_config_params(&args)?;
    // let contents = fs::read_to_string(a);
    dbg!(args);
    dbg!(a);
}

impl ConfigParam {
    fn get_config_params(params: &[String]) -> Result<ConfigParam, &'static str> {
        if params.len() < 3 {
            return Err("缺少命令行参数！");
        }
        Ok(ConfigParam {
            quary: params[0].clone(),
            path: params[0].clone(),
        })
    }
}
