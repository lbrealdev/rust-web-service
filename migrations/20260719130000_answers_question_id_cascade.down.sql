ALTER TABLE answers
    DROP CONSTRAINT IF EXISTS answers_question_id_fkey;

ALTER TABLE answers
    RENAME COLUMN question_id TO corresponding_question;

ALTER TABLE answers
    ADD CONSTRAINT answers_corresponding_question_fkey
    FOREIGN KEY (corresponding_question) REFERENCES questions(id);
