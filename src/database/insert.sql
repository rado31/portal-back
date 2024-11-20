INSERT INTO admins (login, password)
VALUES ('rado', '$2y$12$z9AtDma2Ewz/Z/PR58H7h.PrZB8/y/yGT1g.Y8RinWgG/IoBkP9q.');

INSERT INTO categories (title)
VALUES
	('{ "tk": "kinolar", "ru": "фильмы" }'),
	('{ "tk": "aydymlar", "ru": "музыка" }');

INSERT INTO sub_categories (title, category_id)
VALUES
	('{ "tk": "Harby", "ru": "Военный" }', 1),
	('{ "tk": "Maşgala üçin", "ru": "Семейный" }', 1),
	('{ "tk": "Dokumental", "ru": "Документальный" }', 1),
	('{ "tk": "Taryhy", "ru": "Исторический" }', 1),
	('{ "tk": "Başdan geçirmel", "ru": "Приключения" }', 1),
	('{ "tk": "Fantastika", "ru": "Фантастика" }', 1),
	('{ "tk": "Söweşiji", "ru": "Боевик" }', 1),
	('{ "tk": "Detektiw", "ru": "Детективный" }', 1),
	('{ "tk": "Çagalar üçin", "ru": "Для детей" }', 1),
	('{ "tk": "Drama", "ru": "Драма" }', 1),
	('{ "tk": "Komediýa", "ru": "Комедийный" }', 1),
	('{ "tk": "Triller", "ru": "Триллер" }', 1);
