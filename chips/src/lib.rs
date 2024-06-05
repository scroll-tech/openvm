pub mod page1_read;
pub mod page2_controller;
pub mod page2_read;
pub mod page2_requester;
// pub mod page_btree;
pub mod page_controller;
pub mod page_node;
pub mod page_read;
pub mod pagebtree;
/// Chip to range check a value has less than a fixed number of bits
pub mod range;
pub mod range_gate;
pub mod xor_bits;
pub mod xor_limbs;
pub mod xor_lookup;

pub const MAX_COMMITMENT_LEN: usize = 8;
