use crate::db::{User, UserProfile, UserMetadata};
use crate::user_info_formatter::{construct_name, format_user_greeting};

#[test]
fn test_construct_name_full_name() {
    let result = construct_name(
        Some("John".to_string()),
        Some("Doe".to_string()),
        "jdoe".to_string(),
    );
    assert_eq!(result, "John Doe");
}

#[test]
fn test_construct_name_first_only() {
    let result = construct_name(
        Some("John".to_string()),
        None,
        "jdoe".to_string(),
    );
    assert_eq!(result, "John");
}

#[test]
fn test_construct_name_last_only() {
    let result = construct_name(
        None,
        Some("Doe".to_string()),
        "jdoe".to_string(),
    );
    assert_eq!(result, "Doe");
}

#[test]
fn test_construct_name_username_fallback() {
    let result = construct_name(None, None, "jdoe".to_string());
    assert_eq!(result, "jdoe");
}

#[test]
fn test_format_user_greeting_all_fields() {
    let user = User {
        id: 1,
        username: "jdoe".to_string(),
        password: "pass".to_string(),
        profile: Some(UserProfile {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: Some("john@email.com".to_string()),
        }),
        metadata: vec![
            UserMetadata {
                parent_property: None,
                property: "title".to_string(),
                value: Some("Software Engineer".to_string()),
            },
            UserMetadata {
                parent_property: None,
                property: "hobby".to_string(),
                value: Some("hiking".to_string()),
            }
        ],
        created_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
    };
    let result = format_user_greeting(&user);
    assert_eq!(
        result,
        "Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"
    );
}

#[test]
fn test_format_user_greeting_minimal() {
    let user = User {
        id: 1,
        username: "jdoe".to_string(),
        password: "pass".to_string(),
        profile: None,
        metadata: vec![],
        created_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
    };
    let result = format_user_greeting(&user);
    assert_eq!(result, "Hello jdoe, welcome!");
}

#[test]
fn test_format_user_greeting_no_hobby() {
    let user = User {
        id: 1,
        username: "jdoe".to_string(),
        password: "pass".to_string(),
        profile: Some(UserProfile {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: Some("john@email.com".to_string()),
        }),
        metadata: vec![
            UserMetadata {
                parent_property: None,
                property: "title".to_string(),
                value: Some("Software Engineer".to_string()),
            }
        ],
        created_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
    };
    let result = format_user_greeting(&user);
    assert_eq!(result, "Hello Software Engineer John Doe, welcome!");
}

#[test]
fn test_format_user_greeting_no_title() {
    let user = User {
        id: 1,
        username: "jdoe".to_string(),
        password: "pass".to_string(),
        profile: Some(UserProfile {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: Some("john@email.com".to_string()),
        }),
        metadata: vec![
            UserMetadata {
                parent_property: None,
                property: "hobby".to_string(),
                value: Some("hiking".to_string()),
            }
        ],
        created_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
    };
    let result = format_user_greeting(&user);
    assert_eq!(
        result,
        "Hello John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"
    );
}

#[test]
fn test_format_user_greeting_hobby_no_email() {
    let user = User {
        id: 1,
        username: "jdoe".to_string(),
        password: "pass".to_string(),
        profile: Some(UserProfile {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: None,
        }),
        metadata: vec![
            UserMetadata {
                parent_property: None,
                property: "title".to_string(),
                value: Some("Software Engineer".to_string()),
            },
            UserMetadata {
                parent_property: None,
                property: "hobby".to_string(),
                value: Some("hiking".to_string()),
            }
        ],
        created_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
    };
    let result = format_user_greeting(&user);
    assert_eq!(
        result,
        "Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know!"
    );
}
