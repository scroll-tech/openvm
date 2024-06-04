use afs_stark_backend::config::Com;
use afs_stark_backend::prover::trace::{ProverTraceData, TraceCommitter};
use p3_field::AbstractField;
use p3_matrix::dense::{DenseMatrix, RowMajorMatrix};
use p3_matrix::Matrix;
use p3_uni_stark::{StarkGenericConfig, Val};

use super::init_final_chip::InitFinalChip;
use super::middle_chip::MiddleChip;

#[derive(PartialEq, Clone, Debug)]
pub enum OpType {
    Read = 0,
    Write = 1,
}

#[derive(Clone, Debug)]
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

pub struct PageController<SC: StarkGenericConfig>
where
    Val<SC>: AbstractField,
{
    pub init_chip: InitFinalChip,
    pub middle_chip: MiddleChip,
    pub final_chip: InitFinalChip,

    init_chip_trace: Option<DenseMatrix<Val<SC>>>,
    pub middle_chip_trace: Option<DenseMatrix<Val<SC>>>,
    final_chip_trace: Option<DenseMatrix<Val<SC>>>,

    init_page_commitment: Option<Com<SC>>,
    final_page_commitment: Option<Com<SC>>,
}

impl<SC: StarkGenericConfig> PageController<SC> {
    pub fn new(bus_index: usize, key_len: usize, val_len: usize) -> Self {
        Self {
            init_chip: InitFinalChip::new(bus_index, 1 + key_len + val_len, true),
            middle_chip: MiddleChip::new(bus_index, key_len, val_len),
            final_chip: InitFinalChip::new(bus_index, 1 + key_len + val_len, false),

            init_chip_trace: None,
            middle_chip_trace: None,
            final_chip_trace: None,

            init_page_commitment: None,
            final_page_commitment: None,
        }
    }

    pub fn middle_chip_trace(&self) -> DenseMatrix<Val<SC>> {
        self.middle_chip_trace.clone().unwrap()
    }

    fn get_page_trace(&self, page: Vec<Vec<u32>>) -> DenseMatrix<Val<SC>> {
        self.init_chip.generate_trace::<SC>(page)
    }

    fn gen_ops_trace(
        &self,
        page: &mut Vec<Vec<u32>>,
        ops: &Vec<Operation>,
        trace_degree: usize,
    ) -> RowMajorMatrix<Val<SC>> {
        self.middle_chip
            .generate_trace::<SC>(page, ops.clone(), trace_degree)
    }

    pub fn load_page_and_ops(
        &mut self,
        mut page: Vec<Vec<u32>>,
        key_len: usize,
        val_len: usize,
        ops: Vec<Operation>,
        trace_degree: usize,
        trace_committer: &mut TraceCommitter<SC>,
    ) -> (Vec<DenseMatrix<Val<SC>>>, Vec<ProverTraceData<SC>>) {
        println!("in load_page_and_ops");
        println!("page: {:?}", page);

        assert!(page.len() > 0);
        self.init_chip_trace = Some(self.get_page_trace(page.clone()));

        let bus_index = self.middle_chip.bus_index();
        let page_width = 1 + key_len + val_len;

        self.init_chip = InitFinalChip::new(bus_index, page_width, true);
        self.init_chip_trace = Some(self.get_page_trace(page.clone()));

        println!("initialized init_chip and its trace");

        self.middle_chip = MiddleChip::new(bus_index, key_len, val_len);
        self.middle_chip_trace = Some(self.gen_ops_trace(&mut page, &ops, trace_degree));

        println!("initialized middle_chip and its trace");

        self.final_chip = InitFinalChip::new(bus_index, page_width, false);
        self.final_chip_trace = Some(self.get_page_trace(page.clone()));

        println!("initialized final_chip and its trace");

        println!("initial page trace: {:?}", self.init_chip_trace);

        let prover_data = vec![
            trace_committer.commit(vec![self.init_chip_trace.clone().unwrap()]),
            trace_committer.commit(vec![self.final_chip_trace.clone().unwrap()]),
        ];

        println!("committed to the traces using the committer");

        self.init_page_commitment = Some(prover_data[0].commit.clone());
        self.final_page_commitment = Some(prover_data[1].commit.clone());

        println!(
            "heights of all traces: {} {} {}",
            self.init_chip_trace.as_ref().unwrap().height(),
            self.middle_chip_trace.as_ref().unwrap().height(),
            self.final_chip_trace.as_ref().unwrap().height()
        );

        (
            vec![
                self.init_chip_trace.clone().unwrap(),
                self.final_chip_trace.clone().unwrap(),
            ],
            prover_data,
        )
    }
}
