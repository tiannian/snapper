/// Profile
#[derive(Debug, Clone)]
pub enum ProfileType {
    Debug,
    Release,
}

impl ProfileType {
    pub fn from_str(ty: &str) -> Option<Self> {
        match ty {
            "release" => Some(Self::Release),
            "debug" => Some(Self::Debug),
            _ => None,
        }
    }
}
