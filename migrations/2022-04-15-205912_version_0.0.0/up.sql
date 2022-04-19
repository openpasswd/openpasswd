-- Your SQL goes here
CREATE TABLE users (
  -- id INT PRIMARY KEY AUTO_INCREMENT,
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL,
  password TEXT NOT NULL,
  last_login TIMESTAMP NULL,
  fail_attempts SMALLINT NOT NULL DEFAULT(0),
  last_attempt TIMESTAMP NULL
);

CREATE UNIQUE INDEX index_user_email on users(email);

CREATE TABLE account_groups (
  -- id INT PRIMARY KEY AUTO_INCREMENT,
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  user_id INT NOT NULL,
  FOREIGN KEY (user_id) 
      REFERENCES users (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);

CREATE TABLE accounts (
  -- id INT PRIMARY KEY AUTO_INCREMENT,
  id SERIAL PRIMARY KEY,
  level SMALLINT NOT NULL DEFAULT(0),
  name VARCHAR(50) NOT NULL,
  username VARCHAR(100) NOT NULL,
  password TEXT NOT NULL,
  account_groups_id INT NOT NULL,
  user_id INT NOT NULL,
  FOREIGN KEY (user_id) 
      REFERENCES users (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION,
  FOREIGN KEY (account_groups_id) 
      REFERENCES account_groups (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);

CREATE TABLE devices (
  -- id INT PRIMARY KEY AUTO_INCREMENT,
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  last_access TIMESTAMP NOT NULL,
  active BOOLEAN NOT NULL,
  public_key TEXT NOT NULL,
  user_id INT NOT NULL,
  FOREIGN KEY (user_id) 
      REFERENCES users (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);

CREATE UNIQUE INDEX index_devices_id_name on devices(id, name);
