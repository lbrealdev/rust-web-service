ALTER TABLE answers DROP CONSTRAINT IF EXISTS answers_content_min_len;
ALTER TABLE questions DROP CONSTRAINT IF EXISTS questions_content_min_len;
ALTER TABLE questions DROP CONSTRAINT IF EXISTS questions_title_min_len;
DROP INDEX IF EXISTS answers_question_id_idx;
