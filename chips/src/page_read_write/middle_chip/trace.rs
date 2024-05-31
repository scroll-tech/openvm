use std::collections::HashMap;
use std::collections::HashSet;

use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use super::super::checker_controller::OpType;
use super::columns::MiddleChipCols;
use super::MiddleChip;

impl MiddleChip {
    // Each row in the trace follows the same order as the Cols struct:
    // [is_initial, is_final, clk, page_row, op_type]
    pub fn generate_trace<F: PrimeField64>(
        &self,
        init_page: Vec<Vec<u32>>,
        final_page: Vec<Vec<u32>>,
        key_index_map: HashMap<Vec<u32>, usize>,
    ) -> RowMajorMatrix<F> {
        let mut touched_keys = HashSet::new();
        for op in &self.ops {
            touched_keys.insert(op.key.clone());
        }

        let mut rows = vec![];

        // Adding to the trace an initial write operation for each touched row
        for key in &touched_keys {
            let index = key_index_map.get(key).unwrap();
            let cols = MiddleChipCols::new(
                F::from_canonical_u8(1),
                F::from_canonical_u8(0),
                F::from_canonical_u8(0),
                init_page[*index]
                    .clone()
                    .into_iter()
                    .map(F::from_canonical_u32)
                    .collect(),
                F::from_canonical_u8(1), // Write
            );

            rows.push(cols.flatten());
        }

        let mut max_clk = 0;

        // Adding to the trace all the internal operations
        for op in &self.ops {
            max_clk = max_clk.max(op.clk);

            let cols = MiddleChipCols::new(
                F::from_canonical_u8(0),
                F::from_canonical_u8(0),
                F::from_canonical_usize(op.clk),
                op.key
                    .clone()
                    .into_iter()
                    .chain(op.val.clone())
                    .map(F::from_canonical_u32)
                    .collect(),
                F::from_canonical_u8(op.op_type.clone() as u8),
            );
            rows.push(cols.flatten());
        }

        // Adding 1 to max_clk so its bigger than all timestamps in the operations
        max_clk += 1;

        // Adding to the trace a final write operation for each touched row
        for key in &touched_keys {
            let index = key_index_map.get(key).unwrap();
            let cols = MiddleChipCols::new(
                F::from_canonical_u8(0),
                F::from_canonical_u8(1),
                F::from_canonical_usize(max_clk),
                final_page[*index]
                    .clone()
                    .into_iter()
                    .map(F::from_canonical_u32)
                    .collect(),
                F::from_canonical_u8(0), // Read
            );
            rows.push(cols.flatten());
        }

        RowMajorMatrix::new(rows.concat(), self.air_width())
    }
}
