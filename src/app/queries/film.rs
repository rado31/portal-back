pub const GET_FILMS: &str = r#"
    SELECT
        f.id,
        f.title,
        f.description,
        f.duration,
        f.image,
        ARRAY(
            SELECT
                JSON_BUILD_OBJECT(
                    'id', fsc.sub_category_id,
                    'title', (
                        SELECT sc.title
                        FROM sub_categories sc
                        WHERE sc.id = fsc.sub_category_id
                    )
                )::VARCHAR
            FROM films_sub_categories fsc
            WHERE fsc.film_id = f.id
        ) AS sub_categories
    FROM films f
    ORDER BY f.id desc
    OFFSET $1
    LIMIT $2
"#;
pub const GET_FILM: &str = r#"
    SELECT
        f.id,
        f.title,
        f.description,
        f.duration,
        f.image,
        ARRAY(
            SELECT
                JSON_BUILD_OBJECT(
                    'id', fsc.sub_category_id,
                    'title', (
                        SELECT sc.title
                        FROM sub_categories sc
                        WHERE sc.id = fsc.sub_category_id
                    )
                )::VARCHAR
            FROM films_sub_categories fsc
            WHERE fsc.film_id = f.id
        ) AS sub_categories
    FROM films f
    WHERE f.id = $1
"#;
pub const CREATE_FILM: &str = r#"
    INSERT INTO films (title, description, duration, category_id)
    VALUES ($1, $2, $3, $4) RETURNING id
"#;
