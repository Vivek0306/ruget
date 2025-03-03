
# ruget: wget clone but in Rust


A simple CLI tool written in Rust that emulates the `wget` CLI utility for downloading files from the web using HTTP, HTTPS, and FTP.


## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) must be installed on your machine. If you don't have Rust installed, follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).


This will download and compile the project, making the `ruget` command available on your system.

### Manual Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Vivek0306/ruget.git
   ```

2. Navigate to the project directory:

   ```bash
   cd ruget
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

4. You can now run `ruget` using the path to the binary, or move it to a directory in your `PATH`.

## Usage

### Basic Usage

To download a file using `ruget`, use the following command:

```bash
ruget --url <URL>
```

For example, to download a file from `https://www.example.com`:

```bash
ruget --url https://www.example.com
```

By default, if no output filename is provided, `ruget` will save the file as `index.html` for domains (like `https://www.example.com/`) or use the filename extracted from the URL.

### Custom Output Filename

You can specify a custom filename using the `--output` (or `-o`) flag:

```bash
ruget --url <URL> --output <filename>
```

For example, to download `https://www.example.com/example.txt` and save it as `myfile.txt`:

```bash
ruget --url https://www.example.com/example.txt --output myfile.txt
```

### Progress Bar

`ruget` displays a progress bar showing the download status, including the number of bytes downloaded and the estimated time remaining.

### Quiet Mode

If you prefer to run the download silently (without showing the progress bar), you can enable quiet mode by passing the `--quiet` flag:

```bash
ruget --url <URL> --quiet
```

### Example

1. **Basic download:**

   ```bash
   ruget --url https://www.example.com
   ```

   This will download `https://www.example.com` and save it as `index.html`.

2. **Custom filename:**

   ```bash
   ruget --url https://www.example.com/example.txt --output myfile.txt
   ```

   This will download the file from `https://www.example.com/example.txt` and save it as `myfile.txt`.

3. **Download in quiet mode:**

   ```bash
   ruget --url https://www.example.com --quiet
   ```

   This will download `https://www.example.com` without displaying the progress bar.

## Contributing

We welcome contributions to the project! To contribute:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/feature-name`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature/feature-name`).
5. Open a pull request.

Please make sure your code follows the Rust style guidelines and includes tests where appropriate.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- This project is inspired by the functionality of the `wget` utility, implemented in Rust for better performance and safety.
