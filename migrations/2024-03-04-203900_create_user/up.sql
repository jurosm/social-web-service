-- Your SQL goes here
CREATE TABLE "user" (
	id SERIAL PRIMARY KEY,
	first_name VARCHAR(255),
	last_name VARCHAR(255),
	username VARCHAR(255),
	email VARCHAR(255) NOT NULL,
	refresh_token VARCHAR(255),
	refresh_token_expiry VARCHAR(255),
	password VARCHAR(255) NOT NULL,
	CONSTRAINT UC_email UNIQUE (email)
);

CREATE UNIQUE INDEX UX_email ON "user"(email);