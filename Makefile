install:
	docker docker run --name rust-postgres -e POSTGRES_PASSWORD=p@ssw0rd -d postgres
	docker exec -it rust-postgres psql -U postgres -c "CREATE DATABASE rustdb;"
	echo "DATABASE_URL=mysql://root:p@ssw0rd@127.0.0.1:3306/rustdb" > .env
	echo "Done"

