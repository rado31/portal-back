pub const GET_ME: &str = r#"
    SELECT username, password
    FROM admins
    WHERE username = $1
"#;
