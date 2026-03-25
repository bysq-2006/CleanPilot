pub fn get_required_config(value: &str, key: &str) -> Result<String, String> {
    let value = value.trim().to_string();

    if value.is_empty() {
        return Err(format!("missing required config: {key}"));
    }

    Ok(value)
}

pub fn normalize_openai_api_base(base_url: &str) -> String {
    let trimmed = base_url.trim_end_matches('/');

    if trimmed.ends_with("/v1") {
        trimmed.to_string()
    }
    else {
        format!("{trimmed}/v1")
    }
}
