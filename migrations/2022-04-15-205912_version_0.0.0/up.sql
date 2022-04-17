-- Your SQL goes here
CREATE TABLE users (
  -- id INT PRIMARY KEY AUTO_INCREMENT,
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL,
  password TEXT NOT NULL
);

CREATE TABLE accounts (
  -- id INT PRIMARY KEY AUTO_INCREMENT,
  id SERIAL PRIMARY KEY,
  level INT NOT NULL,
  name VARCHAR(100) NOT NULL,
  username VARCHAR(100) NOT NULL,
  password TEXT NOT NULL,
  user_id INT NOT NULL,
  FOREIGN KEY (user_id) 
      REFERENCES users (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);

CREATE TABLE devices (
  -- id INT PRIMARY KEY AUTO_INCREMENT,
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  -- last_access DATETIME NOT NULL,
  last_access TIMESTAMP NOT NULL,
  active BOOLEAN NOT NULL,
  public_key TEXT NOT NULL,
  user_id INT NOT NULL,
  FOREIGN KEY (user_id) 
      REFERENCES users (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);