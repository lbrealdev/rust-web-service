ALTER TABLE answers
    DROP CONSTRAINT IF EXISTS answers_corresponding_question_fkey;

ALTER TABLE answers
    RENAME COLUMN corresponding_question TO question_id;

ALTER TABLE answers
    ADD CONSTRAINT answers_question_id_fkey
    FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE;
