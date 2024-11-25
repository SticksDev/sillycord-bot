use std::process::Command;

pub async fn get_rustc_version() -> String {
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("failed to get rustc version");

    String::from_utf8(rustc_version.stdout).expect("failed to convert rustc version to string")
}

pub async fn get_random_action_image(action: String) -> Result<String, reqwest::Error> {
    let action = action.to_lowercase();
    let action = action.replace(" ", "_");

    let url = format!("https://api.otakugifs.xyz/gif?reaction={}", action);
    let response = reqwest::get(&url)
        .await
        .expect("failed to get random action image");

    let response_json: serde_json::Value =
        serde_json::from_str(&response.text().await.expect("failed to get response text"))
            .expect("failed to parse response json");

    let image_url = response_json["url"]
        .as_str()
        .expect("failed to get image url from response json");

    Ok(image_url.to_string())
}
