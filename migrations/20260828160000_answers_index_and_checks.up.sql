CREATE INDEX IF NOT EXISTS answers_question_id_idx ON answers(question_id);

ALTER TABLE questions
    ADD CONSTRAINT questions_title_min_len CHECK (char_length(btrim(title)) >= 1);

ALTER TABLE questions
    ADD CONSTRAINT questions_content_min_len CHECK (char_length(btrim(content)) >= 1);

ALTER TABLE answers
    ADD CONSTRAINT answers_content_min_len CHECK (char_length(btrim(content)) >= 1);
