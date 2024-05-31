use p3_field::AbstractField;
use p3_matrix::dense::{DenseMatrix, RowMajorMatrix};
use p3_uni_stark::{StarkGenericConfig, Val};
use std::collections::HashMap;

use super::init_final_chip::InitFinalChip;
use super::middle_chip::MiddleChip;

#[derive(PartialEq, Clone)]
pub enum OpType {
    Read = 0,
    Write = 1,
}

pub struct Operation {
    pub clk: usize,
    pub key: Vec<u32>,
    pub val: Vec<u32>,
    pub op_type: OpType,
}

impl Operation {
    pub fn new(clk: usize, key: Vec<u32>, val: Vec<u32>, op_type: OpType) -> Self {
        Self {
            clk,
            key,
            val,
            op_type,
        }
    }
}

pub struct CheckerController<SC: StarkGenericConfig>
where
    Val<SC>: AbstractField,
{
    ops: Vec<Operation>,

    init_chip: InitFinalChip,
    middle_chip: MiddleChip,
    final_chip: InitFinalChip,

    init_page_trace: Option<DenseMatrix<Val<SC>>>,
    final_page_trace: Option<DenseMatrix<Val<SC>>>,
}

impl<SC: StarkGenericConfig> CheckerController<SC> {
    pub fn new(bus_index: usize) -> Self {
        Self {
            ops: vec![],
            init_chip: InitFinalChip::new(bus_index, 0, true),
            middle_chip: MiddleChip::new(bus_index, 0, 0),
            final_chip: InitFinalChip::new(bus_index, 0, false),
            init_page_trace: None,
            final_page_trace: None,
        }
    }

    fn get_page_trace(&self, page: Vec<Vec<u32>>, page_width: usize) -> DenseMatrix<Val<SC>> {
        RowMajorMatrix::new(
            page.into_iter()
                .flat_map(|row| {
                    row.into_iter()
                        .map(Val::<SC>::from_wrapped_u32)
                        .collect::<Vec<Val<SC>>>()
                })
                .collect(),
            page_width,
        )
    }

    pub fn load_page_and_ops(
        &mut self,
        mut page: Vec<Vec<u32>>,
        key_len: usize,
        val_len: usize,
        ops: Vec<Operation>,
    ) {
        assert!(page.len() > 0);
        self.init_page_trace = Some(self.get_page_trace(page.clone(), page[0].len()));

        let bus_index = self.middle_chip.bus_index();
        let page_len = key_len + val_len;

        self.init_chip = InitFinalChip::new(bus_index, page_len, true);
        self.middle_chip = MiddleChip::new(bus_index, key_len, val_len);
        self.final_chip = InitFinalChip::new(bus_index, page_len, false);

        let mut map: HashMap<Vec<u32>, usize> = HashMap::new();
        for (i, row) in page.iter().enumerate() {
            map.insert(row[..key_len].to_vec(), i);
        }

        for op in ops {
            if op.op_type == OpType::Read {
                continue;
            }

            let row = *map.get(&op.key).expect("Key in operation but not in page");
            page[row] = op.key.iter().chain(op.val.iter()).cloned().collect();
        }

        self.final_page_trace = Some(self.get_page_trace(page.clone(), page[0].len()));
    }
}
