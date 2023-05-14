-- Add up migration script here
CREATE TABLE IF NOT EXISTS tbl_category (
    category_id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    category_name VARCHAR(255) NOT NULL,
    category_description TEXT,
    PRIMARY KEY (category_id)
);

CREATE TABLE IF NOT EXISTS tbl_duration_task (
    task_id INT UNSIGNED NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    PRIMARY KEY (task_id)
);

CREATE TABLE IF NOT EXISTS tbl_reminder_task (
    task_id INT UNSIGNED NOT NULL,
    remind_time TIMESTAMP NOT NULL,
    PRIMARY KEY (task_id)
);

CREATE TABLE IF NOT EXISTS tbl_tag (
    tag_id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    tag_name VARCHAR(255) NOT NULL,
    PRIMARY KEY (tag_id)
);

CREATE TABLE IF NOT EXISTS tbl_task (
    task_id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    task_title VARCHAR(255) NOT NULL,
    task_description TEXT,
    task_deadline TIMESTAMP,
    task_priority INT UNSIGNED,
    task_status TEXT,
    category_id INT UNSIGNED,
    PRIMARY KEY (task_id)
);

CREATE TABLE IF NOT EXISTS tbl_task_tag (
    task_id INT UNSIGNED NOT NULL,
    tag_id INT UNSIGNED NOT NULL,
    PRIMARY KEY (task_id, tag_id)
);

ALTER TABLE
    tbl_duration_task
ADD
    CONSTRAINT fk_duration_task FOREIGN KEY (task_id) REFERENCES tbl_task (task_id);

ALTER TABLE
    tbl_reminder_task
ADD
    CONSTRAINT fk_reminder_task FOREIGN KEY (task_id) REFERENCES tbl_task (task_id);

ALTER TABLE
    tbl_task
ADD
    CONSTRAINT fk_task FOREIGN KEY (category_id) REFERENCES tbl_category (category_id);

ALTER TABLE
    tbl_task_tag
ADD
    CONSTRAINT fk_task_tag_1 FOREIGN KEY (task_id) REFERENCES tbl_task (task_id);

ALTER TABLE
    tbl_task_tag
ADD
    CONSTRAINT fk_task_tag_2 FOREIGN KEY (tag_id) REFERENCES tbl_tag (tag_id);

CREATE TRIGGER trg_category_delete
BEFORE
    DELETE ON tbl_category FOR EACH ROW BEGIN
UPDATE
    tbl_task
SET
    category_id = NULL
WHERE
    category_id = OLD.category_id;

END;

CREATE TRIGGER trg_tag_delete
BEFORE
    DELETE ON tbl_tag FOR EACH ROW BEGIN
DELETE FROM
    tbl_task_tag
WHERE
    tag_id = OLD.tag_id;

END;