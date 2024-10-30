pub const GET_CATEGORIES: &str = r#"
    SELECT id, title
    FROM categories
    ORDER BY id DESC
"#;
pub const GET_CATEGORY: &str = r#"
    SELECT id, title
    FROM categories
    WHERE id = $1
"#;
pub const CREATE_CATEGORY: &str = r#"
    INSERT INTO categories (title) VALUES ($1) RETURNING id
"#;
pub const GET_SUB_CATEGORIES: &str = r#"
    SELECT id, title
    FROM sub_categories
    WHERE category_id = $1
    ORDER BY id DESC
"#;
pub const GET_SUB_CATEGORY: &str = r#"
    SELECT id, title
    FROM sub_categories
    WHERE id = $1
"#;
pub const CREATE_SUB_CATEGORY: &str = r#"
    INSERT INTO sub_categories (title, category_id) VALUES ($1, $2) RETURNING id
"#;
