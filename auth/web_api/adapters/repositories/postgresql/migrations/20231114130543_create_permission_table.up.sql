CREATE TABLE "permission" (
  "user_id" uuid,
  "group" int NOT NULL
);

ALTER TABLE "permission" ADD FOREIGN KEY ("user_id") REFERENCES "user" ("id");
