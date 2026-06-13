INSERT INTO
    roles(name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users(name, email, password_hash, role_id)
SELECT
    'Terada Shunya',
    'TeradaShunya@example.com',
    '$2b$12$6oqFOPi7s417e6ECUgALVOqI0ad6D8wvGDN8aOIx9lYfuIjcQOz.O',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
