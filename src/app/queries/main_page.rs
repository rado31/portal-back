pub const GET_MAIN_PAGE_DATA: &str = r#"
    SELECT
        s.id,
        s.title,
        ARRAY(
            SELECT JSON_BUILD_OBJECT(
                'id', m.id,
                'title', m.title,
                'description', m.description,
                'duration', m.duration,
                'image', m.image
            )::VARCHAR
            FROM movies m
            INNER JOIN movies_sub_categories msc ON msc.movie_id = m.id
            WHERE msc.sub_category_id = s.id
            ORDER BY m.id DESC
            LIMIT 10
        ) AS movies
    FROM sub_categories s
"#;
