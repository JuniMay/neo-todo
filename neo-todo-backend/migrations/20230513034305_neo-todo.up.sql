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
    -- 0: common
    -- 1: duration
    -- 2: reminder
    kind INT UNSIGNED DEFAULT 0,
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

CREATE VIEW v_duration_task AS
SELECT
    *
FROM
    tbl_task NATURAL JOIN tbl_duration_task;

CREATE VIEW v_reminder_task AS
SELECT
    *
FROM
    tbl_task NATURAL JOIN tbl_reminder_task;

CREATE VIEW v_task_tag AS
SELECT
    *
FROM tbl_task_tag NATURAL JOIN tbl_tag;

-- Convertion to common task
CREATE PROCEDURE to_common(IN in_task_id INT)
BEGIN
    DECLARE exit handler for sqlexception
    BEGIN
        -- ERROR occurred, rollback
        ROLLBACK;
    END;

    START TRANSACTION;
    
    DELETE FROM tbl_duration_task WHERE task_id = in_task_id;
    DELETE FROM tbl_reminder_task WHERE task_id = in_task_id;

    UPDATE tbl_task SET kind = 0 WHERE task_id = in_task_id;

    COMMIT;
END;


-- Converting to duration task
CREATE PROCEDURE to_duration(
    IN in_task_id INT UNSIGNED, IN in_start_time TIMESTAMP, IN in_end_time TIMESTAMP
) BEGIN
    DECLARE exit handler for sqlexception
    BEGIN
        -- ERROR occurred, rollback
        ROLLBACK;
    END;

    START TRANSACTION;

    -- overwrite
    DELETE FROM tbl_duration_task WHERE task_id = in_task_id;
    DELETE FROM tbl_reminder_task WHERE task_id = in_task_id;
    
    INSERT INTO tbl_duration_task(task_id, start_time, end_time) 
    VALUES (in_task_id, in_start_time, in_end_time);

    UPDATE tbl_task SET kind = 1 WHERE task_id = in_task_id;

    COMMIT;
END;

-- Converting to reminder task
CREATE PROCEDURE to_reminder(
    IN in_task_id INT UNSIGNED, IN in_remind_time TIMESTAMP
) BEGIN
    DECLARE exit handler for sqlexception
    BEGIN
        -- ERROR occurred, rollback
        ROLLBACK;
    END;

    START TRANSACTION;
    -- overwrite
    DELETE FROM tbl_duration_task WHERE task_id = in_task_id;
    DELETE FROM tbl_reminder_task WHERE task_id = in_task_id;
    
    INSERT INTO tbl_reminder_task(task_id, remind_time) VALUES(in_task_id, in_remind_time);

    UPDATE tbl_task SET kind = 2 WHERE task_id = in_task_id;

    COMMIT;
END;

-- Update duration start/end time
CREATE PROCEDURE update_duration(
    IN in_task_id INT UNSIGNED, IN in_start_time TIMESTAMP, IN in_end_time TIMESTAMP
) BEGIN
    DECLARE exit handler for sqlexception
    BEGIN
        -- ERROR occurred, rollback
        ROLLBACK;
    END;

    START TRANSACTION;

    IF NOT EXISTS (SELECT * FROM tbl_duration_task WHERE task_id = in_task_id) THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Task ID not found in tbl_duration_task';
    END IF;
    
    UPDATE tbl_duration_task SET start_time = in_start_time, end_time = in_end_time WHERE task_id = in_task_id;

    COMMIT;
END;

-- update reminder time
CREATE PROCEDURE update_reminder(
    IN in_task_id INT UNSIGNED, IN in_remind_time TIMESTAMP
) BEGIN
    DECLARE exit handler for sqlexception
    BEGIN
        -- ERROR occurred, rollback
        ROLLBACK;
    END;

    START TRANSACTION;

    IF NOT EXISTS (SELECT * FROM tbl_reminder_task WHERE task_id = in_task_id) THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Task ID not found in tbl_reminder_task';
    END IF;
    
    UPDATE tbl_reminder_task SET remind_time = in_remind_time WHERE task_id = in_task_id;

    COMMIT;
END;

CREATE PROCEDURE update_common(
    IN in_task_id INT UNSIGNED,
    IN in_task_title VARCHAR(255),
    IN in_task_description TEXT,
    IN in_task_deadline TIMESTAMP,
    IN in_task_priority INT UNSIGNED,
    IN in_task_status TEXT,
    IN in_category_id INT UNSIGNED
) BEGIN

    DECLARE exit handler for sqlexception
    BEGIN
        -- ERROR occurred, rollback
        ROLLBACK;
    END;

    START TRANSACTION;

    IF NOT EXISTS (SELECT * FROM tbl_task WHERE task_id = in_task_id) THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Task ID not found in tbl_task';
    END IF;
    
    UPDATE tbl_task SET
        task_title = in_task_title,
        task_description = in_task_description,
        task_deadline = in_task_deadline,
        task_priority = in_task_priority,
        task_status = in_task_status,
        category_id = in_category_id
    WHERE task_id = in_task_id;

    COMMIT;
END;

CREATE PROCEDURE add_task_tag(
    IN in_task_id INT UNSIGNED, IN in_tag_id INT UNSIGNED
) BEGIN
    DECLARE exit handler for sqlexception
    BEGIN
        -- ERROR occurred, rollback
        ROLLBACK;
    END;

    START TRANSACTION;

    IF NOT EXISTS (SELECT * FROM tbl_task WHERE task_id = in_task_id) THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Task ID not found in tbl_task';
    END IF;

    IF NOT EXISTS (SELECT * FROM tbl_tag WHERE tag_id = in_tag_id) THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Tag ID not found in tbl_tag';
    END IF;

    IF EXISTS (SELECT * FROM tbl_task_tag WHERE task_id = in_task_id AND tag_id = in_tag_id) THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Task ID and Tag ID already exists in tbl_task_tag';
    END IF;

    INSERT INTO tbl_task_tag(task_id, tag_id) VALUES(in_task_id, in_tag_id);

    COMMIT;
END;

