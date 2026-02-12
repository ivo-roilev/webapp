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
    message: String,
    user: Option<String>,
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
    // Variant 1: Format string with single arg + user field (MUST COME FIRST for precedence)
    ($client:expr, $app:expr, $fmt:literal, $arg:tt, user = $user:expr) => {{
        let msg = format!($fmt, $arg);
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Info,
            $app,
            msg,
            Some($user.to_string()),
        )
    }};
    // Variant 2: Simple message with user field
    ($client:expr, $app:expr, $msg:expr, user = $user:expr) => {
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Info,
            $app,
            $msg.to_string(),
            Some($user.to_string()),
        )
    };
    // Variant 3: Format string with args, no user field
    ($client:expr, $app:expr, $fmt:expr, $($arg:tt)+) => {{
        let msg = format!($fmt, $($arg)+);
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Info,
            $app,
            msg,
            None,
        )
    }};
    // Variant 4: Simple message without user field (MUST COME LAST)
    ($client:expr, $app:expr, $msg:expr) => {
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Info,
            $app,
            $msg.to_string(),
            None,
        )
    };
}

#[macro_export]
macro_rules! log_error {
    // Variant 1: Format string with single arg + user field (MUST COME FIRST for precedence)
    ($client:expr, $app:expr, $fmt:literal, $arg:tt, user = $user:expr) => {{
        let msg = format!($fmt, $arg);
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Error,
            $app,
            msg,
            Some($user.to_string()),
        )
    }};
    // Variant 2: Simple message with user field
    ($client:expr, $app:expr, $msg:expr, user = $user:expr) => {
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Error,
            $app,
            $msg.to_string(),
            Some($user.to_string()),
        )
    };
    // Variant 3: Format string with args, no user field
    ($client:expr, $app:expr, $fmt:expr, $($arg:tt)+) => {{
        let msg = format!($fmt, $($arg)+);
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Error,
            $app,
            msg,
            None,
        )
    }};
    // Variant 4: Simple message without user field (MUST COME LAST)
    ($client:expr, $app:expr, $msg:expr) => {
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Error,
            $app,
            $msg.to_string(),
            None,
        )
    };
}

#[macro_export]
macro_rules! log_warn {
    // Variant 1: Format string with args + user field
    ($client:expr, $app:expr, $fmt:expr, $($arg:expr),+, user = $user:expr) => {{
        let msg = format!($fmt, $($arg),+);
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Warn,
            $app,
            msg,
            Some($user.to_string()),
        )
    }};
    // Variant 2: Simple message with user field
    ($client:expr, $app:expr, $msg:expr, user = $user:expr) => {
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Warn,
            $app,
            $msg.to_string(),
            Some($user.to_string()),
        )
    };
    // Variant 3: Simple message without user field
    ($client:expr, $app:expr, $msg:expr) => {
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Warn,
            $app,
            $msg.to_string(),
            None,
        )
    };
    // Variant 4: Format string with args, no user field
    ($client:expr, $app:expr, $fmt:expr, $($arg:tt)+) => {{
        let msg = format!($fmt, $($arg)+);
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Warn,
            $app,
            msg,
            None,
        )
    }};
}

#[macro_export]
macro_rules! log_debug {
    // Variant 1: Format string with args + user field
    ($client:expr, $app:expr, $fmt:expr, $($arg:expr),+, user = $user:expr) => {{
        let msg = format!($fmt, $($arg),+);
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Debug,
            $app,
            msg,
            Some($user.to_string()),
        )
    }};
    // Variant 2: Simple message with user field
    ($client:expr, $app:expr, $msg:expr, user = $user:expr) => {
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Debug,
            $app,
            $msg.to_string(),
            Some($user.to_string()),
        )
    };
    // Variant 3: Simple message without user field
    ($client:expr, $app:expr, $msg:expr) => {
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Debug,
            $app,
            $msg.to_string(),
            None,
        )
    };
    // Variant 4: Format string with args, no user field
    ($client:expr, $app:expr, $fmt:expr, $($arg:tt)+) => {{
        let msg = format!($fmt, $($arg)+);
        $crate::logger::dual_log(
            $client,
            $crate::logger::LogLevel::Debug,
            $app,
            msg,
            None,
        )
    }};
}
