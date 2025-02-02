# Tables
A cli for interacting with SQL databases.


# Next Steps
- [x] Create a test database to test against
- [x] Create and confirm a database connection
- [x] Open a REPL when just given a URL
- [x] Take input from stdin and send to Db
- [x] Take Db response and display in TUI
- [ ] Add using arrow keys to use old inputs
- [ ] Close connection gracefully

# Resources
1. [Sample databases](https://github.com/lerocha/chinook-database)

# Testing
1. Install docker: https://docs.docker.com/engine/install/fedora/
1. Start database: `./scripts/init_db.sh`
1. If database already created, just run the migration: `SKIP_DOCKER=true ./scripts/init_db.sh` 
1. Run the project: `cargo r -- -u postgres://app:secret@localhost:5432/newsletter`

# Install locally
```sh
cargo install --path .
tables --help
```
