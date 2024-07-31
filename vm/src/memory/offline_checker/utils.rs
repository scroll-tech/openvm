use std::iter;

use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use p3_matrix::dense::RowMajorMatrix;

use crate::memory::MemoryAccess;

pub fn gen_dummy_oc_interaction_trace<const WORD_SIZE: usize>(
    ops: &mut [MemoryAccess<WORD_SIZE, BabyBear>],
    width: usize,
) -> RowMajorMatrix<BabyBear> {
    ops.sort_by_key(|op| (op.address_space, op.address, op.timestamp));
    let mut rows = vec![];
    for i in 0..ops.len() {
        let is_first_access = (i == 0)
            || (ops[i].address_space != ops[i - 1].address_space
                && ops[i].address != ops[i - 1].address);
        let is_first_access = BabyBear::from_bool(is_first_access);

        let is_final_access = (i == ops.len() - 1)
            || (ops[i].address_space != ops[i + 1].address_space)
            || (ops[i].address != ops[i + 1].address);
        let is_final_access = BabyBear::from_bool(is_final_access);

        let is_first_read = is_first_access * BabyBear::from_canonical_u8(1 - ops[i].op_type as u8);
        let mut rec_fields = vec![is_first_read * BabyBear::neg_one(), BabyBear::one()];
        rec_fields.extend(vec![ops[i].address_space, ops[i].address]);
        rec_fields.extend(ops[i].data);

        let mut send_fields = vec![is_final_access, BabyBear::neg_one()];
        send_fields.extend(vec![ops[i].address_space, ops[i].address]);
        send_fields.extend(ops[i].data);

        rows.extend(rec_fields);
        rows.extend(send_fields);
    }

    rows.extend(
        iter::repeat_with(|| iter::repeat(BabyBear::zero()).take(width))
            .take(2 * (ops.len().next_power_of_two() - ops.len()))
            .flatten(),
    );

    RowMajorMatrix::new(rows, width)
}
