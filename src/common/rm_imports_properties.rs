use glob::glob;
use std::{
    env,
    fs::{self, File},
};

pub fn rm_imports_properties(id: &str) {
    let upload_dir = env::var("UPLOAD_DIR").expect("UPLOAD_DIR must be set");

    let pattern = format!("{}/{}+*", upload_dir, id);
    for enter in glob(&pattern).expect("Failed to read glob pattern") {
        if let Ok(path) = enter {
            if path.is_file() {
                if let Err(err) = fs::remove_file(path) {
                    log::error!("删除文件失败: {:?}", err);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_rm_imports_properties() {
        dotenv().ok();
        rm_imports_properties("3cac31f6-b99e-4ef9-96b5-cdbe5c729f49");
    }
}
