install:
	docker run -p 127.0.0.1:3306:3306 --name rustdb -e MARIADB_ROOT_PASSWORD=p@ssw0rd -d mariadb:latest
	docker exec -it rustdb mariadb -uroot -pp@ssw0rd -e "CREATE DATABASE rustdb;"
	docker exec -it rustdb mariadb -uroot -pp@ssw0rd -e "SHOW DATABASES;"
	echo "DATABASE_URL=mysql://root:p@ssw0rd@127.0.0.1:3306/rustdb" > .env
	echo "Done"

