CREATE TABLE `user`
             (
                          `id`             INTEGER PRIMARY KEY autoincrement,
                          `user_id`        VARCHAR(50) NOT NULL,
                          `user_account`   VARCHAR(50) NOT NULL,
                          `user_password`  VARCHAR(50) NOT NULL,
                          `user_authority` INT(10) NOT NULL DEFAULT 0,
                          `timestamp` timestamp(3)              );