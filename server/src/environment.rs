pub struct EnvironmentVariables {
    server_mode: ServerMode,
    meilisearch_host: String,
    meilisearch_api_key: String
}

impl EnvironmentVariables {
    fn get_env_var_or_default(key: &str, default: &str) -> String {
        match std::env::var(key) {
            Ok(value) => value,
            _ => String::from(default),
        }
    }

    fn parse_server_mode_or_panic(raw_server_mode: String) -> ServerMode {
        if raw_server_mode == RAW_PROD_SERVER_MODE {
            ServerMode::Prod
        } else if raw_server_mode == RAW_MOCK_SERVER_MODE {
            ServerMode::Mock
        } else {
            panic!("SERVER_MODE environment variable must be 'prod' or 'mock'!");
        }
    }

    pub fn get_server_mode(&self) -> ServerMode {
        self.server_mode
    }

    pub fn get_meilisearch_host(&self) -> &str {
        &self.meilisearch_host
    }

    pub fn get_meilisearch_api_key(&self) -> &str {
        &self.meilisearch_api_key
    }
}

impl Default for EnvironmentVariables {
    fn default() -> Self {
        Self {
            server_mode: Self::parse_server_mode_or_panic(Self::get_env_var_or_default(
                "SERVER_MODE",
                RAW_PROD_SERVER_MODE,
            )),
            meilisearch_host: Self::get_env_var_or_default("MEILISEARCH_HOST", "http://localhost:7700"),
            meilisearch_api_key: Self::get_env_var_or_default("MEILISEARCH_API_KEY", "masterKey")
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ServerMode {
    Prod, // Connects to backend services and loads real data from them.
    Mock, // Doesn't connect to any backend services - uses mock data and is able to run completely standalone. Good for testing and development.
}

// Raw values acceptable for SERVER_MODE environment variable.
const RAW_PROD_SERVER_MODE: &str = "prod";
const RAW_MOCK_SERVER_MODE: &str = "mock";
