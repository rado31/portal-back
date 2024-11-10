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

CREATE TABLE movies (
	id SERIAL PRIMARY KEY,
	title JSONB NOT NULL,
	description JSONB NOT NULL,
	duration INTEGER NOT NULL,
	image VARCHAR NULL,
	status BOOLEAN DEFAULT true,
	category_id INTEGER DEFAULT 1,

	CONSTRAINT category_id
		FOREIGN KEY (category_id)
			REFERENCES categories (id)
				ON DELETE SET NULL
);

CREATE TABLE movies_sub_categories (
	movie_id INTEGER NOT NULL,
	sub_category_id INTEGER NULL,

	CONSTRAINT movie_id
		FOREIGN KEY (movie_id)
			REFERENCES movies (id)
				ON DELETE CASCADE,

	CONSTRAINT sub_category_id
		FOREIGN KEY (sub_category_id)
			REFERENCES sub_categories (id)
				ON DELETE SET NULL
);
