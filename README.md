docker run --name my_postgres_container   --network my_network   -e POSTGRES_USER=postgres   -e POSTGRES_PASSWORD=1234   -e POSTGRES_DB=mydatabase   -p 5432:5432   -d postgres:latest


docker run --name my_pgadmin_container   --network my_network   -e PGADMIN_DEFAULT_EMAIL=jyothsna@example.com   -e PGADMIN_DEFAULT_PASSWORD=1234   -p 5050:80   -d dpage/pgadmin4

#Install sqlx-cli
```cargo install sqlx-cli```

#Run SQL migrations
```sqlx migrate run```


**To check tables in container:**

docker exec -it my_postgres_container psql -U postgres

\c mydatabase

\dt



**To delete migartion:**

docker exec -it my_postgres_container psql -U postgres -d mydatabase

SELECT * FROM _sqlx_migrations;

DELETE FROM _sqlx_migrations WHERE version = '1';

DROP TYPE task_status CASCADE;


**TODO**
1. **Task status enum in database.
2. **Polish apis.
3. Add authentication(auth_token in headers).- done
4. Clear error messages. -done
5. Tracking. - done

Phase 2:
6. Connect to FE
7. Rate limiting
8. Test cases
9. GQL Support
10. docker file

**Steps**
1. Connect db with basic schema. - done
2. Add handlers to db crud. - done
3. Add routes. - done
4. Connect routes to db handlers - done

**Authentication**
1. Routes update to check auth - done
2. Add auth-lib -done
3. Connect auth-lib and routes auth-handler - done
4. Update handler to user-based fetching - done
