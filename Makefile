db:
	@psql -U postgres -d postgres -f ./src/database/init.sql
	@psql -U postgres -d media -f ./src/database/create.sql
	@psql -U postgres -d media -f ./src/database/insert.sql
