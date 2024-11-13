pub const ALL_FOR_ADMIN: &str = r#"
    SELECT
        m.id,
        m.title,
        m.description,
        m.duration,
        m.image,
        m.is_uploaded,
        ARRAY(
            SELECT
                JSON_BUILD_OBJECT(
                    'id', msc.sub_category_id,
                    'title', (
                        SELECT sc.title
                        FROM sub_categories sc
                        WHERE sc.id = msc.sub_category_id
                    )
                )::VARCHAR
            FROM movies_sub_categories msc
            WHERE msc.movie_id = m.id
        ) AS sub_categories
    FROM movies m
    ORDER BY m.id desc
"#;
pub const ALL: &str = r#"
    SELECT
        m.id,
        m.title,
        m.description,
        m.duration,
        m.image,
        m.is_uploaded,
        ARRAY(
            SELECT
                JSON_BUILD_OBJECT(
                    'id', msc.sub_category_id,
                    'title', (
                        SELECT sc.title
                        FROM sub_categories sc
                        WHERE sc.id = msc.sub_category_id
                    )
                )::VARCHAR
            FROM movies_sub_categories msc
            WHERE msc.movie_id = m.id
        ) AS sub_categories
    FROM movies m
    ORDER BY m.id desc
    OFFSET $1
    LIMIT $2
"#;
pub const TOTAL: &str = r#"
    SELECT COUNT(m.id)::INTEGER AS total
    FROM movies m
"#;
pub const ONE: &str = r#"
    SELECT
        m.id,
        m.title,
        m.description,
        m.duration,
        m.image,
        m.is_uploaded,
        ARRAY(
            SELECT
                JSON_BUILD_OBJECT(
                    'id', msc.sub_category_id,
                    'title', (
                        SELECT sc.title
                        FROM sub_categories sc
                        WHERE sc.id = msc.sub_category_id
                    )
                )::VARCHAR
            FROM movies_sub_categories msc
            WHERE msc.movie_id = m.id
        ) AS sub_categories
    FROM movies m
    WHERE m.id = $1
"#;
pub const ALL_BY_SC_TOTAL: &str = r#"
    SELECT COUNT(msc.movie_id)::INTEGER AS total
    FROM movies_sub_categories msc
    INNER JOIN movies m ON m.id = msc.movie_id
    WHERE msc.sub_category_id = $1
"#;
pub const ALL_BY_SC: &str = r#"
    SELECT
        m.id,
        m.title,
        m.description,
        m.duration,
        m.image,
        m.is_uploaded,
        ARRAY(
            SELECT
                JSON_BUILD_OBJECT(
                    'id', msc.sub_category_id,
                    'title', (
                        SELECT sc.title
                        FROM sub_categories sc
                        WHERE sc.id = msc.sub_category_id
                    )
                )::VARCHAR
            FROM movies_sub_categories msc
            WHERE msc.movie_id = m.id
        ) AS sub_categories
    FROM movies_sub_categories msc
    INNER JOIN movies m ON m.id = msc.movie_id
    WHERE msc.sub_category_id = $1
    OFFSET $2
    LIMIT $3
"#;
pub const SEARCH: &str = r#"
    SELECT
        m.id,
        m.title,
        m.description,
        m.duration,
        m.image,
        m.is_uploaded,
        ARRAY(
            SELECT
                JSON_BUILD_OBJECT(
                    'id', msc.sub_category_id,
                    'title', (
                        SELECT sc.title
                        FROM sub_categories sc
                        WHERE sc.id = msc.sub_category_id
                    )
                )::VARCHAR
            FROM movies_sub_categories msc
            WHERE msc.movie_id = m.id
        ) AS sub_categories
    FROM movies m
    WHERE EXISTS (
		SELECT *
		FROM JSONB_EACH_TEXT(m.title) titles
		WHERE titles.value ILIKE CONCAT('%', $1::VARCHAR, '%')
		LIMIT 10
	)  
"#;
pub const CREATE: &str = r#"
    INSERT INTO movies (title, description, duration)
    VALUES ($1, $2, $3) RETURNING id
"#;
pub const CREATE_SC: &str = r#"
    INSERT INTO movies_sub_categories (movie_id, sub_category_id)
    VALUES ($1, $2)
"#;
pub const DELETE_SC: &str = r#"
    DELETE FROM movies_sub_categories WHERE movie_id = $1
"#;
pub const UPDATE_IMAGE_PATH: &str = r#"
    UPDATE movies SET image = $1 WHERE id = $2
"#;
pub const UPDATE: &str = r#"
    UPDATE movies SET title = $1, description = $2, duration = $3, status = $4
    WHERE id = $5
"#;
pub const UPDATE_UPLOADED_STATUS: &str = r#"
    UPDATE movies SET is_uploaded = true WHERE id = $1
"#;
pub const DELETE: &str = r#"
    DELETE FROM movies WHERE id = $1 RETURNING id
"#;
