-- Your SQL goes here
CREATE TABLE `users`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`name` TEXT NOT NULL,
	`email` TEXT NOT NULL
);

INSERT INTO users (name, email) VALUES ("art", "art0139@gmail.com");
INSERT INTO users (name, email) VALUES ("luvcas", "luyas@gmail.com");

