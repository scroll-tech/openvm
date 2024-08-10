use afs_stark_backend::interaction::InteractionBuilder;
use p3_field::{AbstractField};

use super::{columns::FieldExtensionArithmeticCols, EXTENSION_DEGREE, FieldExtensionArithmeticAir};
use crate::cpu::{FIELD_EXTENSION_BUS, MEMORY_BUS};

fn eval_rw_interactions<const WORD_SIZE: usize, AB: InteractionBuilder>(
    builder: &mut AB,
    is_write: bool,
    local: &FieldExtensionArithmeticCols<AB::Var>,
    addr_space: AB::Var,
    address: AB::Var,
    access_idx: usize,
) {
    let io = &local.io;
    let aux = &local.aux;

    // TODO: Handle WORD_SIZE > EXTENSION_DEGREE.

    let data: [AB::Var; EXTENSION_DEGREE] = [io.x, io.y, io.z][access_idx];
    let timestamp = aux.start_timestamp + AB::F::from_canonical_usize(access_idx);

    let fields = [
        timestamp,
        AB::Expr::from_bool(is_write),
        addr_space.into(),
        address.into(),
    ]
        .into_iter()
        .chain(data.into_iter().map(Into::into));

    if access_idx == 1 {
        builder.push_send(MEMORY_BUS, fields, aux.valid_y_read);
    } else {
        builder.push_send(MEMORY_BUS, fields, aux.is_valid);
    }
}

impl<const WORD_SIZE: usize> FieldExtensionArithmeticAir<WORD_SIZE> {
    pub fn eval_interactions<AB: InteractionBuilder>(
        &self,
        builder: &mut AB,
        local: &FieldExtensionArithmeticCols<AB::Var>,
    ) {
        // reads for x
        eval_rw_interactions::<WORD_SIZE, _>(builder, false, local, local.aux.d, local.aux.op_b, 0);
        // reads for y
        eval_rw_interactions::<WORD_SIZE, _>(builder, false, local, local.aux.e, local.aux.op_c, 1);
        // writes for z
        eval_rw_interactions::<WORD_SIZE, _>(builder, true, local, local.aux.d, local.aux.op_a, 2);

        // Receives all IO columns from another chip on bus 3 (FIELD_EXTENSION_BUS)
        let fields = [
            local.io.opcode,
            local.aux.op_a,
            local.aux.op_b,
            local.aux.op_c,
            local.aux.d,
            local.aux.e,
        ];
        builder.push_receive(FIELD_EXTENSION_BUS, fields, local.aux.is_valid);
    }
}
