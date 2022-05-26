-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL,
  password TEXT NOT NULL,
  master_key VARCHAR(32) NULL,
  last_login TIMESTAMP NULL,
  fail_attempts SMALLINT NOT NULL DEFAULT(0),
  last_attempt TIMESTAMP NULL
);

CREATE UNIQUE INDEX index_user_email on users(email);

CREATE TABLE user_password_recovery (
  id uuid PRIMARY KEY NOT NULL,
  user_id INT NOT NULL,
  issued_at TIMESTAMP NULL,
  FOREIGN KEY (user_id) 
      REFERENCES users (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);

CREATE TABLE account_groups (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  name VARCHAR(50) NOT NULL,
  FOREIGN KEY (user_id) 
      REFERENCES users (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);

CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  account_groups_id INT NOT NULL,
  level SMALLINT NOT NULL DEFAULT(0),
  name VARCHAR(50) NOT NULL,
  FOREIGN KEY (user_id) 
      REFERENCES users (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION,
  FOREIGN KEY (account_groups_id) 
      REFERENCES account_groups (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);

CREATE TABLE account_passwords (
  id SERIAL PRIMARY KEY,
  account_id INT NOT NULL,
  username VARCHAR(100) NOT NULL,
  password BYTEA NOT NULL,
  created_date TIMESTAMP NOT NULL,
  FOREIGN KEY (account_id) 
      REFERENCES accounts (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);

CREATE TABLE devices (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  name VARCHAR(100) NOT NULL,
  last_access TIMESTAMP NOT NULL,
  active BOOLEAN NOT NULL,
  public_key TEXT NOT NULL,
  FOREIGN KEY (user_id) 
      REFERENCES users (id) 
         ON DELETE NO ACTION 
         ON UPDATE NO ACTION
);

CREATE UNIQUE INDEX index_devices_id_name on devices(id, name);
