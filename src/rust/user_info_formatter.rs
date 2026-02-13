use crate::db::User;

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
pub fn format_user_greeting(user: &User) -> String {
    let (first_name, last_name, email) = user.profile.as_ref().map(|p| (p.first_name.clone(), p.last_name.clone(), p.email.clone())).unwrap_or((None, None, None));

    let mut title = None;
    let mut hobby = None;

    for m in &user.metadata {
        if m.property == "title" {
            title = m.value.clone();
        } else if m.property == "hobby" {
            hobby = m.value.clone();
        }
    }

    // Construct the user's display name
    let name = construct_name(first_name, last_name, user.username.clone());

    // Build greeting section: "Hello [Title] Name, welcome!"
    let greeting = match title {
        Some(t) => format!("Hello {} {}, welcome!", t, name),
        None => format!("Hello {}, welcome!", name),
    };

    // Build notification section if hobby is present
    let notification = match hobby {
        Some(h) => {
            let mut msg = format!(" If we hear interesting news about {}, we will let you know", h);
            if let Some(e) = email {
                msg.push_str(&format!(" at {}", e));
            }
            msg.push('!');
            msg
        }
        None => String::new(), // No notification if no hobby
    };

    format!("{}{}", greeting, notification)
}
