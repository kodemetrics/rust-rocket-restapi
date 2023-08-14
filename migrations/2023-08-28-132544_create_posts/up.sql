-- Your SQL goes here

CREATE TABLE posts (
  id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
  title VARCHAR(255) NOT NULL, -- Specify a suitable length for VARCHAR
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO posts (title, body, published) VALUES
  ('Title 1', 'Body content for post 1.', TRUE),
  ('Title 2', 'Body content for post 2.', TRUE),
  ('Title 3', 'Body content for post 3.', FALSE),
  ('Title 4', 'Body content for post 4.', TRUE),
  ('Title 5', 'Body content for post 5.', FALSE),
  ('Title 6', 'Body content for post 6.', TRUE),
  ('Title 7', 'Body content for post 7.', FALSE),
  ('Title 8', 'Body content for post 8.', TRUE),
  ('Title 9', 'Body content for post 9.', TRUE),
  ('Title 10', 'Body content for post 10.', FALSE),
  ('Title 11', 'Body content for post 11.', TRUE),
  ('Title 12', 'Body content for post 12.', FALSE),
  ('Title 13', 'Body content for post 13.', TRUE),
  ('Title 14', 'Body content for post 14.', TRUE),
  ('Title 15', 'Body content for post 15.', FALSE);