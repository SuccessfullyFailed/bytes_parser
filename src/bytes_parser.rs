use std::{ fs, error::Error };

use crate::ByteConversion;



pub struct BytesParser {
	bytes:Vec<u8>,
	bytes_size:usize,
	big_endian:bool,

	cursor:usize
}
impl BytesParser {

	/* CONSTRUCTOR METHODS */

	/// Create a parser from a list of bytes.
	pub fn new(bytes:Vec<u8>, big_endian:bool) -> BytesParser {
		let bytes_size:usize = bytes.len();
		BytesParser {
			bytes,
			bytes_size,
			big_endian,

			cursor: 0
		}
	}

	/// Create a new parser from reading a file.
	pub fn from_file(path:&str, big_endian:bool) -> Result<BytesParser, Box<dyn Error>> {
		Ok(BytesParser::new(fs::read(path)?, big_endian))
	}



	/* DATA READING METHODS METHODS */

	/// Returns true if there is no more data to read.
	pub fn is_empty(&self) -> bool {
		self.cursor >= self.bytes_size
	}

	/// Skip an amount of bytes.
	pub fn skip(&mut self, bytes:usize) {
		self.cursor += bytes;
	}

	/// Take the remaining bytes. Increments the cursor.
	pub fn take_remaining_bytes(&mut self) -> Vec<u8> {
		let remaining_bytes:usize = self.bytes_size - self.cursor;
		self.cursor += remaining_bytes;
		self.bytes[self.cursor - remaining_bytes..].to_vec()
	}

	/// Take the given amount of bytes. Increments the cursor.
	pub fn take_bytes(&mut self, bytes:usize) -> Result<Vec<u8>, Box<dyn Error>> {
		self.cursor += bytes;
		if self.cursor <= self.bytes_size {
			Ok(self.bytes[self.cursor - bytes..self.cursor].try_into()?)
		} else {
			Err("Ran out of bytes to read.".into())
		}
	}

	/// Take the set amount of bytes if the condition is met. Condition function only gets the amount of bytes requested to take. Returns bytes and increments the cursor only if condition is met.
	pub fn take_bytes_conditional<T:Fn(&[u8]) -> bool>(&mut self, bytes:usize, condition:T) -> Result<Option<Vec<u8>>, Box<dyn Error>>  {
		Ok(
			if self.cursor + bytes < self.bytes_size && condition(&self.bytes[self.cursor..self.cursor + bytes]) {
				Some(self.take_bytes(bytes)?)
			} else {
				None
			}
		)	
	}

	/// Take bytes while their first bit is 0. Increments the cursor.
	pub fn take_bytes_variable_length(&mut self) -> Vec<u8> {
		let mut more_bytes_to_follow:bool = true;
		let mut bytes:Vec<u8> = Vec::new();
		while more_bytes_to_follow {
			if let Ok(byte) = self.take::<u8>() {
				bytes.push(byte & 0b01111111);
				more_bytes_to_follow = byte >> 7 == 1;
			} else {
				more_bytes_to_follow = false;
			}
		}
		bytes
	}

	/// Try to take a specific datatype from the bytes. Increments the cursor.
	pub fn take<T:ByteConversion>(&mut self) -> Result<T, Box<dyn Error>> {
		Ok(T::from_bytes(self.take_bytes(T::BYTES_SIZE)?, self.big_endian))
	}

	/// Take the set amount of bytes if the condition is met. Condition function only gets the amount of bytes requested to take. Returns bytes and increments the cursor only if condition is met.
	pub fn take_conditional<T:ByteConversion, U:Fn(&[u8]) -> bool>(&mut self, condition:U) -> Result<Option<T>, Box<dyn Error>>  {
		match self.take_bytes_conditional(T::BYTES_SIZE, condition)? {
			Some(bytes) => Ok(Some(T::from_bytes(bytes, self.big_endian))),
			None => Ok(None)
		}
	}
}