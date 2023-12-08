INSERT INTO tasks (Name, Description, Completed)
VALUES
    ('Homework', 'Do maths homework', true),
    ('Homework', 'Do science homework', false),
    ('Program', 'Do `Do` programming', true),
    ('Dinner', 'Make dinner', false);

INSERT INTO users (Name, Hash)
VALUES
    ('Bob', 'hash'),
    ('John', 'hash'),
    ('Steve', 'hash');

INSERT INTO todos (TaskID, UserID)
VALUES
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 1);
