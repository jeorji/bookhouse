CREATE TABLE "user" (
  "id" uuid PRIMARY KEY,
  "email" text NOT NULL,
  "password_hash" text NOT NULL,
  "salt" text NOT NULL
);
