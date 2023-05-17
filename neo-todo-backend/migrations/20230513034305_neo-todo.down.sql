-- Add down migration script here
ALTER TABLE
    tbl_duration_task DROP FOREIGN KEY fk_duration_task;

ALTER TABLE
    tbl_reminder_task DROP FOREIGN KEY fk_reminder_task;

ALTER TABLE
    tbl_task DROP FOREIGN KEY fk_task;

ALTER TABLE
    tbl_task_tag DROP FOREIGN KEY fk_task_tag_1;

ALTER TABLE
    tbl_task_tag DROP FOREIGN KEY fk_task_tag_2;

DROP PROCEDURE to_common;

DROP PROCEDURE to_duration;

DROP PROCEDURE to_reminder;

DROP PROCEDURE update_duration;

DROP PROCEDURE update_reminder;

DROP PROCEDURE update_common;

DROP PROCEDURE add_task_tag;

DROP TRIGGER trg_task_status_change;

DROP VIEW v_duration_task;

DROP VIEW v_reminder_task;

DROP VIEW v_task_tag;

DROP TABLE tbl_category;

DROP TABLE tbl_duration_task;

DROP TABLE tbl_reminder_task;

DROP TABLE tbl_tag;

DROP TABLE tbl_task;

DROP TABLE tbl_task_tag;