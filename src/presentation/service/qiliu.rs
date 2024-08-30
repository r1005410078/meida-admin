use qiniu_credential::Uri;
use qiniu_sdk::download::apis::credential::Credential;
use qiniu_sdk::upload_token::UploadPolicy;
use std::time::Duration;
use std::u64;

static ACCESS_KEY: &str = "SiKapEAp33fGNhZqG1-SAe1TOdd-gzfHxGtk93Au";
static SECRET_KEY: &str = "6m6LQh8iGGivTO3s5PClYHfOJAxZAjLHLvQE6mjf";
static BUCKET_NAME: &str = "meida-family";

pub fn get_upload_token(object_name: &str) -> String {
    let credential = Credential::new(ACCESS_KEY, SECRET_KEY);
    let upload_token =
        UploadPolicy::new_for_object(BUCKET_NAME, object_name, Duration::from_secs(60 * 5))
            .build_token(credential, Default::default());

    upload_token.to_string()
}

pub fn get_upload_url(object_name: &str) -> anyhow::Result<Uri> {
    let domain = "img.rongts.tech";
    let credential: Credential = Credential::new(ACCESS_KEY, SECRET_KEY);
    let mut path = "/".to_string();
    url_escape::encode_path_to_string(object_name, &mut path);

    let url = Uri::builder()
        .scheme("http")
        .authority(domain)
        .path_and_query(path)
        .build()?;

    let url = credential.sign_download_url(url, Duration::from_micros(u64::MAX));

    Ok(url)
}

#[test]
fn test_get_upload_token() {
    let token = get_upload_token("test.jpg");
    println!("\n{}\n", token);
}

#[test]
fn test_get_upload_token_provider() {
    let url = get_upload_url("5abcff46-c590-4357-a78f-1ecda164b95a_107.jpg").unwrap();
    println!("{}", url);
}
