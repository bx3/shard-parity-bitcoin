
//! Bitcoin trainsaction.
//! https://en.bitcoin.it/wiki/Protocol_documentation#tx

use hex::FromHex;
use primitives::Bytes;
use ser::{
	Deserializable, Reader, Error as ReaderError, deserialize,
	Serializable, Stream, serialize
};
use crypto::dhash256;
use hash::H256;

// Below flags apply in the context of BIP 68
// If this flag set, CTxIn::nSequence is NOT interpreted as a
// relative lock-time.
pub const SEQUENCE_LOCKTIME_DISABLE_FLAG: u32 = 1u32 << 31;

// Setting nSequence to this value for every input in a transaction
// disables nLockTime.
pub const SEQUENCE_FINAL: u32 = 0xffffffff;

// If CTxIn::nSequence encodes a relative lock-time and this flag
// is set, the relative lock-time has units of 512 seconds,
// otherwise it specifies blocks with a granularity of 1.
pub const SEQUENCE_LOCKTIME_TYPE_FLAG: u32 = (1 << 22);

// If CTxIn::nSequence encodes a relative lock-time, this mask is
// applied to extract that lock-time from the sequence field.
pub const SEQUENCE_LOCKTIME_MASK: u32 = 0x0000ffff;


#[derive(Debug, Clone)]
pub struct OutPoint {
	pub hash: H256,
	pub index: u32,
}

impl Serializable for OutPoint {
	fn serialize(&self, stream: &mut Stream) {
		stream
			.append_slice(&self.hash)
			.append(&self.index);
	}
}

impl Deserializable for OutPoint {
	fn deserialize(reader: &mut Reader) -> Result<Self, ReaderError> where Self: Sized {
		let mut hash = [0u8; 32];
		hash.copy_from_slice(try!(reader.read_slice(32)));
		let index = try!(reader.read());
		let result = OutPoint {
			hash: hash,
			index: index,
		};

		Ok(result)
	}
}

impl OutPoint {
	pub fn hash(&self) -> &H256 {
		&self.hash
	}

	pub fn index(&self) -> u32 {
		self.index
	}
}

#[derive(Debug)]
pub struct TransactionInput {
	pub previous_output: OutPoint,
	pub script_sig: Bytes,
	pub sequence: u32,
}

impl Serializable for TransactionInput {
	fn serialize(&self, stream: &mut Stream) {
		stream
			.append(&self.previous_output)
			.append(&self.script_sig)
			.append(&self.sequence);
	}
}

impl Deserializable for TransactionInput {
	fn deserialize(reader: &mut Reader) -> Result<Self, ReaderError> where Self: Sized {
		let previous_output = try!(reader.read());
		let script_sig = try!(reader.read());
		let sequence = try!(reader.read());

		let result = TransactionInput {
			previous_output: previous_output,
			script_sig: script_sig,
			sequence: sequence,
		};

		Ok(result)
	}
}

impl TransactionInput {
	pub fn previous_output(&self) -> &OutPoint {
		&self.previous_output
	}

	pub fn script_sig(&self) -> &[u8] {
		&self.script_sig
	}

	pub fn sequence(&self) -> u32 {
		self.sequence
	}
}

#[derive(Debug, Clone)]
pub struct TransactionOutput {
	pub value: u64,
	pub script_pubkey: Bytes,
}

impl Serializable for TransactionOutput {
	fn serialize(&self, stream: &mut Stream) {
		stream
			.append(&self.value)
			.append(&self.script_pubkey);
	}
}

impl Deserializable for TransactionOutput {
	fn deserialize(reader: &mut Reader) -> Result<Self, ReaderError> where Self: Sized {
		let value = try!(reader.read());
		let script_pubkey = try!(reader.read());

		let result = TransactionOutput {
			value: value,
			script_pubkey: script_pubkey,
		};

		Ok(result)
	}
}

impl Default for TransactionOutput {
	fn default() -> Self {
		TransactionOutput {
			value: 0xffffffffffffffffu64,
			script_pubkey: Bytes::default(),
		}
	}
}

impl TransactionOutput {
	pub fn value(&self) -> u64 {
		self.value
	}

	pub fn script_pubkey(&self) -> &[u8] {
		&self.script_pubkey
	}
}

#[derive(Debug)]
pub struct Transaction {
	pub version: i32,
	pub inputs: Vec<TransactionInput>,
	pub outputs: Vec<TransactionOutput>,
	pub lock_time: u32,
}

impl Serializable for Transaction {
	fn serialize(&self, stream: &mut Stream) {
		stream
			.append(&self.version)
			.append_list(&self.inputs)
			.append_list(&self.outputs)
			.append(&self.lock_time);
	}
}

impl Deserializable for Transaction {
	fn deserialize(reader: &mut Reader) -> Result<Self, ReaderError> where Self: Sized {
		let version = try!(reader.read());
		let tx_inputs = try!(reader.read_list());
		let tx_outputs = try!(reader.read_list());
		let lock_time = try!(reader.read());

		let result = Transaction {
			version: version,
			inputs: tx_inputs,
			outputs: tx_outputs,
			lock_time: lock_time,
		};

		Ok(result)
	}
}

impl From<&'static str> for Transaction {
	fn from(s: &'static str) -> Self {
		deserialize(&s.from_hex().unwrap()).unwrap()
	}
}

impl Transaction {
	pub fn hash(&self) -> H256 {
		dhash256(&serialize(self))
	}

	pub fn inputs(&self) -> &[TransactionInput] {
		&self.inputs
	}

	pub fn outputs(&self) -> &[TransactionOutput] {
		&self.outputs
	}
}

#[cfg(test)]
mod tests {
	use hex::FromHex;
	use ser::deserialize;
	use hash::h256_from_str;
	use super::Transaction;

	// real transaction from block 80000
	// https://blockchain.info/rawtx/5a4ebf66822b0b2d56bd9dc64ece0bc38ee7844a23ff1d7320a88c5fdb2ad3e2
	// https://blockchain.info/rawtx/5a4ebf66822b0b2d56bd9dc64ece0bc38ee7844a23ff1d7320a88c5fdb2ad3e2?format=hex
	#[test]
	fn test_transaction_reader() {
		let encoded_tx = "0100000001a6b97044d03da79c005b20ea9c0e1a6d9dc12d9f7b91a5911c9030a439eed8f5000000004948304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501ffffffff0100f2052a010000001976a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac00000000".from_hex().unwrap();
		let t: Transaction = deserialize(&encoded_tx).unwrap();
		assert_eq!(t.version, 1);
		assert_eq!(t.lock_time, 0);
		assert_eq!(t.inputs.len(), 1);
		assert_eq!(t.outputs.len(), 1);
		let tx_input = &t.inputs[0];
		assert_eq!(tx_input.sequence, 4294967295);
		assert_eq!(tx_input.script_sig, "48304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501".into());
		let tx_output = &t.outputs[0];
		assert_eq!(tx_output.value, 5000000000);
		assert_eq!(tx_output.script_pubkey, "76a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac".into());
	}

	#[test]
	fn test_transaction_hash() {
		let encoded_tx = "0100000001a6b97044d03da79c005b20ea9c0e1a6d9dc12d9f7b91a5911c9030a439eed8f5000000004948304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501ffffffff0100f2052a010000001976a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac00000000".from_hex().unwrap();
		let hash = h256_from_str("5a4ebf66822b0b2d56bd9dc64ece0bc38ee7844a23ff1d7320a88c5fdb2ad3e2");
		let t: Transaction = deserialize(&encoded_tx).unwrap();
		assert_eq!(t.hash(), hash);
	}
}