# OfficeSpaceSplit

## Description

OfficeSpaceSplit is a program that can be used to simulate the attribution of offices to office space seekers. Matching can be run in two modes: allowing splitting an office between multiple seekers or not. To find the matches, a greedy algorithm is used, see below for more details. Given the nature of the algorithm, the found matching might not be optimal.

## Installation

To download the program, you can follow the following steps:

1. Clone the repository

```bash
git clone https://github.com/pavelkurach/office-space-split.github
```

2. Step into the project directory

```bash
cd office-space-split
```

To run the program, you can use one of the following methods:


### With Docker

For this method, you will need to have Docker installed on your machine. If you don't have it, you can download it [here](https://www.docker.com/products/docker-desktop). Follow the official instruction to install and run Docker on your machine.

3. Build the Docker image

```bash
docker build -t office-space-split .
```

4. Run the Docker container

```bash
docker run office-space-split
```

### Natively

For this method, you will need Rust toolchain to be installed. If you don't have it, you can download it [here](https://www.rust-lang.org/tools/install). Follow the official instruction to install Rust on your machine.

3. Run the program

```bash
cargo run --release
```

## Usage

The program is based on two base objects: rental spaces and users. Rental spaces correspond to physical offices, and users correspond to people owning rental spaces (hosts) or looking for rental spaces (guests). The mathing algoithm will generate a list of rental contracts between hosts and guests. Matching is done over two years, and every contract assumes a one year duration, so that guests that could not find a rental space in the first year might be able to find one in the second year.

Once started, the program will prompt you to choose one of the following commands:

- `add`: Add a new rental space or user. All objects must be provided in JSON format, see examples below.
- `print`: Print all rental spaces and users. You will be promted to choose a category to print.
- `match`: Match users with rental spaces. You will be prompted to choose if splitting is allowed.
- `add sample data`: Add sample rental spaces and users to illustrate the difference between the two matching modes.
- `exit`: Exit the program.

## Object model

### Rental spaces

Rental spaces are represented by the following fields:

- `id`: A unique identifier for the rental space. Generated automatically.
- `created_at`: The date at which the rental space was created. Generated automatically.
- `name`: The name of the rental space.
- `address`: The address of the rental space.
- `surface`: The surface of the rental space in square meters.
- `nb_workstations`: The number of workstations in the rental space.
- `price_per_workstation`: The price per workstation in the rental space per year.
- `owner_id`: The id of the owner of the rental space.

Rental space must respect the following constraints:

- `nb_workstations` must be between 40 and 180.
- `price_per_workstation` must be between 300 and 800.
- Rental space cannot have more than 5 workstations per 8m² if there are less than 60 workstations and more 5 workstations per 7m² if there are more than 60 workstations.

Example of request to add a rental space:

```json 
{
    "name": "Office 101",
    "address": "123 Main St, Cityville, Country",
    "surface": 120,
    "nb_workstations": 50,
    "price_per_workstation": 400,
    "owner_id": "usr-22795DC7-E972-44D7-A74B-553EA6589044"
}
```

### Users

Users are represented by the following fields:

- `id`: A unique identifier for the user. Generated automatically. 
- `created_at`: The date at which the user was created. Generated automatically.
- `first_name`: The first name of the user.
- `last_name`: The last name of the user.
- `workspace_request`: Optional, consists of the following fields:
    - `nb_workstations`: The number of workstations the user is looking for.
    - `budget`: The maximum budget the user is willing to pay per year.

Example of request to add a user:

```json
{
    "first_name": "John",
    "last_name": "Doe",
    "workspace_request": {
        "nb_workstations": 10,
        "budget": 500
    }
}
```

### Split

The split object is used to represent the area that can be rented in a given rental space. If splitting is not allowed, there is only one split per rental space, equivalent to the whole rental space. If splitting is allowed, the rental space can be split into multiple splits, each corresponding to a part of the rental space.

The split object must respect the same constraints as the rental space object.

### Contracts

The contract object is used to represent the rental contract between a host and a guest in a rental space owned by the host. Each contract assumes a one year duration. The contract object consists of the following fields:

- `id`: A unique identifier for the contract. Generated automatically.
- `created_at`: The date at which the contract was created. Generated automatically.
- `rental_space_id`: The id of the rental space rented.
- `host_id`: The id of the host of the rental space.
- `guest_id`: The id of the guest renting the rental space.
- `nb_workstations`: The number of workstations rented.
- `price`: Total price paid by the guest over the year.

If `nb_workstations` is less than the number of workstations in the rental space, the rental space has been split.

## Matching algorithm

The matching algorithm is a greedy algorithm. For every user, it tries to select the best match from available splits that correspond to user's budget and required number of workstations. The rules for selecting the best match are described below.

All generated contracts have one year duration. The algorithm will try to match as many users as possible in the first year, and try to match the remaining users in the second year.

### Without subsplitting

The best candidate is the smallest rental space respecting the criteria above.

### With subsplitting

At first, the best match is selected from rental spaces that cannot be split due to split constraits. If no such match is found, the algorithm will consider rental spaces that can be split into two splits such that one of them has the number of workspaces higher or equal to the number of workspaces requested by the user and corresponds to user's budget.
