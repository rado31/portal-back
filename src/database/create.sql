CREATE TABLE categories (
	id SERIAL PRIMARY KEY,
	title JSONB NOT NULL
);

CREATE TABLE sub_categories (
	id SERIAL PRIMARY KEY,
	title JSONB NOT NULL,
	category_id INTEGER NULL,

	CONSTRAINT category_id
		FOREIGN KEY (category_id)
			REFERENCES categories (id)
				ON DELETE SET NULL
);

CREATE TABLE admins (
	id SERIAL PRIMARY KEY,
	login VARCHAR NOT NULL,
	password VARCHAR NOT NULL
);
