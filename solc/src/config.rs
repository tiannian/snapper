#[derive(Debug, Default)]
pub struct Config {
    pub upstream: Option<String>,
    pub out_dir: Option<String>,
    pub snapper_file: Option<String>,
}
