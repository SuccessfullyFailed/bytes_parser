#[cfg(test)]
mod test {
	use crate::BytesParser;



	#[test]
	fn bytes_parser_full_read_test() {
		let mut parser:BytesParser = BytesParser::new((0..100).collect(), true);

		assert_eq!(parser.take::<u128>().unwrap(), u128::from_be_bytes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]));
		assert_eq!(parser.take::<i128>().unwrap(), i128::from_be_bytes([16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]));
		assert_eq!(parser.take::<u64>().unwrap(), u64::from_be_bytes([32, 33, 34, 35, 36, 37, 38, 39]));
		assert_eq!(parser.take::<i64>().unwrap(), i64::from_be_bytes([40, 41, 42, 43, 44, 45, 46, 47]));
		assert_eq!(parser.take::<u32>().unwrap(), u32::from_be_bytes([48, 49, 50, 51]));
		assert_eq!(parser.take::<i32>().unwrap(), i32::from_be_bytes([52, 53, 54, 55]));
		assert_eq!(parser.take::<u16>().unwrap(), u16::from_be_bytes([56, 57]));
		assert_eq!(parser.take::<i16>().unwrap(), i16::from_be_bytes([58, 59]));
		assert_eq!(parser.take::<u8>().unwrap(), u8::from_be_bytes([60]));
		assert_eq!(parser.take::<i8>().unwrap(), i8::from_be_bytes([61]));
		assert_eq!(parser.take::<[u8; 10]>().unwrap(), [62, 63, 64, 65, 66, 67, 68, 69, 70, 71]);
		assert_eq!(parser.take_conditional(|next_num| next_num >> 8 == 80_u16).unwrap(), None::<u16>);
		assert_eq!(parser.take_many::<u16>(4).unwrap(), vec![u16::from_be_bytes([72, 73]), u16::from_be_bytes([74, 75]), u16::from_be_bytes([76, 77]), u16::from_be_bytes([78, 79])]);
		assert_eq!(parser.take_conditional(|next_num| next_num >> 8 == 80_u16).unwrap(), Some(u16::from_be_bytes([80, 81])));
		parser.skip(13);
		assert_eq!(parser.take_remaining_bytes(), vec![95, 96, 97, 98, 99]);
		assert!(parser.take::<u8>().is_err());
	}

	#[test]
	fn bytes_parser_full_write_test() {
		let mut parser:BytesParser = BytesParser::new(Vec::new(), false);

		parser.write(u8::from_le_bytes([0]));
		parser.write(i8::from_le_bytes([1]));
		parser.write(u16::from_le_bytes([2, 3]));
		parser.write(i16::from_le_bytes([4, 5]));
		parser.write(u32::from_le_bytes([6, 7, 8, 9]));
		parser.write(i32::from_le_bytes([10, 11, 12, 13]));
		parser.write(u64::from_le_bytes([14, 15, 16, 17, 18, 19, 20, 21]));
		parser.write(i64::from_le_bytes([22, 23, 24, 25, 26, 27, 28, 29]));
		parser.write(u128::from_le_bytes([30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45]));
		parser.write(i128::from_le_bytes([46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61]));
		parser.skip(4);
		parser.write::<[u8; 4]>([66, 67, 68, 69]);
		parser.write_many::<u16>(vec![u16::from_le_bytes([70, 71]), u16::from_le_bytes([72, 73])]);

		assert_eq!(parser.raw_data(), (0..74).map(|index| if index > 61 && index < 66 { 0 } else { index }).collect::<Vec<u8>>());
	}
}