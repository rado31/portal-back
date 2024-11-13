pub const ALL_FOR_ADMIN: &str = r#"
    SELECT
        b.id,
        b.title,
        b.path
    FROM books b
    ORDER BY b.id
"#;
pub const ALL: &str = r#"
    SELECT
        b.id,
        b.title,
        b.path
    FROM books b
    ORDER BY b.id
    OFFSET $1
    LIMIT $2
"#;
pub const TOTAL: &str = r#"
    SELECT COUNT(b.id)::INTEGER AS total
    FROM books b
"#;
pub const ONE: &str = r#"
    SELECT
        b.id,
        b.title,
        b.path
    FROM books b
    WHERE b.id = $1
"#;
pub const CREATE: &str = r#"
    INSERT INTO books (title) VALUES ($1) RETURNING id
"#;
pub const UPDATE_BOOK_PATH: &str = r#"
    UPDATE books SET path = $1 WHERE id = $2
"#;
pub const UPDATE: &str = r#"
    UPDATE books SET title = $1 WHERE id = $2
"#;
pub const DELETE: &str = r#"DELETE FROM books WHERE id = $1"#;
