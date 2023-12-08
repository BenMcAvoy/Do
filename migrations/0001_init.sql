CREATE TABLE `Tasks` (
    `TaskId` INT  NOT NULL,
    `Name` VARCHAR(255)  NOT NULL,
    `Description` VARCHAR(255)  NOT NULL,
    `UserID` INT  NOT NULL,
    PRIMARY KEY (
        `TaskId`
    )
);

CREATE TABLE `Users` (
    `UserId` INT  NOT NULL,
    `Username` VARCHAR(255)  NOT NULL,
    `Hash` VARCHAR(255)  NOT NULL,
    PRIMARY KEY (
        `UserId`
    )
);

ALTER TABLE `Tasks` ADD CONSTRAINT `fk_Tasks_UserID` FOREIGN KEY(`UserID`)
REFERENCES `Users` (`UserId`);
