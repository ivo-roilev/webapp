use std::sync::Arc;

#[allow(dead_code)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

pub fn dual_log(
    client: &Arc<reqwest::Client>,
    level: LogLevel,
    app: &str,
    user: Option<String>,
    message: String,
) {
    // 1. Local logging with function name prefix
    let formatted = format!("[{}] {}", app, message);
    match level {
        LogLevel::Debug => log::debug!("{}", formatted),
        LogLevel::Info => log::info!("{}", formatted),
        LogLevel::Warn => log::warn!("{}", formatted),
        LogLevel::Error => log::error!("{}", formatted),
    }

    // 2. Remote logging (fire and forget)
    if let Ok(logger_url) = std::env::var("LOGGER_URL") {
        let client = Arc::clone(client);
        let payload = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "level": level.as_str(),
            "message": message,
            "user": user,
            "app": app,
        });

        tokio::spawn(async move {
            let _ = client
                .post(format!("{}/logs", logger_url))
                .json(&payload)
                .send()
                .await;
            // Ignore result - fire and forget
        });
    }
}

// Macros for ergonomic usage
#[macro_export]
macro_rules! log_info {
    // Variant 1: Format string with args
    ($client:expr, $app:expr, $user:expr, $fmt:expr, $($arg:tt)+) => {{
        let msg = format!($fmt, $($arg)+);
        let user_opt = if $user.to_string().is_empty() { None } else { Some($user.to_string()) };
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Info,
            $app,
            user_opt,
            msg,
        )
    }};
    // Variant 2: Simple message
    ($client:expr, $app:expr, $user:expr, $msg:expr) => {
        let user_opt = if $user.to_string().is_empty() { None } else { Some($user.to_string()) };
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Info,
            $app,
            user_opt,
            $msg.to_string(),
        )
    };
}

#[macro_export]
macro_rules! log_error {
    // Variant 1: Format string with args
    ($client:expr, $app:expr, $user:expr, $fmt:expr, $($arg:tt)+) => {{
        let msg = format!($fmt, $($arg)+);
        let user_opt = if $user.to_string().is_empty() { None } else { Some($user.to_string()) };
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Error,
            $app,
            user_opt,
            msg,
        )
    }};
    // Variant 2: Simple message
    ($client:expr, $app:expr, $user:expr, $msg:expr) => {
        let user_opt = if $user.to_string().is_empty() { None } else { Some($user.to_string()) };
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Error,
            $app,
            user_opt,
            $msg.to_string(),
        )
    };
}

#[macro_export]
macro_rules! log_warn {
    // Variant 1: Format string with args
    ($client:expr, $app:expr, $user:expr, $fmt:expr, $($arg:tt)+) => {{
        let msg = format!($fmt, $($arg)+);
        let user_opt = if $user.to_string().is_empty() { None } else { Some($user.to_string()) };
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Warn,
            $app,
            user_opt,
            msg,
        )
    }};
    // Variant 2: Simple message
    ($client:expr, $app:expr, $user:expr, $msg:expr) => {
        let user_opt = if $user.to_string().is_empty() { None } else { Some($user.to_string()) };
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Warn,
            $app,
            user_opt,
            $msg.to_string(),
        )
    };
}

#[macro_export]
macro_rules! log_debug {
    // Variant 1: Format string with args
    ($client:expr, $app:expr, $user:expr, $fmt:expr, $($arg:tt)+) => {{
        let msg = format!($fmt, $($arg)+);
        let user_opt = if $user.to_string().is_empty() { None } else { Some($user.to_string()) };
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Debug,
            $app,
            user_opt,
            msg,
        )
    }};
    // Variant 2: Simple message
    ($client:expr, $app:expr, $user:expr, $msg:expr) => {
        let user_opt = if $user.to_string().is_empty() { None } else { Some($user.to_string()) };
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Debug,
            $app,
            user_opt,
            $msg.to_string(),
        )
    };
}
