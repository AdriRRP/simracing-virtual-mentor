if (typeof env.LOAD_FILE_TEST_DATA !== 'undefined' && env.LOAD_FILE_TEST_DATA === 'true') {
    // Create a new collection and insert documents
    db.file.insert([
        {
            _id: "TEST#FILE#001",
            name: "Test File 1",
            status: "accepted",
            created_on: new ISODate("2024-02-24T20:30:00Z"),
        },
        {
            _id: "TEST#FILE#002",
            name: "Test File 2",
            status: "success",
            created_on: new ISODate("2024-02-25T20:30:00Z"),
        },
        {
            _id: "TEST#FILE#003",
            name: "Test File 3",
            status: { fail: "Error Message"},
            created_on: new ISODate("2024-02-25T20:30:00Z"),
        },
    ]);
}
