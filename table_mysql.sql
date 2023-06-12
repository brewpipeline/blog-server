CREATE TABLE `user` (
  `username` VARCHAR(50) NOT NULL,
  `first_name` VARCHAR(50) NULL DEFAULT NULL,
  `middle_name` VARCHAR(50) NULL DEFAULT NULL,
  `last_name` VARCHAR(50) NULL DEFAULT NULL,
  `mobile` VARCHAR(15) NULL,
  `email` VARCHAR(50) NULL,
  `password_hash` VARCHAR(32) NOT NULL,
  `registered_at` DATETIME NOT NULL,
  `status` TINYTEXT NULL DEFAULT NULL,
  PRIMARY KEY (`username`),
  UNIQUE INDEX `uq_mobile` (`mobile` ASC),
  UNIQUE INDEX `uq_email` (`email` ASC) );