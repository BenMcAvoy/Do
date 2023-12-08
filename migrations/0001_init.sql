CREATE TABLE `todos` (
    `TodoID` INT AUTO_INCREMENT NOT NULL,
    `TaskID` INT NOT NULL,
    `UserID` INT NOT NULL,
    PRIMARY KEY (
        `TodoID`
    )
);

CREATE TABLE `tasks` (
    `TaskID` INT AUTO_INCREMENT NOT NULL,
    `Name` VARCHAR(255) NOT NULL,
    `Description` VARCHAR(255) NOT NULL,
    `Completed` BOOLEAN NOT NULL,
    PRIMARY KEY (
        `TaskID`
    )
);

CREATE TABLE `users` (
    `UserID` INT AUTO_INCREMENT NOT NULL,
    `Name` VARCHAR(255) NOT NULL,
    `Hash` VARCHAR(255) NOT NULL,
    PRIMARY KEY (
        `UserID`
    )
);

ALTER TABLE `todos` ADD CONSTRAINT `fk_todos_TaskID` FOREIGN KEY(`TaskID`)
REFERENCES `tasks` (`TaskID`);

ALTER TABLE `todos` ADD CONSTRAINT `fk_todos_UserID` FOREIGN KEY(`UserID`)
REFERENCES `users` (`UserID`);
