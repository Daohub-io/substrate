#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Capabilities {
	/// Does the contract have the capability to write to storage?
	pub write: bool,
	/// Does the contract have the capability to log events?
	pub log: bool,
}

impl Capabilities {

	/// Create a capability set that has no capabilities.
	pub fn none() -> Self {
		Capabilities {
			write: false,
			log: false,
		}
	}

	/// Create a capability set that has all capabilities.
	pub fn all() -> Self {
		Capabilities {
			write: true,
			log: true,
		}
	}
}

impl Default for Capabilities {
	fn default() -> Self {
		Self::none()
	}
}

impl codec::Encode for Capabilities {
	fn encode_to<T: codec::Output>(&self, dest: &mut T) {
		dest.push_byte(if self.write { 1_u8 } else { 0_u8 });
		dest.push_byte(if self.log { 1_u8 } else { 0_u8 });
	}
}

impl codec::EncodeLike for Capabilities {}

impl codec::Decode for Capabilities {
	// TODO: we can make this much more tightly packed
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		let write = match input.read_byte()? {
			1_u8 => Ok(true),
			0_u8 => Ok(false),
			_ => Err(codec::Error::from("Invalid bytes")),
		}?;
		let log = match input.read_byte()? {
			1_u8 => Ok(true),
			0_u8 => Ok(false),
			_ => Err(codec::Error::from("Invalid bytes")),
		}?;
		Ok(Capabilities {
			write,
			log,
		})
	}
}
