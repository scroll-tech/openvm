use std::{collections::HashMap, iter};

use p3_field::AbstractField;
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{StarkGenericConfig, Val};

use super::columns::OfflineCheckerCols;
use super::OfflineChecker;
use crate::is_equal_vec::IsEqualVecChip;
use crate::page_rw_checker::page_controller::Operation;
use crate::sub_chip::LocalTraceInstructions;

impl OfflineChecker {
    // TODO: update this
    // Each row in the trace follow the same order as the Cols struct:
    // [is_initial, is_final, clk, page_row, op_type, same_key, same_val, is_extra, is_equal_key_aux, is_equal_val_aux]
    //
    // The trace consists of a row for every read/write operation plus some extra rows
    // The trace is sorted by key (in page_row) and then by clk, so every key has a block of consective rows in the trace with the following structure
    // If the key exists in the initial page, the block starts with a row of the initial data with is_initial=1
    // Then, a row is added to the trace for every read/write operation with the corresponding data
    // Then, a row is added with the final value for that key with is_final=1
    // The trace is padded at the end to be of height trace_degree
    pub fn generate_trace<SC: StarkGenericConfig>(
        &self,
        page: &mut Vec<Vec<u32>>,
        mut ops: Vec<Operation>,
        trace_degree: usize,
    ) -> RowMajorMatrix<Val<SC>>
    where
        Val<SC>: AbstractField,
    {
        let is_equal_key = IsEqualVecChip::new(self.key_len);
        let is_equal_val = IsEqualVecChip::new(self.val_len);

        let mut rows_allocated = 0;
        while rows_allocated < page.len() && page[rows_allocated][0] == 1 {
            rows_allocated += 1;
        }

        let mut key_index_map = HashMap::new();
        for i in 0..rows_allocated {
            key_index_map.insert(page[i][1..self.key_len + 1].to_vec(), i);
        }

        // Creating a timestamp bigger than all others
        let max_clk = ops.iter().map(|op| op.clk).max().unwrap_or(0) + 1;

        ops.sort_by_key(|op| (op.key.clone(), op.clk));

        let conv_to_f = |v: Vec<u32>| {
            v.iter()
                .map(|x| Val::<SC>::from_canonical_u32(*x))
                .collect::<Vec<Val<SC>>>()
        };

        let gen_row = |is_first_row: &mut bool,
                       page: &mut Vec<Vec<u32>>,
                       index: usize,
                       is_initial: u8,
                       is_final: u8,
                       clk: usize,
                       op_type: u8,
                       last_key: &mut Vec<u32>,
                       last_val: &mut Vec<u32>,
                       is_extra: u8| {
            // Make sure the row in the page is allocated
            assert!(page[index][0] == 1);

            let cur_key = page[index][1..self.key_len + 1].to_vec();
            let cur_val = page[index][self.key_len + 1..].to_vec();

            if *is_first_row {
                // Making sure the last_key and last_val are different from current when its the first row
                *last_key = cur_key.clone();
                *last_val = cur_val.clone();

                last_key[0] += 1;
                last_val[0] += 1;

                *is_first_row = false;
            }

            let my_last_key = last_key.clone();
            let my_last_val = last_val.clone();

            *last_key = cur_key.clone();
            *last_val = cur_val.clone();

            let last_key = my_last_key;
            let last_val = my_last_val;

            let same_key = cur_key == last_key;
            let same_val = cur_val == last_val;

            let last_key = conv_to_f(last_key);
            let cur_key = conv_to_f(cur_key);

            let last_val = conv_to_f(last_val);
            let cur_val = conv_to_f(cur_val);

            let key_equal_cols =
                LocalTraceInstructions::generate_trace_row(&is_equal_key, (last_key, cur_key));

            let val_equal_cols =
                LocalTraceInstructions::generate_trace_row(&is_equal_val, (last_val, cur_val));

            let cols = OfflineCheckerCols::new(
                Val::<SC>::from_canonical_u8(is_initial),
                Val::<SC>::from_canonical_u8(is_final),
                Val::<SC>::from_canonical_usize(clk),
                page[index]
                    .to_vec()
                    .into_iter()
                    .map(Val::<SC>::from_canonical_u32)
                    .collect(),
                Val::<SC>::from_canonical_u8(op_type),
                Val::<SC>::from_canonical_u8(same_key as u8),
                Val::<SC>::from_canonical_u8(same_val as u8),
                Val::<SC>::from_canonical_u8(is_extra),
                key_equal_cols.aux,
                val_equal_cols.aux,
            );

            cols.flatten()
        };

        let mut rows = vec![];

        let mut last_key = vec![0; self.key_len];
        let mut last_val = vec![0; self.val_len];
        let mut is_first_row = true;

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

                rows.push(gen_row(
                    &mut is_first_row,
                    page,
                    idx,
                    1,
                    0,
                    0,
                    1,
                    &mut last_key,
                    &mut last_val,
                    0,
                ));
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
                    &mut is_first_row,
                    page,
                    idx,
                    0,
                    0,
                    ops[k].clk,
                    ops[k].op_type.clone() as u8,
                    &mut last_key,
                    &mut last_val,
                    0,
                ));
            }

            // Adding the is_final row to the trace
            rows.push(gen_row(
                &mut is_first_row,
                page,
                idx,
                0,
                1,
                max_clk,
                0,
                &mut last_key,
                &mut last_val,
                0,
            ));

            i = j;
        }

        // Ensure that trace degree is a power of two
        assert!(trace_degree > 0 && trace_degree & (trace_degree - 1) == 0);

        // Adding rows to the trace to make the height trace_degree
        while rows.len() < trace_degree {
            rows.push(gen_row(
                &mut is_first_row,
                page,
                0,
                0,
                0,
                0,
                0,
                &mut last_key,
                &mut last_val,
                1,
            ));
        }

        tracing::debug!("Middle Chip trace by row");
        for (i, row) in rows.iter().enumerate() {
            let cols =
                OfflineCheckerCols::from_slice(row, self.page_width(), self.key_len, self.val_len);
            tracing::debug!("row {}: {:?}", i, cols);
        }

        RowMajorMatrix::new(rows.concat(), self.air_width())
    }
}
