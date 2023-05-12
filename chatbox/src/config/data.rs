#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub working_dir: String,

    #[serde(skip)]
    pub config_path: String,

    #[serde(skip)]
    pub db_path: String,

    #[serde(skip)]
    pub cache_dir: String,

    #[serde(skip)]
    pub cache_audio_dir: String,

    pub ui: UI,

    pub socks5: Socks5,

    pub openai: OpenAi,

    pub audio: Audio,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UI {
    pub font_size: u32,
    pub win_width: u32,
    pub win_height: u32,
    pub language: String,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            font_size: 18,
            win_width: 1200,
            win_height: 800,
            language: "cn".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Socks5 {
    pub enabled: bool,
    pub url: String,
    pub port: u16,
}

impl Default for Socks5 {
    fn default() -> Self {
        Self {
            enabled: false,
            url: "127.0.0.1".to_string(),
            port: 1080,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAi {
    pub request_timeout: u64,
    pub stream_timeout: u64,
    pub api_key: String,
    pub chat: OpenAiChat,
}

impl Default for OpenAi {
    fn default() -> Self {
        Self {
            request_timeout: 30,
            stream_timeout: 15,
            api_key: String::default(),
            chat: OpenAiChat::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiChat {
    pub url: String,
    pub model: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

impl Default for OpenAiChat {
    fn default() -> Self {
        Self {
            url: "https://api.openai.com/v1/chat/completions".to_string(),
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 1024,
            temperature: 0.8,
            frequency_penalty: 0.5,
            presence_penalty: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Audio {
    pub region: String,
    pub api_key: String,
    pub current_input_device: String,
    pub is_auto_v2t: bool,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            region: String::default(),
            api_key: String::default(),
            current_input_device: "default".to_string(),
            is_auto_v2t: false,
        }
    }
}
