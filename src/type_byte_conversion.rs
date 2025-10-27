use std::fmt::Debug;

pub trait ByteConversion:Sized {
	const BYTES_SIZE:usize;

	/// Create the value from bytes.
	fn from_bytes(bytes:Vec<u8>, be:bool) -> Self;

	/// Convert self to bytes.
	fn to_bytes(&self, be:bool) -> Vec<u8>;
}



/* IMPLEMENT NUMBERS */

macro_rules! impl_byteconversion_to_number {
	($type:ty, $size:expr) => {
		impl ByteConversion for $type {
			const BYTES_SIZE:usize = $size;

			/// Create the value from bytes.
			fn from_bytes(bytes:Vec<u8>, be:bool) -> Self {
				if be {
					<$type>::from_be_bytes(bytes.try_into().unwrap())
				} else {
					<$type>::from_le_bytes(bytes.try_into().unwrap())
				}
			}

			/// Convert self to bytes.
			fn to_bytes(&self, be:bool) -> Vec<u8> {
				if be {
					self.to_be_bytes().to_vec()
				} else {
					self.to_le_bytes().to_vec()
				}
			}
		}
	};
}
impl_byteconversion_to_number!(u128, 16);
impl_byteconversion_to_number!(u64, 8);
impl_byteconversion_to_number!(u32, 4);
impl_byteconversion_to_number!(u16, 2);
impl_byteconversion_to_number!(u8, 1);
impl_byteconversion_to_number!(i128, 16);
impl_byteconversion_to_number!(i64, 8);
impl_byteconversion_to_number!(i32, 4);
impl_byteconversion_to_number!(i16, 2);
impl_byteconversion_to_number!(i8, 1);
impl_byteconversion_to_number!(f64, 8);
impl_byteconversion_to_number!(f32, 4);



/* LIST IMPLEMENTATIONS */

impl<T:ByteConversion + Debug, const U:usize> ByteConversion for [T; U] {
	const BYTES_SIZE:usize = T::BYTES_SIZE * U;

	/// Create the value from bytes.
	fn from_bytes(bytes:Vec<u8>, be:bool) -> Self {
		bytes.chunks(T::BYTES_SIZE).map(|bytes| T::from_bytes(bytes.to_vec(), be)).collect::<Vec<T>>().try_into().unwrap()
	}

	/// Convert self to bytes.
	fn to_bytes(&self, be:bool) -> Vec<u8> {
		self.iter().map(|value| value.to_bytes(be)).flatten().collect()
	}
}