# Project Design Folder

This folder is dedicated to the design of the application. Design plays a crucial role in the application development lifecycle as it lays the foundation for user experience and functionality.

## Penpot Design Tool

Included in this folder is a `docker-compose.yaml` file that allows you to run the [Penpot](https://penpot.app) design tool seamlessly. Penpot is a powerful design application that enables collaborative design work and prototyping.

### Getting Started

To run Penpot using Docker Compose, follow these steps:

1. Ensure you have Docker installed on your system.

2. Open a terminal and navigate to this folder.

3. Run the following command to start Penpot:

```bash
docker-compose -p penpot -f docker-compose.yaml up -d
```

### Accessing Penpot

Once Penpot is up and running, you can access it through the following URL in your web browser: `http://localhost:9001`.

### User Setup

- Create a new user account through Penpot's interface.
- After creating your account, create a new project within Penpot.
- Import the `.penpot` project file included in this folder into your newly created project.


### Shutdown Penpot

You can shut down Penplot using the following command:

```bash
docker-compose -p penpot -f docker-compose.yaml down
```