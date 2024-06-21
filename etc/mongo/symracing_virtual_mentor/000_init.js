// Create a new database and switch to it
db = db.getSiblingDB('symracing_virtual_mentor');
// Create a user with read and write privileges for the database
db.createUser({
    user: 'admin',
    pwd: '1234',
    roles: [
        {
            role: 'readWrite',
            db: 'symracing_virtual_mentor',
        },
    ],
});
