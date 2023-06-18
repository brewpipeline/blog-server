CREATE TABLE `author` (
  `authorname` VARCHAR(50) NOT NULL,
  `firstName` VARCHAR(50) NULL DEFAULT NULL,
  `middleName` VARCHAR(50) NULL DEFAULT NULL,
  `lastName` VARCHAR(50) NULL DEFAULT NULL,
  `mobile` VARCHAR(15) NULL,
  `email` VARCHAR(50) NULL,
  `passwordHash` VARCHAR(32) NOT NULL,
  `registeredAt` DATETIME NOT NULL,
  `status` TINYTEXT NULL DEFAULT NULL,
  PRIMARY KEY (`authorname`),
  UNIQUE INDEX `uq_mobile` (`mobile` ASC),
  UNIQUE INDEX `uq_email` (`email` ASC) );