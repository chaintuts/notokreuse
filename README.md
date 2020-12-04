## General
____________

### Author
* Josh McIntyre

### Website
* jmcintyre.net

### Overview
* NotOkReuse demonstrates the dangers of ECDSA signature nonce (k value) reuse 

## Development
________________

### Git Workflow
* master for releases (merge development)
* development for bugfixes and new features

### Building
* make build
Build the application - wraps `cargo build`
* make clean
Clean the build directory

### Features
* Calculate k and d (private key) given hexadecimal signature and message hash data
* Data is read from a formatted file
* The curve used in this demonstration is secp256k1, the curve used in Bitcoin and other major cryptocurrencies

### Requirements
* Requires Rust language build tools
* Requires a file with hex-formatted (no preceding 0x) data in order s1, s2, r, h1, h2
    * s1 and s2 are signature s points
    * r is the signature r point (will be the same for both sigs if k is reused)
    * h1 and h2 are SHA-256 hashes of the signed messages

### Platforms
* Windows
* MacOSX
* Linux

## Usage
____________

### Command Line Usage
* Put the formatted data file in the same directory as the compiled binary OR in the root directory of the project
* Run `./notok_reuse` or `cargo run`
* The program will output the resulting k and d (private key) as hex (no preceding 0x)