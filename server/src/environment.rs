pub struct EnvironmentVariables {
    mongo_uri: String,
    mongo_database: String,
}

impl EnvironmentVariables {
    fn get_env_var_or_default(key: &str, default: &str) -> String {
        match std::env::var(key) {
            Ok(value) => value,
            _ => String::from(default),
        }
    }

    pub fn get_mongo_uri(&self) -> &str {
        &self.mongo_uri
    }

    pub fn get_mongo_database(&self) -> &str {
        &self.mongo_database
    }

    pub fn new() -> Self {
        Self {
            mongo_uri: Self::get_env_var_or_default("MONGO_URI", "mongodb://localhost:27017/"),
            mongo_database: Self::get_env_var_or_default("MONGO_DATABASE", "fdr"),
        }
    }
}