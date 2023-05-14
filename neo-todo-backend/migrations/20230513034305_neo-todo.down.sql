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

DROP TABLE tbl_category;

DROP TABLE tbl_duration_task;

DROP TABLE tbl_reminder_task;

DROP TABLE tbl_tag;

DROP TABLE tbl_task;

DROP TABLE tbl_task_tag;