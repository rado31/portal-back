pub const GET_CATEGORY: &str = r#"
    SELECT id, title
    FROM categories
    WHERE id = $1
"#;
pub const ALL: &str = r#"
    SELECT id, title
    FROM sub_categories
    WHERE category_id = $1
    ORDER BY id DESC
"#;
pub const ONE: &str = r#"
    SELECT id, title
    FROM sub_categories
    WHERE id = $1
"#;
pub const CREATE: &str = r#"
    INSERT INTO sub_categories (title, category_id) VALUES ($1, $2) RETURNING id
"#;
pub const UPDATE: &str = r#"
    UPDATE sub_categories SET title = $1 WHERE id = $2
"#;
pub const DELETE: &str = r#"
    DELETE FROM sub_categories WHERE id = $1
"#;
