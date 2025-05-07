-- Your SQL goes here

-- Your SQL goes here
CREATE TABLE "pets" (
    "id" SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "tutor_id" INTEGER NOT NULL
);


ALTER TABLE pets ADD CONSTRAINT fk_pet_user FOREIGN KEY (tutor_id) REFERENCES users(id) ON DELETE CASCADE;