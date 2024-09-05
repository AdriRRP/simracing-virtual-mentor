# Simracing Virtual Mentor
[![clippy](https://github.com/AdriRRP/simracing-virtual-mentor/actions/workflows/clippy.yaml/badge.svg)](https://github.com/AdriRRP/simracing-virtual-mentor/actions/workflows/clippy.yaml) [![tests](https://github.com/AdriRRP/simracing-virtual-mentor/actions/workflows/tests.yaml/badge.svg)](https://github.com/AdriRRP/simracing-virtual-mentor/actions/workflows/tests.yaml) [![codecov](https://codecov.io/gh/AdriRRP/simracing-virtual-mentor/graph/badge.svg?token=EesiDPcxBF)](https://codecov.io/gh/AdriRRP/simracing-virtual-mentor)

**Virtual Assistant for Sports Driving Simulation: A Comparative Telemetry Approach**

![Screenshot of Simracing Virtual Mentor](./assets/screenshot.png)

## Table of Contents

1. [About the Project](#about-the-project)
2. [Features](#features)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Directory Structure](#directory-structure)
6. [Contributing](#contributing)
7. [License](#license)
8. [Acknowledgements](#acknowledgements)

## About the Project

This repository hosts my final project for the degree in Computer Engineering at the [Escuela Superior de Informática (ESI) of Ciudad Real](https://esi.uclm.es), [Universidad de Castilla la Mancha](https://www.uclm.es), Spain.

The goal of this project is to develop a virtual assistant for iRacing users that enables them to compare their lap performances across various circuits. By leveraging telemetry data, the assistant provides users with natural language feedback and detailed comparative graphs of critical telemetry variables, aiding in lap time improvement.

The project is entirely developed in Rust, covering both frontend and backend, showcasing a comprehensive approach to full-stack development. Additionally, it integrates advanced machine learning models to cluster differences between compared laps, providing insights into driving performance.

The project is organized using Domain-Driven Design (DDD) principles, ensuring a modular and scalable architecture that can be easily maintained and extended.

## Features

- **Telemetry Analysis**: Compare lap performances by analyzing key telemetry data.
- **Natural Language Feedback**: Receive insights and suggestions in easy-to-understand natural language.
- **Graphical Comparisons**: Visualize critical telemetry variables through comparative graphs.
- **Full-Stack Rust Development**: From the backend to the frontend, the project is entirely built in Rust.
- **Advanced Machine Learning**: Utilizes machine learning models to cluster differences between compared laps.

## Installation

### Prerequisites

- [Docker](https://www.docker.com/get-started) installed on your system.
- Basic knowledge of Rust and Docker is recommended.

### Setup

1. **Clone the repository:**
    ```bash
    git clone https://github.com/AdriRRP/simracing-virtual-mentor.git
    cd simracing-virtual-mentor
    ```
2. **Build and run the application using Docker Compose:**
    ```bash
    docker-compose up --build
    ```
   This will set up the MongoDB database and start the backend server.

3. **Access the application:**
   Once the services are running, the backend should be accessible at http://localhost:16666.

## Usage

1. **Uploading Telemetry Files:**
   - Upload your iRacing telemetry files through the frontend interface.

2. **Analyzing Laps:**
   - Select laps to compare. The system will provide feedback and graphical analysis.

3. **Reviewing Results:**
   - View detailed analysis results, including graphs and suggestions, to improve your lap times.

## Directory Structure

The project is organized with a focus on clarity and maintainability, adhering to Domain-Driven Design (DDD) principles and hexagonal architecture. Here is an overview of the directory structure and its purpose:

**Top-Level Directories**

- **`app/`**: Contains all the application code, divided into three main modules: `shared`, `backend`, and `frontend`.

- **`docs/`**: Includes the project report and other relevant documentation. This is essential for both development and the final project presentation.

- **`etc/`**: Stores configuration files and additional scripts. This includes the MongoDB initialization script and other environment-specific settings.

- **`.github/`**: Contains the GitHub Actions workflows for continuous integration and deployment (CI/CD). This ensures that the code is automatically tested, built, and deployed.



## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the project.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a pull request.

## License

Distributed under the GPL-3.0 License. See [LICENSE](LICENSE) for more information.

## Acknowledgements

- [Escuela Superior de Informática (ESI)](https://esi.uclm.es)
- [Universidad de Castilla la Mancha](https://www.uclm.es)
- [Rust Programming Language](https://www.rust-lang.org)
