pub use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Parser<R: Read> {
    reader: R,
}

impl<R: Read> Parser<R> {
    pub fn new(reader: R) -> Self {
        Parser { reader }
    }

    pub fn parse(&mut self) -> Result<EOFContainer, Error> {
        let header = self.parse_header()?;
        let body = self.parse_body(&header)?;

        Ok(EOFContainer { header, body })
    }

    fn parse_header(&mut self) -> Result<EOFHeader, Error> {
        let mut magic = [0u8; 2];
        self.reader.read_exact(&mut magic)?;
        if magic != EOF_MAGIC {
            return Err(Error::InvalidMagic);
        }

        let version = self.reader.read_u8()?;
        if version != 1 {
            return Err(Error::InvalidVersion);
        }

        let kind_type = self.reader.read_u8()?;
        let type_size = self.reader.read_u16::<LittleEndian>()?;
        let kind_code = self.reader.read_u8()?;
        let num_code_sections = self.reader.read_u16::<LittleEndian>()?;

        if num_code_sections == 0 || num_code_sections as usize > MAX_CODE_SECTIONS {
            return Err(Error::InvalidCodeSectionCount);
        }

        let code_size = self.reader.read_u16::<LittleEndian>()?;
        let kind_container = self.reader.read_u8()?;
        let num_container_sections = self.reader.read_u16::<LittleEndian>()?;
        let container_size = self.reader.read_u8()?;
        let kind_data = self.reader.read_u8()?;
        let data_size = self.reader.read_u16::<LittleEndian>()?;
        let terminator = self.reader.read_u8()?;

        Ok(EOFHeader {
            magic,
            version,
            kind_type,
            type_size,
            kind_code,
            num_code_sections,
            code_size,
            kind_container,
            num_container_sections,
            container_size,
            kind_data,
            data_size,
            terminator,
        })
    }

    fn parse_body(&mut self, header: &EOFHeader) -> Result<Body, Error> {
        let types_section = self.parse_types_section(header)?;
        let code_section = self.read_section(header.code_size as usize)?;
        let container_section = self.read_section(header.container_size as usize)?;
        let data_section = self.read_section(header.data_size as usize)?;

        Ok(Body {
            types_section: types_section.clone().items,
            inputs: types_section.items[0].inputs,
            outputs: types_section.items[0].outputs,
            max_stack_height: types_section.items[0].max_stack_height,
            code_section,
            container_section,
            data_section,
        })
    }

    fn parse_types_section(&mut self, header: &EOFHeader) -> Result<TypesSection, Error> {
        let mut types = Vec::new();
        let mut remaining_size = header.type_size as usize;

        while remaining_size > 0 {
            if remaining_size < 4 {
                return Err(Error::InvalidTypeSectionSize);
            }

            let inputs = self.reader.read_u8()?;
            let outputs = self.reader.read_u8()?;
            let max_stack_height = self.reader.read_u16::<LittleEndian>()?;

            types.push(TypeMetadata {
                inputs,
                outputs,
                max_stack_height,
            });

            remaining_size -= 4;
        }

        if types.is_empty() {
            return Err(Error::InvalidZeroSectionMetadata);
        }

        Ok(TypesSection { items: types })
    }

    fn read_section(&mut self, size: usize) -> Result<Vec<u8>, Error> {
        let mut buffer = vec![0u8; size];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_eof() {
        // Create a mock EOF container
        let mock_eof = vec![
            0xEF, 0x00, // magic
            0x01, // version
            0x01, // kind_type
            0x04, 0x00, // type_size
            0x02, // kind_code
            0x01, 0x00, // num_code_sections
            0x0A, 0x00, // code_size
            0x03, // kind_container
            0x00, 0x00, // num_container_sections
            0x00, // container_size
            0x04, // kind_data
            0x00, 0x00, // data_size
            0x00, // terminator
            // Types section
            0x00, 0x01, 0x02, 0x00, // Code section
            0x60, 0x00, 0x60, 0x00, 0x60, 0x00, 0x60, 0x00, 0x60, 0x00,
        ];

        let mut parser = Parser::new(Cursor::new(mock_eof));
        let result = parser.parse();
        assert!(result.is_ok());

        let container = result.unwrap();
        assert_eq!(container.header.magic, [0xEF, 0x00]);
        assert_eq!(container.header.version, 1);
        assert_eq!(container.body.code_section.len(), 10);
    }

    #[test]
    fn test_parse_invalid_magic() {
        let invalid_eof = vec![0x00, 0x00]; // Invalid magic number
        let mut parser = Parser::new(Cursor::new(invalid_eof));
        let result = parser.parse();
        assert!(matches!(result, Err(Error::InvalidMagic)));
    }
}
