# IMAPLE: IMAP Service based on RFC-3501

![](img/imaple.png)

IMAPLE is an IMAP service based on RFC-3501. IMAPLE is currently in heavy development and is not production ready. Please use with caution as breaking changes may occur.

## Features

- IMAPv4rev1 support: IMAPLE adheres to the IMAPv4rev1 protocol as defined in RFC-3501.
- Performance and Throughput: IMAPLE leverages the Rust programming language and the tokio library for high performance I/O.
- TLS Support: IMAPLE supports secure communications through rustls.

## Usage

As IMAPLE is still in development, it is recommended to follow the instructions below to run the application:

1. Clone the IMAPLE repository.
2. Install Rust and ensure Cargo is available.
3. Run `cargo build` to build the project.
4. Run `cargo run -- --imap` to start the program.
5. Access the IMAPLE service using an IMAP client, or you can use `test_fetch.py` for testing:
   ```
   python test_fetch.py
   ```

## Contributing

Contributions to IMAPLE are welcomed! If you would like to contribute, please follow the guidelines below:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes.
4. Run the tests using `cargo test` to ensure everything is working as expected.
5. Type `makef fmt` to format the code.
5. Submit a pull request with your changes.

## License

IMAPLE is open source and licensed under the [MIT License](https://opensource.org/licenses/MIT). Please see the [LICENSE](LICENSE) file for more details.

## Acknowledgements

IMAPLE is built upon the efforts of the Rust programming language community and the contributors of the tokio and rustls libraries. We would like to express our gratitude to everyone involved.

[] Robin Syihab
