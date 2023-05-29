# PlantBuddy-Client

Plantbuddy is a desktop application for managing plants. It allows users to view and edit plant data, manage users, and customize settings. The application is built using the Rust programming language and the Iced GUI library.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/)

### Installation

1. Clone the repo

```https
git clone https://github.com/plantineers/client.git
```
2. Build the project

```bash
cargo build
```

3. Run the project

```bash
cargo run
```

## Usage
Once you start the application, you can navigate through the application using the tab bar at the top/bottom (depending on your settings). Here are some things you can do:

* Home Page: Provides a brief overview of your plants
* Detail Page: Allows you to view detailed information about a particular plant
* Settings Page: Lets you customize application settings
* Login/Logout Page: Allows you to log in or out
* Management Page: Allows you to manage users (Admin only)

## Testing
To run the tests:

```bash
cargo test
```