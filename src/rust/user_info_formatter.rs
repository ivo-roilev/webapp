msguse crate::db::User;

/// Constructs a display name from user fields with prioritized fallback logic
pub fn construct_name(
    first_name: Option<String>,
    last_name: Option<String>,
    username: String,
) -> String {
    match (first_name, last_name) {
        (Some(first), Some(last)) => format!("{} {}", first, last),
        (Some(first), None) => first,
        (None, Some(last)) => last,
        (None, None) => username,
    }
}

/// Formats a user greeting message in the format:
/// "Hello [Title] [Name], welcome! If we hear interesting news about [Hobby], we will let you know at [Email]!"
pub fn format_user_greeting(user: User) -> String {
    // Construct the user's display name
    let name = construct_name(user.first_name, user.last_name, user.username);

    // Build greeting section: "Hello [Title] Name, welcome!"
    let greeting = match user.title {
        Some(title) => format!("Hello {} {}, welcome!", title, name),
        None => format!("Hello {}, welcome!", name),
    };

    // Build notification section if hobby is present
    let notification = match user.hobby {
        Some(hobby) => {
            let mut msg = format!(" If we hear interesting news about {}, we will let you know", hobby);
            if let Some(email) = user.email {
                msg.push_str(&format!(" at {}", email));
            }
            msg.push('!');
            msg
        }
        None => String::new(), // No notification if no hobby
    };

    format!("{}{}", greeting, notification)
}
