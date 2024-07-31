use afs_stark_backend::interaction::InteractionBuilder;
use p3_field::AbstractField;

use crate::cpu::MEMORY_INTERACTION_BUS;

use super::{columns::MemoryOfflineCheckerCols, MemoryOfflineChecker};

impl MemoryOfflineChecker {
    pub fn eval_interactions<AB: InteractionBuilder>(
        &self,
        builder: &mut AB,
        local: &MemoryOfflineCheckerCols<AB::Var>,
    ) {
        let mut send_fields = vec![AB::Expr::one()];
        send_fields.extend(
            local
                .offline_checker_cols
                .idx
                .iter()
                .map(|idx| (*idx).into()),
        );
        send_fields.extend(
            local
                .offline_checker_cols
                .data
                .iter()
                .map(|data| (*data).into()),
        );
        // send when first access in a block is read
        let send_count = (AB::Expr::one() - local.offline_checker_cols.same_idx)
            * (AB::Expr::one() - local.offline_checker_cols.op_type)
            * local.offline_checker_cols.is_valid;
        builder.push_send(MEMORY_INTERACTION_BUS, send_fields, send_count);

        let mut rec_fields = vec![AB::Expr::neg_one()];
        rec_fields.extend(
            local
                .offline_checker_cols
                .idx
                .iter()
                .map(|idx| (*idx).into()),
        );
        rec_fields.extend(
            local
                .offline_checker_cols
                .data
                .iter()
                .map(|data| (*data).into()),
        );
        // receive when final access in a block
        let rec_count = local.is_final_access;
        builder.push_receive(MEMORY_INTERACTION_BUS, rec_fields, rec_count);
    }
}
