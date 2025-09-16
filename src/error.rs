use thiserror::Error;

// Re-export anyhow's Result but with our error type as default
pub type Result<T> = std::result::Result<T, RestlessError>;

#[derive(Error, Debug)]
pub enum RestlessError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL encoding error: {0}")]
    UrlEncoding(String),

    #[error("Invalid HTTP method: {method}")]
    InvalidHttpMethod { method: String },

    #[error("Invalid URL: {url}")]
    InvalidUrl { url: String },

    #[error("Request timeout")]
    Timeout,

    #[error("Invalid header format: {header}")]
    InvalidHeader { header: String },

    #[error("Invalid parameter format: {param}")]
    InvalidParameter { param: String },

    #[error("Tab error: {message}")]
    Tab { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("Response parsing error: {message}")]
    ResponseParsing { message: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Application state error: {message}")]
    AppState { message: String },
}

impl RestlessError {
    pub fn url_encoding<S: Into<String>>(msg: S) -> Self {
        Self::UrlEncoding(msg.into())
    }

    pub fn invalid_http_method<S: Into<String>>(method: S) -> Self {
        Self::InvalidHttpMethod {
            method: method.into(),
        }
    }

    pub fn invalid_url<S: Into<String>>(url: S) -> Self {
        Self::InvalidUrl { url: url.into() }
    }

    pub fn invalid_header<S: Into<String>>(header: S) -> Self {
        Self::InvalidHeader {
            header: header.into(),
        }
    }

    pub fn invalid_parameter<S: Into<String>>(param: S) -> Self {
        Self::InvalidParameter {
            param: param.into(),
        }
    }

    pub fn tab<S: Into<String>>(message: S) -> Self {
        Self::Tab {
            message: message.into(),
        }
    }

    pub fn terminal<S: Into<String>>(msg: S) -> Self {
        Self::Terminal(msg.into())
    }

    pub fn response_parsing<S: Into<String>>(message: S) -> Self {
        Self::ResponseParsing {
            message: message.into(),
        }
    }

    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    pub fn app_state<S: Into<String>>(message: S) -> Self {
        Self::AppState {
            message: message.into(),
        }
    }
}

// Conversion from anyhow::Error to RestlessError
impl From<anyhow::Error> for RestlessError {
    fn from(err: anyhow::Error) -> Self {
        RestlessError::AppState {
            message: format!("{}", err),
        }
    }
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Invalid URL format: {url}")]
    InvalidUrl { url: String },

    #[error("Request timeout after {seconds} seconds")]
    Timeout { seconds: u64 },

    #[error("Invalid request header: {key}={value}")]
    InvalidHeader { key: String, value: String },

    #[error("Request body serialization failed: {0}")]
    BodySerialization(String),

    #[error("Connection failed: {message}")]
    Connection { message: String },
}

impl RequestError {
    pub fn invalid_url<S: Into<String>>(url: S) -> Self {
        Self::InvalidUrl { url: url.into() }
    }

    pub fn timeout(seconds: u64) -> Self {
        Self::Timeout { seconds }
    }

    pub fn invalid_header<S: Into<String>>(key: S, value: S) -> Self {
        Self::InvalidHeader {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn body_serialization<S: Into<String>>(msg: S) -> Self {
        Self::BodySerialization(msg.into())
    }

    pub fn connection<S: Into<String>>(message: S) -> Self {
        Self::Connection {
            message: message.into(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("Failed to parse response body: {0}")]
    BodyParsing(String),

    #[error("Invalid response headers: {0}")]
    HeaderParsing(String),

    #[error("JSON formatting error: {0}")]
    JsonFormatting(#[from] serde_json::Error),

    #[error("Response body is empty")]
    EmptyBody,

    #[error("Unsupported content type: {content_type}")]
    UnsupportedContentType { content_type: String },
}

impl ResponseError {
    pub fn body_parsing<S: Into<String>>(msg: S) -> Self {
        Self::BodyParsing(msg.into())
    }

    pub fn header_parsing<S: Into<String>>(msg: S) -> Self {
        Self::HeaderParsing(msg.into())
    }

    pub fn unsupported_content_type<S: Into<String>>(content_type: S) -> Self {
        Self::UnsupportedContentType {
            content_type: content_type.into(),
        }
    }
}

#[derive(Error, Debug)]
pub enum UiError {
    #[error("Terminal initialization failed: {0}")]
    TerminalInit(String),

    #[error("Drawing error: {0}")]
    Drawing(String),

    #[error("Event handling error: {0}")]
    EventHandling(String),

    #[error("Invalid screen state: {state}")]
    InvalidScreenState { state: String },
}

impl UiError {
    pub fn terminal_init<S: Into<String>>(msg: S) -> Self {
        Self::TerminalInit(msg.into())
    }

    pub fn drawing<S: Into<String>>(msg: S) -> Self {
        Self::Drawing(msg.into())
    }

    pub fn event_handling<S: Into<String>>(msg: S) -> Self {
        Self::EventHandling(msg.into())
    }

    pub fn invalid_screen_state<S: Into<String>>(state: S) -> Self {
        Self::InvalidScreenState {
            state: state.into(),
        }
    }
}

// Conversion from other error types to RestlessError
impl From<RequestError> for RestlessError {
    fn from(err: RequestError) -> Self {
        match err {
            RequestError::Http(e) => RestlessError::Network(e),
            RequestError::InvalidUrl { url } => RestlessError::InvalidUrl { url },
            RequestError::Timeout { .. } => RestlessError::Timeout,
            RequestError::InvalidHeader { key, value } => {
                RestlessError::InvalidHeader {
                    header: format!("{}: {}", key, value),
                }
            }
            RequestError::BodySerialization(msg) => RestlessError::ResponseParsing { message: msg },
            RequestError::Connection { message } => RestlessError::ResponseParsing { message },
        }
    }
}

impl From<ResponseError> for RestlessError {
    fn from(err: ResponseError) -> Self {
        match err {
            ResponseError::BodyParsing(msg) | ResponseError::HeaderParsing(msg) => {
                RestlessError::ResponseParsing { message: msg }
            }
            ResponseError::JsonFormatting(e) => RestlessError::Json(e),
            ResponseError::EmptyBody => RestlessError::ResponseParsing {
                message: "Response body is empty".to_string(),
            },
            ResponseError::UnsupportedContentType { content_type } => {
                RestlessError::ResponseParsing {
                    message: format!("Unsupported content type: {}", content_type),
                }
            }
        }
    }
}

impl From<UiError> for RestlessError {
    fn from(err: UiError) -> Self {
        match err {
            UiError::TerminalInit(msg) | UiError::Drawing(msg) | UiError::EventHandling(msg) => {
                RestlessError::Terminal(msg)
            }
            UiError::InvalidScreenState { state } => RestlessError::AppState {
                message: format!("Invalid screen state: {}", state),
            },
        }
    }
}