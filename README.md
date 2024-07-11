# EOF Parser

EOF Parser is a Rust library for parsing Ethereum Object Format (EOF) files. EOF is a new binary format for smart contracts on the Ethereum blockchain, designed to improve efficiency and introduce new features.

## Features

- Parse EOF containers from binary data
- Validate EOF headers and structure
- Extract code, data, and type sections
- Support for EOF version 1

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
eof-parser = "0.1.0"
```

Basic usage example:
  

    use eof_parser::Parser;
	use std::fs::File;

	fn main() -> Result<(), Box<dyn std::error::Error>> {   
    let file = File::open("path/to/your/eof/file")?;
   
    let mut parser = Parser::new(file);
    
    match parser.parse() {
        Ok(container) => {
            println!("EOF Container parsed successfully:");
            println!("Header: {:?}", container.header);
            println!("Code section size: {}", container.body.code_section.len());
            // Access other parsed data as needed
        },
        Err(e) => println!("Error parsing EOF: {:?}", e),
    }

    Ok(())
}

  
## Structure

The library consists of the following main components:

-   `Parser`: The main struct for parsing EOF files
-   `EOFContainer`: Represents a parsed EOF container
-   `EOFHeader`: Contains the parsed header information
-   `Body`: Represents the parsed body of the EOF container
-   `TypesSection` and `TypeMetadata`: Represent the types section of the EOF

## Error Handling

The library uses a custom `Error` enum to handle various parsing errors, such as invalid magic numbers, unsupported versions, or malformed sections.

## Testing

The library includes unit tests to ensure correct parsing of valid EOF files and proper error handling for invalid inputs. Run the tests using:

Copy

`cargo test`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).

