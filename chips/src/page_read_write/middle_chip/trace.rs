use std::collections::HashMap;
use std::iter;

use p3_field::AbstractField;
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{StarkGenericConfig, Val};

use super::columns::MiddleChipCols;
use super::MiddleChip;
use crate::page_read_write::page_controller::Operation;

impl MiddleChip {
    // Each row in the trace follows the same order as the Cols struct:
    // [is_initial, is_final, clk, page_row, op_type]
    pub fn generate_trace<SC: StarkGenericConfig>(
        &self,
        page: &mut Vec<Vec<u32>>,
        mut ops: Vec<Operation>,
    ) -> RowMajorMatrix<Val<SC>>
    where
        Val<SC>: AbstractField,
    {
        println!("ops.len(): {:?}", ops.len());

        let mut rows_allocated = 0;
        while page[rows_allocated][0] == 1 {
            rows_allocated += 1;
        }

        let mut key_index_map = HashMap::new();
        for i in 0..rows_allocated {
            key_index_map.insert(page[i][1..self.key_len + 1].to_vec(), i);
        }

        // Creating a timestamp bigger than all others
        let max_clk = ops.iter().map(|op| op.clk).max().unwrap_or(0) + 1;

        ops.sort_by_key(|op| (op.key.clone(), op.clk));

        let gen_row = |page: &mut Vec<Vec<u32>>,
                       index: usize,
                       is_initial: u8,
                       is_final: u8,
                       clk: usize,
                       op_type: u8,
                       same_key: u8,
                       same_val: u8,
                       is_extra: u8| {
            // Make sure the row in the page is allocated
            assert!(page[index][0] == 1);

            let cols = MiddleChipCols::new(
                Val::<SC>::from_canonical_u8(is_initial),
                Val::<SC>::from_canonical_u8(is_final),
                Val::<SC>::from_canonical_usize(clk),
                page[index]
                    .to_vec()
                    .into_iter()
                    .map(Val::<SC>::from_canonical_u32)
                    .collect(),
                Val::<SC>::from_canonical_u8(op_type),
                Val::<SC>::from_canonical_u8(same_key),
                Val::<SC>::from_canonical_u8(same_val),
                Val::<SC>::from_canonical_u8(is_extra),
            );
            cols.flatten()
        };

        let mut rows = vec![];

        let mut last_val = vec![];
        let mut i = 0;
        while i < ops.len() {
            let cur_key = ops[i].key.clone();

            let mut j = i + 1;
            while j < ops.len() && ops[j].key == cur_key {
                j += 1;
            }

            let idx;
            if key_index_map.contains_key(&cur_key) {
                // Adding the is_initial row to the trace
                idx = *key_index_map.get(&cur_key).unwrap();

                let cur_val = page[idx][self.key_len + 1..].to_vec();
                rows.push(gen_row(
                    page,
                    idx,
                    1,
                    0,
                    0,
                    1,
                    0,
                    (last_val == cur_val) as u8,
                    0,
                ));
                last_val = cur_val;
            } else {
                assert!(rows_allocated < page.len());
                idx = rows_allocated;
                key_index_map.insert(cur_key, idx);
                rows_allocated += 1;
            }

            for k in i..j {
                page[idx] = iter::once(1)
                    .chain(ops[k].key.clone())
                    .chain(ops[k].val.clone())
                    .collect();

                rows.push(gen_row(
                    page,
                    idx,
                    0,
                    0,
                    ops[k].clk,
                    ops[k].op_type.clone() as u8,
                    1,
                    (last_val == ops[k].val) as u8,
                    0,
                ));
                last_val = ops[k].val.clone();
            }

            // Adding the is_final row to the trace
            rows.push(gen_row(page, idx, 0, 1, max_clk, 0, 1, 1, 0));

            i = j;
        }

        println!("before extending: rows.len(): {:?}", rows.len());

        // Adding rows to the trace to make the height a power of two
        let dummy_row = gen_row(page, 0, 0, 0, 0, 0, 0, 0, 1);
        let is_power_of_two = |x: usize| x > 0 && (x & (x - 1)) == 0;
        while !is_power_of_two(rows.len()) {
            rows.push(dummy_row.clone());
        }

        println!("after extending: rows.len(): {:?}", rows.len());

        println!(
            "generating the middle chip trace and the height of the trace is {:?}",
            rows.len()
        );

        RowMajorMatrix::new(rows.concat(), self.air_width())
    }
}
