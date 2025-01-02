DO $$ BEGIN
  IF NOT EXISTS(select * from pg_tables where schemaname = 'public' and tablename = 'author') THEN
    CREATE SEQUENCE author_seq;

    CREATE TABLE author (
      id BIGINT NOT NULL DEFAULT NEXTVAL ('author_seq'),
      slug VARCHAR(100) NOT NULL,
      first_name VARCHAR(50) NULL DEFAULT NULL,
      middle_name VARCHAR(50) NULL DEFAULT NULL,
      last_name VARCHAR(50) NULL DEFAULT NULL,
      mobile VARCHAR(15) NULL,
      email VARCHAR(50) NULL,
      password_hash VARCHAR(100) NOT NULL,
      registered_at TIMESTAMP(0) NOT NULL,
      status VARCHAR(255) NULL DEFAULT NULL,
      PRIMARY KEY (id),
      CONSTRAINT uq_author_slug UNIQUE  (slug),
      CONSTRAINT uq_author_mobile UNIQUE  (mobile),
      CONSTRAINT uq_author_email UNIQUE  (email) );
  END IF;

  IF NOT EXISTS(select * from pg_tables where schemaname = 'public' and tablename = 'post') THEN
    CREATE SEQUENCE post_seq;

    CREATE TABLE post (
      id BIGINT NOT NULL DEFAULT NEXTVAL ('post_seq'),
      author_id BIGINT NOT NULL,
      title VARCHAR(75) NOT NULL,
      slug VARCHAR(100) NOT NULL,
      summary VARCHAR(255) NOT NULL,
      published SMALLINT NOT NULL DEFAULT 0,
      created_at TIMESTAMP(0) NOT NULL,
      content TEXT NULL DEFAULT NULL,
      PRIMARY KEY (id),
      CONSTRAINT uq_post_slug UNIQUE  (slug),
      CONSTRAINT fk_post_author
        FOREIGN KEY (author_id)
        REFERENCES author (id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION);

    CREATE INDEX idx_post_author ON post (author_id ASC);
  END IF;

  IF NOT EXISTS(select * from pg_tables where schemaname = 'public' and tablename = 'tag') THEN
    CREATE SEQUENCE tag_seq;

    CREATE TABLE tag (
      id BIGINT NOT NULL DEFAULT NEXTVAL ('tag_seq'),
      title VARCHAR(75) NOT NULL,
      published SMALLINT NOT NULL DEFAULT 0,
      slug VARCHAR(100) NOT NULL,
      PRIMARY KEY (id),
      CONSTRAINT uq_tag_slug UNIQUE  (slug));

    CREATE TABLE post_tag (
      post_id BIGINT NOT NULL,
      tag_id BIGINT NOT NULL,
      PRIMARY KEY (post_id, tag_id),
      CONSTRAINT fk_pt_post
        FOREIGN KEY (post_id)
        REFERENCES post (id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,
      CONSTRAINT fk_pt_tag
        FOREIGN KEY (tag_id)
        REFERENCES tag (id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION);

    CREATE INDEX idx_pt_tag ON post_tag (tag_id ASC);
    CREATE INDEX idx_pt_post ON post_tag (post_id ASC);
  END IF;

  IF NOT EXISTS(select * from pg_tables where schemaname = 'public' and tablename = 'post_comment') THEN
    CREATE SEQUENCE post_comment_seq;

    CREATE TABLE post_comment (
      id BIGINT NOT NULL DEFAULT NEXTVAL ('post_comment_seq'),
      post_id BIGINT NOT NULL,
      author_id BIGINT NOT NULL,
      published SMALLINT NOT NULL DEFAULT 0,
      created_at TIMESTAMP(0) NOT NULL,
      content TEXT NOT NULL,
      PRIMARY KEY (id),
      CONSTRAINT fk_comment_post
        FOREIGN KEY (post_id)
        REFERENCES post (id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,
      CONSTRAINT fk_comment_author
        FOREIGN KEY (author_id)
        REFERENCES author (id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION);

    CREATE INDEX idx_comment_post ON post_comment (post_id ASC);
  END IF;
END $$

;

DO $$ BEGIN
  IF EXISTS(select * from pg_constraint where conname = 'uq_post_slug') THEN
    ALTER TABLE post DROP CONSTRAINT uq_post_slug;
  END IF;
END $$

;

DO $$ BEGIN
  IF EXISTS(select * from pg_constraint where conname = 'uq_tag_slug') THEN
    ALTER TABLE tag DROP CONSTRAINT uq_tag_slug;
  END IF;

  IF NOT EXISTS(select * from pg_constraint where conname = 'uq_tag_title') THEN
    --delete duplicated by title tags and all post_tag references before adding unique constraint
    DELETE FROM post_tag
    WHERE tag_id IN
    (
      SELECT orig.id FROM tag as orig
      JOIN 
        (
          SELECT title FROM tag
          GROUP BY title
          HAVING COUNT(1) > 1
        ) AS sQ 
      ON orig.title = sQ.title
    );

    DELETE FROM tag
    WHERE title IN 
    (
      SELECT title FROM tag
      GROUP BY title
      HAVING COUNT(1) > 1
    );    

    ALTER TABLE tag ADD CONSTRAINT uq_tag_title UNIQUE (title);
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'post' and column_name = 'image_url') THEN
    ALTER TABLE post ADD COLUMN image_url VARCHAR(150);
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'author' and column_name = 'image_url') THEN
    ALTER TABLE author ADD COLUMN image_url VARCHAR(150);
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'author' and column_name = 'editor') THEN
    ALTER TABLE author ADD COLUMN editor SMALLINT NOT NULL DEFAULT 0;
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'author' and column_name = 'blocked') THEN
    ALTER TABLE author ADD COLUMN blocked SMALLINT NOT NULL DEFAULT 0;
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'author' and column_name = 'yandex_id') THEN
    ALTER TABLE author ADD COLUMN yandex_id BIGINT;
    ALTER TABLE author ADD CONSTRAINT uq_yandex_id UNIQUE (yandex_id);
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'author' and column_name = 'password_hash' and is_nullable = 'YES') THEN
    ALTER TABLE author ALTER COLUMN password_hash DROP NOT NULL;
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'author' and column_name = 'telegram_id') THEN
    ALTER TABLE author ADD COLUMN telegram_id BIGINT;
    ALTER TABLE author ADD CONSTRAINT uq_telegram_id UNIQUE (telegram_id);
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'author' and column_name = 'notification_subscribed') THEN
    ALTER TABLE author ADD COLUMN notification_subscribed SMALLINT NOT NULL DEFAULT 0;
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'author' and column_name = 'override_social_data') THEN
    ALTER TABLE author ADD COLUMN override_social_data SMALLINT NOT NULL DEFAULT 0;
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from information_schema.columns where table_name = 'post' and column_name = 'plain_text_content') THEN
    ALTER TABLE post ADD COLUMN plain_text_content TEXT;
  END IF;
END $$

;

DO $$ BEGIN
  IF NOT EXISTS(select * from pg_tables where schemaname = 'public' and tablename = 'migration') THEN
    CREATE TABLE migration (
      key VARCHAR(100) NOT NULL,
      created_at TIMESTAMP(0) NOT NULL,
      PRIMARY KEY (key)
    );
  END IF;
END $$

;

DO $$ BEGIN
  IF EXISTS(select * from information_schema.columns where table_name = 'post' and column_name = 'published') THEN
ALTER TABLE post RENAME COLUMN published TO published_type;
END IF;
END $$

;