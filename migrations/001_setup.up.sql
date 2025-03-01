CREATE TABLE IF NOT EXISTS users (
    username VARCHAR(255) PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tasks (
    task_uuid uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_username VARCHAR(255),
    FOREIGN KEY (user_username) REFERENCES users(username)
);

CREATE INDEX idx_user_username ON tasks(user_username);

CREATE TABLE IF NOT EXISTS tracking (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    status TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    task_task_uuid uuid,
    FOREIGN KEY (task_task_uuid) REFERENCES tasks(task_uuid)
);

CREATE INDEX idx_task_task_uuid ON tracking(task_task_uuid);

