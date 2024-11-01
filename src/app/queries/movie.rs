pub const GET_MOVIES_FOR_ADMIN: &str = r#"
    SELECT
        m.id,
        m.title,
        m.description,
        m.duration,
        m.image,
        m.status,
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
pub const GET_MOVIES: &str = r#"
    SELECT
        m.id,
        m.title,
        m.description,
        m.duration,
        m.image,
        m.status,
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
            FROM movie_sub_categories msc
            WHERE msc.movie_id = m.id
        ) AS sub_categories
    FROM movies m
    WHERE m.status = true
    ORDER BY m.id desc
    OFFSET $1
    LIMIT $2
"#;
pub const GET_MOVIE: &str = r#"
    SELECT
        m.id,
        m.title,
        m.description,
        m.duration,
        m.image,
        m.status,
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
pub const CREATE_MOVIE: &str = r#"
    INSERT INTO movies (title, description, duration, category_id)
    VALUES ($1, $2, $3, 1) RETURNING id
"#;
pub const CREATE_MOVIE_SC: &str = r#"
    INSERT INTO movies_sub_categories (movie_id, sub_category_id)
    VALUES ($1, $2)
"#;
pub const UPDATE_MOVIE_IMAGE: &str = r#"
    UPDATE movies SET image = $1 WHERE id = $2
"#;
