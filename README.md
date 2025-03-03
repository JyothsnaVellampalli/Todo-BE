# Todo Application Backend  

This is a backend service for a Todo application, featuring authentication and full CRUD operations.  

## Setup  

### 1. Create a PostgreSQL Container  
```sh
docker run --name my_postgres_container \
  --network my_network \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=1234 \
  -e POSTGRES_DB=mydatabase \
  -p 5432:5432 \
  -d postgres:latest
```
### 2. Create a pgAdmin Container
```
docker run --name my_pgadmin_container \
  --network my_network \
  -e PGADMIN_DEFAULT_EMAIL=jyothsna@example.com \
  -e PGADMIN_DEFAULT_PASSWORD=1234 \
  -p 5050:80 \
  -d dpage/pgadmin4
```
### 3.  Install sqlx-cli
```
cargo install sqlx-cli
```
### 4. Run SQL Migrations
```
sqlx migrate run
```
## Features  
- ✅ Connected to PostgreSQL database  
- ✅ Authentication flow implemented  
- ✅ Backend logging enabled  
- ✅ APIs for all CRUD operations  

## Future Enhancements  
- 🔹 Connect to frontend  
- 🔹 Implement rate limiting  
- 🔹 Add test cases  
- 🔹 Support for GraphQL (GQL)  
- 🔹 Add a Dockerfile  
