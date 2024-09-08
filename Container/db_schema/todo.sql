CREATE TABLE todos (
  id BIGSERIAL NOT NULL PRIMARY KEY,
  title VARCHAR(80) NOT NULL,
  completion_status BOOLEAN NOT NULL,
  date_created TIMESTAMP NOT NULL,
  date_modified TIMESTAMP NOT NULL,
  description TEXT,
  date_completed TIMESTAMP,
  date_deadline TIMESTAMP
)