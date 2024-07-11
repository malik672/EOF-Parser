//! # EOF Parser

#![warn(
    missing_debug_implementations,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub mod constants;
pub mod error;
pub mod parser;
pub mod prelude;

#[derive(Debug, PartialEq, Clone)]
pub struct EOFContainer {
    pub header: EOFHeader,
    pub body: Body,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EOFHeader {
    pub magic: [u8; 2],
    pub version: u8,
    pub kind_type: u8,
    pub type_size: u16,
    pub kind_code: u8,
    pub num_code_sections: u16,
    pub code_size: u16,
    pub kind_container: u8,
    pub num_container_sections: u16,
    pub container_size: u8,
    pub kind_data: u8,
    pub data_size: u16,
    pub terminator: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    pub types_section: Vec<TypeMetadata>,
    pub inputs: u8,
    pub outputs: u8,
    pub max_stack_height: u16,
    pub code_section: Vec<u8>,
    pub container_section: Vec<u8>,
    pub data_section: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypesSection {
    pub items: Vec<TypeMetadata>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeMetadata {
    pub inputs: u8,
    pub outputs: u8,
    pub max_stack_height: u16,
}
