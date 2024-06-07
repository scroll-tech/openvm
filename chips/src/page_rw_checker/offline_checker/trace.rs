use std::collections::BTreeMap;
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
    // Each row in the trace follow the same order as the Cols struct:
    // [is_initial, is_final, clk, page_row, op_type, same_idx, same_data, is_extra, is_equal_idx_aux, is_equal_data_aux]
    //
    // The trace consists of a row for every read/write operation plus some extra rows
    // The trace is sorted by index (in page_row) and then by clk, so every index has a block of consective rows in the trace with the following structure
    // If the index exists in the initial page, the block starts with a row of the initial data with is_initial=1
    // Then, a row is added to the trace for every read/write operation with the corresponding data
    // Then, a row is added with the final data for that index with is_final=1
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
        let is_equal_idx = IsEqualVecChip::new(self.idx_len);
        let is_equal_data = IsEqualVecChip::new(self.data_len);

        let mut rows_allocated = 0;
        while rows_allocated < page.len() && page[rows_allocated][0] == 1 {
            rows_allocated += 1;
        }

        let mut idx_i_map = HashMap::new();
        for (i, row) in page.iter().enumerate().take(rows_allocated) {
            idx_i_map.insert(row[1..self.idx_len + 1].to_vec(), i);
        }

        // Creating a timestamp bigger than all others
        let max_clk = ops.iter().map(|op| op.clk).max().unwrap_or(0) + 1;

        ops.sort_by_key(|op| (op.idx.clone(), op.clk));

        let conv_to_f = |v: Vec<u32>| {
            v.iter()
                .map(|x| Val::<SC>::from_canonical_u32(*x))
                .collect::<Vec<Val<SC>>>()
        };

        let gen_row = |is_first_row: &mut bool,
                       page: &mut Vec<Vec<u32>>,
                       idx: usize,
                       is_initial: u8,
                       is_final: u8,
                       clk: usize,
                       op_type: u8,
                       last_idx: &mut Vec<u32>,
                       last_data: &mut Vec<u32>,
                       is_extra: u8| {
            // Make sure the row in the page is allocated
            assert!(page[idx][0] == 1);

            let cur_idx = page[idx][1..self.idx_len + 1].to_vec();
            let cur_data = page[idx][self.idx_len + 1..].to_vec();

            if *is_first_row {
                // Making sure the last_idx and last_data are different from current when its the first row
                last_idx.clone_from(&cur_idx);
                last_data.clone_from(&cur_data);

                last_idx[0] += 1;
                last_data[0] += 1;

                *is_first_row = false;
            }

            let my_last_idx = last_idx.clone();
            let my_last_data = last_data.clone();

            last_idx.clone_from(&cur_idx);
            last_data.clone_from(&cur_data);

            let last_idx = my_last_idx;
            let last_data = my_last_data;

            let same_idx = cur_idx == last_idx;
            let same_data = cur_data == last_data;

            let last_idx = conv_to_f(last_idx);
            let cur_idx = conv_to_f(cur_idx);

            let last_data = conv_to_f(last_data);
            let cur_data = conv_to_f(cur_data);

            let idx_equal_cols =
                LocalTraceInstructions::generate_trace_row(&is_equal_idx, (last_idx, cur_idx));

            let data_equal_cols =
                LocalTraceInstructions::generate_trace_row(&is_equal_data, (last_data, cur_data));

            let cols = OfflineCheckerCols::new(
                Val::<SC>::from_canonical_u8(is_initial),
                Val::<SC>::from_canonical_u8(is_final),
                Val::<SC>::from_canonical_usize(clk),
                page[idx]
                    .iter()
                    .copied()
                    .map(Val::<SC>::from_canonical_u32)
                    .collect(),
                Val::<SC>::from_canonical_u8(op_type),
                Val::<SC>::from_canonical_u8(same_idx as u8),
                Val::<SC>::from_canonical_u8(same_data as u8),
                Val::<SC>::from_canonical_u8(is_extra),
                idx_equal_cols.aux,
                data_equal_cols.aux,
            );

            cols.flatten()
        };

        let mut rows = vec![];

        let mut last_idx = vec![0; self.idx_len];
        let mut last_data = vec![0; self.data_len];
        let mut is_first_row = true;

        let mut i = 0;
        while i < ops.len() {
            let cur_idx = ops[i].idx.clone();

            let mut j = i + 1;
            while j < ops.len() && ops[j].idx == cur_idx {
                j += 1;
            }

            let idx;
            if idx_i_map.contains_key(&cur_idx) {
                // Adding the is_initial row to the trace
                idx = *idx_i_map.get(&cur_idx).unwrap();

                rows.push(gen_row(
                    &mut is_first_row,
                    page,
                    idx,
                    1,
                    0,
                    0,
                    1,
                    &mut last_idx,
                    &mut last_data,
                    0,
                ));
            } else {
                assert!(rows_allocated < page.len());
                idx = rows_allocated;
                idx_i_map.insert(cur_idx, idx);
                rows_allocated += 1;
            }

            for op in ops.iter().take(j).skip(i) {
                page[idx] = iter::once(1)
                    .chain(op.idx.clone())
                    .chain(op.data.clone())
                    .collect();

                rows.push(gen_row(
                    &mut is_first_row,
                    page,
                    idx,
                    0,
                    0,
                    op.clk,
                    op.op_type.clone() as u8,
                    &mut last_idx,
                    &mut last_data,
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
                &mut last_idx,
                &mut last_data,
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
                &mut last_idx,
                &mut last_data,
                1,
            ));
        }

        tracing::debug!("Offline Checker trace by row");
        for (i, row) in rows.iter().enumerate() {
            let cols =
                OfflineCheckerCols::from_slice(row, self.page_width(), self.idx_len, self.data_len);
            tracing::debug!("row {}: {:?}", i, cols);
        }

        RowMajorMatrix::new(rows.concat(), self.air_width())
    }

    pub fn generate_trace_from_map<SC: StarkGenericConfig>(
        &self,
        map: &mut BTreeMap<Vec<u32>, Vec<u32>>,
        mut ops: Vec<Operation>,
        trace_degree: usize,
    ) -> RowMajorMatrix<Val<SC>>
    where
        Val<SC>: AbstractField,
    {
        let is_equal_idx = IsEqualVecChip::new(self.idx_len);
        let is_equal_data = IsEqualVecChip::new(self.data_len);

        // Creating a timestamp bigger than all others
        let max_clk = ops.iter().map(|op| op.clk).max().unwrap_or(0) + 1;

        ops.sort_by_key(|op| (op.idx.clone(), op.clk));

        let conv_to_f = |v: Vec<u32>| {
            v.iter()
                .map(|x| Val::<SC>::from_canonical_u32(*x))
                .collect::<Vec<Val<SC>>>()
        };

        let gen_row = |is_first_row: &mut bool,
                       is_initial: u8,
                       is_final: u8,
                       clk: usize,
                       op_type: u8,
                       cur_idx: Vec<u32>,
                       cur_data: Vec<u32>,
                       last_idx: &mut Vec<u32>,
                       last_data: &mut Vec<u32>,
                       is_extra: u8| {
            // Make sure the row in the page is allocated

            if *is_first_row {
                // Making sure the last_idx and last_data are different from current when its the first row
                last_idx.clone_from(&cur_idx);
                last_data.clone_from(&cur_data);

                last_idx[0] += 1;
                last_data[0] += 1;

                *is_first_row = false;
            }

            let my_last_idx = last_idx.clone();
            let my_last_data = last_data.clone();

            last_idx.clone_from(&cur_idx);
            last_data.clone_from(&cur_data);

            let last_idx = my_last_idx;
            let last_data = my_last_data;

            let same_idx = cur_idx == last_idx;
            let same_data = cur_data == last_data;

            let last_idx = conv_to_f(last_idx);
            let cur_idx = conv_to_f(cur_idx);

            let last_data = conv_to_f(last_data);
            let cur_data = conv_to_f(cur_data);

            let mut page_row = vec![Val::<SC>::from_canonical_u32(1)];
            page_row.extend(cur_idx.clone());
            page_row.extend(cur_data.clone());

            let idx_equal_cols =
                LocalTraceInstructions::generate_trace_row(&is_equal_idx, (last_idx, cur_idx));

            let data_equal_cols =
                LocalTraceInstructions::generate_trace_row(&is_equal_data, (last_data, cur_data));

            let cols = OfflineCheckerCols::new(
                Val::<SC>::from_canonical_u8(is_initial),
                Val::<SC>::from_canonical_u8(is_final),
                Val::<SC>::from_canonical_usize(clk),
                page_row,
                Val::<SC>::from_canonical_u8(op_type),
                Val::<SC>::from_canonical_u8(same_idx as u8),
                Val::<SC>::from_canonical_u8(same_data as u8),
                Val::<SC>::from_canonical_u8(is_extra),
                idx_equal_cols.aux,
                data_equal_cols.aux,
            );

            cols.flatten()
        };

        let mut rows = vec![];

        let mut last_idx = vec![0; self.idx_len];
        let mut last_data = vec![0; self.data_len];
        let mut is_first_row = true;

        let mut i = 0;
        while i < ops.len() {
            let cur_idx = ops[i].idx.clone();
            let mut cur_data = ops[i].data.clone();
            let mut j = i + 1;
            while j < ops.len() && ops[j].idx == cur_idx {
                j += 1;
            }

            if map.contains_key(&cur_idx) {
                // Adding the is_initial row to the trace
                rows.push(gen_row(
                    &mut is_first_row,
                    1,
                    0,
                    0,
                    1,
                    cur_idx.clone(),
                    cur_data.clone(),
                    &mut last_idx,
                    &mut last_data,
                    0,
                ));
            }

            for op in ops.iter().take(j).skip(i) {
                map.insert(op.idx.clone(), op.data.clone());
                cur_data = op.data.clone();
                rows.push(gen_row(
                    &mut is_first_row,
                    0,
                    0,
                    op.clk,
                    op.op_type.clone() as u8,
                    cur_idx.clone(),
                    cur_data.clone(),
                    &mut last_idx,
                    &mut last_data,
                    0,
                ));
            }

            // Adding the is_final row to the trace
            rows.push(gen_row(
                &mut is_first_row,
                0,
                1,
                max_clk,
                0,
                cur_idx.clone(),
                cur_data.clone(),
                &mut last_idx,
                &mut last_data,
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
                0,
                0,
                0,
                0,
                last_idx.clone(),
                last_data.clone(),
                &mut last_idx,
                &mut last_data,
                1,
            ));
        }

        tracing::debug!("Offline Checker trace by row");
        for (i, row) in rows.iter().enumerate() {
            let cols =
                OfflineCheckerCols::from_slice(row, self.page_width(), self.idx_len, self.data_len);
            tracing::debug!("row {}: {:?}", i, cols);
        }

        RowMajorMatrix::new(rows.concat(), self.air_width())
    }
}
