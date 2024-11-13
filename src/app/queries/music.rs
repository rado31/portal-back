pub const ALL_FOR_ADMIN: &str = r#"
    SELECT
        m.id,
        m.title,
        m.path
    FROM musics m
    ORDER BY m.id DESC
"#;
pub const ALL: &str = r#"
    SELECT
        m.id,
        m.title,
        m.path
    FROM musics m
    ORDER BY m.id
    OFFSET $1
    LIMIT $2
"#;
pub const TOTAL: &str = r#"
    SELECT COUNT(m.id)::INTEGER AS total
    FROM musics m
"#;
pub const ONE: &str = r#"
    SELECT
        m.id,
        m.title,
        m.path
    FROM musics m
    WHERE m.id = $1
"#;
pub const CREATE: &str = r#"
    INSERT INTO musics (title) VALUES ($1) RETURNING id
"#;
pub const UPDATE_MUSIC_PATH: &str = r#"
    UPDATE musics SET path = $1 WHERE id = $2
"#;
pub const UPDATE: &str = r#"
    UPDATE musics SET title = $1 WHERE id = $2
"#;
pub const DELETE: &str = r#"DELETE FROM musics WHERE id = $1"#;
