use crate::sub_chip::SubAirWithInteractions;
use afs_stark_backend::interaction::Chip;
use p3_field::PrimeField64;

use super::IsLessThanTupleChip;

impl<F: PrimeField64> Chip<F> for IsLessThanTupleChip {}

impl<F: PrimeField64> SubAirWithInteractions<F> for IsLessThanTupleChip {}
