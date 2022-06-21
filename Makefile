setup:
	cargo install sea-orm-cli
refresh:
	sea-orm-cli migrate refresh
generate:
	sea-orm-cli generate entity -o entity/src
docker_dev_up:
	docker-compose up -d redis postgres mailhog
docker_test_up:
	docker-compose up -d
docker_down:
	docker-compose down
build:
	cargo build -p openpasswd-server
run:
	cargo run -p openpasswd-server