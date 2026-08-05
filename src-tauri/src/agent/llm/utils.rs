pub fn get_required_config(value: &str, key: &str) -> Result<String, String> {
    let value = value.trim().to_string();

    if value.is_empty() {
        return Err(format!("missing required config: {key}"));
    }

    Ok(value)
}

pub fn normalize_openai_api_base(base_url: &str) -> String {
    base_url.trim_end_matches('/').to_string()
}
