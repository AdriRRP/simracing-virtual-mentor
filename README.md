# Simracing Virtual Mentor
[![clippy](https://github.com/AdriRRP/simracing-virtual-mentor/actions/workflows/clippy.yaml/badge.svg)](https://github.com/AdriRRP/simracing-virtual-mentor/actions/workflows/clippy.yaml) [![tests](https://github.com/AdriRRP/simracing-virtual-mentor/actions/workflows/tests.yaml/badge.svg)](https://github.com/AdriRRP/simracing-virtual-mentor/actions/workflows/tests.yaml)[![codecov](https://codecov.io/gh/AdriRRP/simracing-virtual-mentor/graph/badge.svg?token=EesiDPcxBF)](https://codecov.io/gh/AdriRRP/simracing-virtual-mentor)

Virtual Assistant for Sports Driving Simulation: A Comparative Telemetry Approach

## About the project

This repository hosts my final project for the degree in computer engineering at the [Escuela Superior de Informática (ESI) of Ciudad Real](https://esi.uclm.es), [Universidad de Castilla la Mancha](https://www.uclm.es), Spain.

Simulators have undergone a significant evolution from being simplified models of reality to complex systems that accurately reflect the systems they simulate.
This ability to mimic almost perfectly the simulated systems makes these simulators effective tools for learning and training.

One area where the popularity of these simulators has grown significantly is in sports driving, also known as simracing.
In these systems, the driver uses a set of hardware that simulates the vehicle, the track and the physical laws that govern the interactions, allowing training sessions to be carried out at a very low cost.

The training and learning process can be autonomous, where the driver observes the different telemetries provided by the simulator and tries to improve them, or can be assisted by a trainer who provides the necessary knowledge and experience to indicate the improvements to be made.

This work is placed in this context with the objective of defining and building a virtual simracing assistant or trainer. This assistant will be able to interpret in a comparative way the telemetry of an expert versus that of the driver in training, providing training plans and providing the driver with the necessary knowledge and experience to improve his performance.

### Objective

Construction of an intelligent system capable of indicating the necessary improvements, actions to be taken, so that a trainee's skills are improved by those of an expert.

### Sub-objectives

To develop this system the sub-objectives will be:

1. Manage and interpret telemetry files.
2. Set metrics between different telemetry files.
3. Set the set of actions to minimize distances between metrics files.
4. Build the set of suggestions with a natural language response.
5. Develop a web application where all the actions are performed and the results are graphically displayed.
