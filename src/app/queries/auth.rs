pub const GET_ADMIN: &str = r#"
    SELECT login, password
    FROM admins
    WHERE login = $1
"#;
