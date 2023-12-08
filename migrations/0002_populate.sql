INSERT INTO tasks (Name, Description, Completed)
VALUES
    ('First task', 'Description for Task 1', true),
    ('Second task', 'Description for Task 2', false),
    ('Third task', 'Description for Task 3', true);

INSERT INTO users (Name, Hash)
VALUES
    ('Bob', 'hash'),
    ('John', 'hash'),
    ('Steve', 'hash');

INSERT INTO todos (TaskID, UserID)
VALUES
    (1, 1),
    (2, 2),
    (3, 3);
