CREATE TABLE `todos` (
    `TodoId` INT AUTO_INCREMENT NOT NULL,
    `TaskId` INT  NOT NULL,
    `UserId` INT  NOT NULL,
    PRIMARY KEY (
        `TodoId`
    )
);

CREATE TABLE `tasks` (
    `TaskId` INT AUTO_INCREMENT NOT NULL,
    `Name` VARCHAR(255)  NOT NULL,
    `Description` VARCHAR(255)  NOT NULL,
    PRIMARY KEY (
        `TaskId`
    )
);

CREATE TABLE `users` (
    `UserID` INT AUTO_INCREMENT NOT NULL,
    `Name` VARCHAR(255)  NOT NULL,
    `Hash` VARCHAR(255)  NOT NULL,
    PRIMARY KEY (
        `UserID`
    )
);

ALTER TABLE `todos` ADD CONSTRAINT `fk_todos_TaskId` FOREIGN KEY(`TaskId`)
REFERENCES `tasks` (`TaskId`);

ALTER TABLE `todos` ADD CONSTRAINT `fk_todos_UserId` FOREIGN KEY(`UserId`)
REFERENCES `users` (`UserID`);
