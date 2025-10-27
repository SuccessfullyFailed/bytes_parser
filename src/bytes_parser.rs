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



	/* CURSOR METHODS */

	/// Get the current position of the cursor.
	pub fn cursor(&self) -> usize {
		self.cursor
	}

	/// Skip an amount of bytes.
	pub fn skip(&mut self, bytes:usize) {
		self.cursor += bytes;
	}



	/* FILE METHODS */

	/// Create a new parser from reading a file.
	pub fn from_file(path:&str, big_endian:bool) -> Result<BytesParser, Box<dyn Error>> {
		Ok(BytesParser::new(fs::read(path)?, big_endian))
	}

	/// Save the current bytes to a file.
	pub fn to_file(&self, path:&str) -> Result<(), Box<dyn Error>> {
		Ok(fs::write(path, &self.bytes)?)
	}



	/* DATA READING METHODS METHODS */

	/// Get all raw data.
	#[cfg(test)]
	pub(crate) fn raw_data(&self) -> &[u8] {
		&self.bytes
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
	pub fn take_conditional<T:ByteConversion, U:Fn(T) -> bool>(&mut self, condition:U) -> Result<Option<T>, Box<dyn Error>>  {
		Ok(
			if self.cursor + T::BYTES_SIZE < self.bytes_size && condition(T::from_bytes(self.bytes[self.cursor..self.cursor + T::BYTES_SIZE].to_vec(), self.big_endian)) {
				Some(self.take()?)
			} else {
				None
			}
		)
	}

	/// Take bytes while their first bit is 0. Returns a u64 containing the found bytes. Increments the cursor.
	pub fn take_int_variable_length(&mut self) -> u64 {
		let mut time_bytes:Vec<u8> = self.take_bytes_variable_length();
		time_bytes = [vec![0; 8 - time_bytes.len()], time_bytes].into_iter().flatten().collect();
		u64::from_be_bytes(time_bytes.try_into().unwrap())
	}



	/* DATA WRITING METHODS */

	/// Take the given amount of bytes. Increments the cursor.
	pub fn write_bytes(&mut self, additional_bytes:Vec<u8>) {
		let additional_bytes_len:usize = additional_bytes.len();

		// If cursor is ahead of last byte, insert zero-bytes to bridge the gap.
		if self.cursor > self.bytes_size {
			let gap_size:usize = self.cursor - self.bytes_size;
			self.bytes.extend(vec![0; gap_size]);
			self.bytes_size += gap_size;
		}

		// If the cursor is at the end of the data, add to the end.
		if self.cursor == self.bytes_size {
			self.bytes.extend(additional_bytes);
			self.cursor += additional_bytes_len;
			self.bytes_size += additional_bytes_len;
		}
		
		// If the cursor is inside of the data, either replace or add new bytes depending on each byte.
		else if self.cursor < self.bytes_size {
			for byte_index in 0..additional_bytes_len {
				if self.cursor < self.bytes_size {
					self.bytes[self.cursor] = additional_bytes[byte_index];
				} else {
					self.bytes.push(additional_bytes[byte_index]);
					self.bytes_size += 1;
				}
				self.cursor += 1;
			}
		}
	}

	/// Take bytes while their first bit is 0. Increments the cursor.
	pub fn write_bytes_variable_length<T:ByteConversion>(&mut self, value:T) {
		let mut value_bytes:Vec<u8> = value.to_bytes(self.big_endian);
		if value_bytes.is_empty() {
			value_bytes = vec![0];
		}
		let bytes_count:usize = value_bytes.len();
		value_bytes[..bytes_count - 1].iter_mut().for_each(|byte| *byte |= 0b10000000);
		value_bytes[bytes_count] &= 0b01111111;
		self.write_bytes(value_bytes);
	}

	/// Try to take a specific datatype from the bytes. Increments the cursor.
	pub fn write<T:ByteConversion>(&mut self, value:T) {
		self.write_bytes(value.to_bytes(self.big_endian));
	}
}