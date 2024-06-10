use std::collections::BTreeMap;
use std::sync::Arc;

use afs_stark_backend::config::Com;
use afs_stark_backend::prover::trace::{ProverTraceData, TraceCommitter};
use p3_field::{AbstractField, PrimeField64};
use p3_matrix::dense::{DenseMatrix, RowMajorMatrix};
use p3_matrix::Matrix;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::page_rw_checker::offline_checker::OfflineChecker;
use crate::page_rw_checker::page_controller::Operation;
use crate::range_gate::RangeCheckerGateChip;

use super::internal_page_chip::InternalPageChip;
use super::leaf_page_chip::LeafPageChip;
use super::root_signal_chip::RootSignalChip;

#[derive(Clone)]
pub struct PageTreeParams {
    pub path_bus_index: usize,
    pub leaf_cap: usize,
    pub internal_cap: usize,
    pub leaf_page_height: usize,
    pub internal_page_height: usize,
}

#[derive(Clone)]
pub struct LessThanTupleParams {
    pub range_max: u32,
    pub limb_bits: Vec<usize>,
    pub decomp: usize,
}

#[derive(Clone)]
struct Node(bool, usize);

fn cmp(key1: &Vec<u32>, key2: &Vec<u32>) -> i32 {
    assert!(key1.len() == key2.len());
    let mut i = 0;
    while i < key1.len() && key1[i] == key2[i] {
        i += 1;
    }
    if i == key1.len() {
        return 0;
    } else {
        return 2 * ((key1[i] > key2[i]) as i32) - 1;
    }
}

struct PageTreeGraph<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub root: Node,
    // pub adj: BTreeMap<Node, Vec<Node>>,
    pub mults: Vec<Vec<u32>>,
    pub leaf_ranges: Vec<(Vec<u32>, Vec<u32>)>,
    pub internal_ranges: Vec<(Vec<u32>, Vec<u32>)>,
    pub root_range: (Vec<u32>, Vec<u32>),
    pub root_mult: u32,
    pub commitment_to_node: BTreeMap<Vec<Val<SC>>, Node>,
    pub map: BTreeMap<Vec<u32>, Vec<u32>>,
}

impl<SC: StarkGenericConfig, const COMMITMENT_LEN: usize> PageTreeGraph<SC, COMMITMENT_LEN>
where
    Val<SC>: AbstractField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    pub fn dfs(
        leaf_pages: &Vec<Vec<Vec<u32>>>,
        internal_pages: &Vec<Vec<Vec<u32>>>,
        leaf_ranges: &mut Vec<(Vec<u32>, Vec<u32>)>,
        internal_ranges: &mut Vec<(Vec<u32>, Vec<u32>)>,
        commitment_to_node: &BTreeMap<Vec<Val<SC>>, Node>,
        map: &mut BTreeMap<Vec<u32>, Vec<u32>>,
        mults: &mut Vec<Vec<u32>>,
        idx_len: usize,
        cur_node: Node,
    ) -> u32 {
        if !cur_node.0 {
            let mut ans = 0;
            let mut mult = vec![];
            for row in &internal_pages[cur_node.1] {
                if row[0] == 0 {
                    mult.push(0);
                } else {
                    let f_row: Vec<Val<SC>> = row
                        .iter()
                        .map(|r| Val::<SC>::from_canonical_u32(*r))
                        .collect();
                    let next_node = commitment_to_node
                        .get(&f_row[1 + 2 * idx_len..1 + 2 * idx_len + COMMITMENT_LEN].to_vec());
                    match next_node {
                        None => mult.push(1),
                        Some(n) => {
                            if n.0 {
                                leaf_ranges[n.1].0 = row[1..1 + idx_len].to_vec();
                                leaf_ranges[n.1].1 = row[1 + idx_len..1 + 2 * idx_len].to_vec();
                            } else {
                                internal_ranges[n.1].0 = row[1..1 + idx_len].to_vec();
                                internal_ranges[n.1].1 = row[1 + idx_len..1 + 2 * idx_len].to_vec();
                            }
                            let m = PageTreeGraph::<SC, COMMITMENT_LEN>::dfs(
                                leaf_pages,
                                internal_pages,
                                leaf_ranges,
                                internal_ranges,
                                commitment_to_node,
                                map,
                                mults,
                                idx_len,
                                n.clone(),
                            );
                            ans += m;
                            mult.push(m);
                        }
                    }
                }
            }
            mults[cur_node.1] = mult;
            return ans + 1;
        } else {
            let mut ans = 0;
            for row in &leaf_pages[cur_node.1] {
                if row[0] == 1 {
                    map.insert(row[1..1 + idx_len].to_vec(), row[1 + idx_len..].to_vec());
                    ans += 1;
                }
            }
            return ans + 1;
        }
    }

    pub fn new(
        leaf_pages: &Vec<Vec<Vec<u32>>>,
        internal_pages: &Vec<Vec<Vec<u32>>>,
        leaf_commits: &Vec<Com<SC>>,
        internal_commits: &Vec<Com<SC>>,
        root: (bool, usize),
        idx_len: usize,
    ) -> Self {
        let root = Node(root.0, root.1);
        let leaf_commits: Vec<[Val<SC>; COMMITMENT_LEN]> = leaf_commits
            .into_iter()
            .map(|c| {
                let c: [Val<SC>; COMMITMENT_LEN] = c.clone().into();
                c
            })
            .collect();
        let internal_commits: Vec<[Val<SC>; COMMITMENT_LEN]> = internal_commits
            .into_iter()
            .map(|c| {
                let c: [Val<SC>; COMMITMENT_LEN] = c.clone().into();
                c
            })
            .collect();
        let mut commitment_to_node = BTreeMap::<Vec<Val<SC>>, Node>::new();
        let root_commitment = if root.0 {
            leaf_commits[root.1]
        } else {
            internal_commits[root.1]
        };
        let mut leaf_ranges = vec![(vec![], vec![]); leaf_pages.len()];
        let mut internal_ranges = vec![(vec![], vec![]); internal_pages.len()];
        if root.0 {
            for row in &leaf_pages[root.1] {
                if row[0] == 1 {
                    if leaf_ranges[root.1].0.len() == 0 {
                        leaf_ranges[root.1] =
                            (row[1..1 + idx_len].to_vec(), row[1..1 + idx_len].to_vec());
                    } else {
                        let idx = row[1..1 + idx_len].to_vec();
                        if cmp(&leaf_ranges[root.1].0, &idx) > 0 {
                            leaf_ranges[root.1].0 = idx.clone();
                        }
                        if cmp(&leaf_ranges[root.1].1, &idx) < 0 {
                            leaf_ranges[root.1].1 = idx;
                        }
                    }
                }
            }
        } else {
            for row in &internal_pages[root.1] {
                if row[0] == 1 {
                    let idx1 = row[1..1 + idx_len].to_vec();
                    let idx2 = row[1 + idx_len..1 + 2 * idx_len].to_vec();
                    if internal_ranges[root.1].0.len() == 0 {
                        internal_ranges[root.1] = (idx1, idx2);
                    } else {
                        if cmp(&internal_ranges[root.1].0, &idx1) > 0 {
                            internal_ranges[root.1].0 = idx1;
                        }
                        if cmp(&internal_ranges[root.1].1, &idx2) < 0 {
                            internal_ranges[root.1].1 = idx2;
                        }
                    }
                }
            }
        }
        for (i, c) in leaf_commits.iter().enumerate() {
            commitment_to_node.insert(c.clone().to_vec(), Node(true, i));
        }
        for (i, c) in internal_commits.iter().enumerate() {
            commitment_to_node.insert(c.clone().to_vec(), Node(false, i));
        }
        let mut mults = vec![vec![0; internal_pages[0].len()]; internal_pages.len()];
        commitment_to_node.insert(root_commitment.clone().to_vec(), root.clone());
        let mut map = BTreeMap::<Vec<u32>, Vec<u32>>::new();
        let root_mult = PageTreeGraph::<SC, COMMITMENT_LEN>::dfs(
            leaf_pages,
            internal_pages,
            &mut leaf_ranges,
            &mut internal_ranges,
            &commitment_to_node,
            &mut map,
            &mut mults,
            idx_len,
            root.clone(),
        );
        for i in 0..leaf_ranges.len() {
            if leaf_ranges[i].0.len() == 0 {
                leaf_ranges[i] = (vec![0; idx_len], vec![0; idx_len]);
            }
        }
        for i in 0..internal_ranges.len() {
            if internal_ranges[i].0.len() == 0 {
                internal_ranges[i] = (vec![0; idx_len], vec![0; idx_len]);
            }
        }
        let root_range = if root.0 {
            leaf_ranges[root.1].clone()
        } else {
            internal_ranges[root.1].clone()
        };
        PageTreeGraph {
            root,
            root_range,
            mults,
            root_mult,
            commitment_to_node,
            leaf_ranges,
            internal_ranges,
            map,
        }
    }
}

#[derive(Clone)]
pub struct PageControllerDataTrace<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub init_leaf_chip_traces: Vec<DenseMatrix<Val<SC>>>,
    pub init_internal_chip_traces: Vec<DenseMatrix<Val<SC>>>,
    pub final_leaf_chip_traces: Vec<DenseMatrix<Val<SC>>>,
    pub final_internal_chip_traces: Vec<DenseMatrix<Val<SC>>>,
}

#[derive(Clone)]
pub struct PageControllerMainTrace<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub init_root_signal_trace: DenseMatrix<Val<SC>>,
    pub init_leaf_chip_main_traces: Vec<DenseMatrix<Val<SC>>>,
    pub init_internal_chip_main_traces: Vec<DenseMatrix<Val<SC>>>,
    pub offline_checker_trace: DenseMatrix<Val<SC>>,
    pub final_root_signal_trace: DenseMatrix<Val<SC>>,
    pub final_leaf_chip_main_traces: Vec<DenseMatrix<Val<SC>>>,
    pub final_internal_chip_main_traces: Vec<DenseMatrix<Val<SC>>>,
}

pub struct PageControllerProverData<SC: StarkGenericConfig>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub init_leaf_page: Vec<ProverTraceData<SC>>,
    pub init_internal_page: Vec<ProverTraceData<SC>>,
    pub final_leaf_page: Vec<ProverTraceData<SC>>,
    pub final_internal_page: Vec<ProverTraceData<SC>>,
}

#[derive(Clone)]
pub struct PageControllerCommit<SC: StarkGenericConfig>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub init_leaf_page_commitments: Vec<Com<SC>>,
    pub init_internal_page_commitments: Vec<Com<SC>>,
    pub init_root_commitment: Com<SC>,
    pub final_leaf_page_commitments: Vec<Com<SC>>,
    pub final_internal_page_commitments: Vec<Com<SC>>,
    pub final_root_commitment: Com<SC>,
}

#[derive(Clone)]
pub struct PageControllerParams {
    pub idx_len: usize,
    pub data_len: usize,
    pub commitment_len: usize,
    pub init_tree_params: PageTreeParams,
    pub final_tree_params: PageTreeParams,
}

pub struct PageController<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub init_root_signal: RootSignalChip<COMMITMENT_LEN>,
    pub init_leaf_chips: Vec<LeafPageChip<COMMITMENT_LEN>>,
    pub init_internal_chips: Vec<InternalPageChip<COMMITMENT_LEN>>,
    pub offline_checker: OfflineChecker,
    pub final_root_signal: RootSignalChip<COMMITMENT_LEN>,
    pub final_leaf_chips: Vec<LeafPageChip<COMMITMENT_LEN>>,
    pub final_internal_chips: Vec<InternalPageChip<COMMITMENT_LEN>>,
    pub params: PageControllerParams,
    pub data_trace: Option<PageControllerDataTrace<SC, COMMITMENT_LEN>>,
    pub main_trace: Option<PageControllerMainTrace<SC, COMMITMENT_LEN>>,
    pub commit: Option<PageControllerCommit<SC>>,
    range_checker: Arc<RangeCheckerGateChip>,
}

impl<SC: StarkGenericConfig, const COMMITMENT_LEN: usize> PageController<SC, COMMITMENT_LEN>
where
    Val<SC>: AbstractField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    pub fn new(
        data_bus_index: usize,
        internal_data_bus_index: usize,
        lt_bus_index: usize,
        idx_len: usize,
        data_len: usize,
        init_param: PageTreeParams,
        final_param: PageTreeParams,
        less_than_tuple_param: LessThanTupleParams,
        range_checker: Arc<RangeCheckerGateChip>,
    ) -> Self {
        Self {
            init_leaf_chips: vec![
                LeafPageChip::new(
                    init_param.path_bus_index,
                    data_bus_index,
                    less_than_tuple_param.clone(),
                    lt_bus_index,
                    idx_len,
                    data_len,
                    true
                );
                init_param.leaf_cap
            ],
            init_internal_chips: vec![
                InternalPageChip::new(
                    init_param.path_bus_index,
                    internal_data_bus_index,
                    less_than_tuple_param.clone(),
                    lt_bus_index,
                    idx_len,
                    true
                );
                init_param.internal_cap
            ],
            offline_checker: OfflineChecker::new(data_bus_index, idx_len, data_len),
            final_leaf_chips: vec![
                LeafPageChip::new(
                    final_param.path_bus_index,
                    data_bus_index,
                    less_than_tuple_param.clone(),
                    lt_bus_index,
                    idx_len,
                    data_len,
                    false
                );
                init_param.leaf_cap
            ],
            final_internal_chips: vec![
                InternalPageChip::new(
                    final_param.path_bus_index,
                    internal_data_bus_index,
                    less_than_tuple_param.clone(),
                    lt_bus_index,
                    idx_len,
                    false
                );
                init_param.internal_cap
            ],
            init_root_signal: RootSignalChip::new(init_param.path_bus_index, true, idx_len),
            final_root_signal: RootSignalChip::new(final_param.path_bus_index, false, idx_len),
            commit: None,
            params: PageControllerParams {
                idx_len,
                data_len,
                commitment_len: COMMITMENT_LEN,
                init_tree_params: init_param,
                final_tree_params: final_param,
            },
            data_trace: None,
            main_trace: None,
            range_checker,
        }
    }

    // pub fn offline_checker_trace(&self) -> DenseMatrix<Val<SC>> {
    //     self.main_trace.clone().unwrap().offline_checker_trace
    // }

    fn get_leaf_page_cache_trace(&self, page: Vec<Vec<u32>>) -> DenseMatrix<Val<SC>> {
        self.init_leaf_chips[0].generate_cached_trace::<Val<SC>>(page)
    }

    fn get_internal_page_cache_trace(&self, page: Vec<Vec<u32>>) -> DenseMatrix<Val<SC>> {
        self.init_internal_chips[0].generate_cached_trace::<Val<SC>>(page)
    }

    fn gen_ops_trace(
        &self,
        map: &BTreeMap<Vec<u32>, Vec<u32>>,
        ops: &[Operation],
        trace_degree: usize,
    ) -> RowMajorMatrix<Val<SC>> {
        self.offline_checker.generate_trace_from_map::<SC>(
            &mut map.clone(),
            ops.to_owned(),
            trace_degree,
        )
    }

    pub fn load_page_and_ops(
        &mut self,
        init_leaf_pages: Vec<Vec<Vec<u32>>>,
        init_internal_pages: Vec<Vec<Vec<u32>>>,
        init_root_is_leaf: bool,
        init_root_idx: usize,
        final_leaf_pages: Vec<Vec<Vec<u32>>>,
        final_internal_pages: Vec<Vec<Vec<u32>>>,
        final_root_is_leaf: bool,
        final_root_idx: usize,
        ops: Vec<Operation>,
        trace_degree: usize,
        trace_committer: &mut TraceCommitter<SC>,
    ) -> (
        PageControllerDataTrace<SC, COMMITMENT_LEN>,
        PageControllerMainTrace<SC, COMMITMENT_LEN>,
        PageControllerCommit<SC>,
        PageControllerProverData<SC>,
    ) {
        let init_leaf_height = self.params.init_tree_params.leaf_page_height;
        let init_internal_height = self.params.init_tree_params.internal_page_height;
        let final_leaf_height = self.params.final_tree_params.leaf_page_height;
        let final_internal_height = self.params.final_tree_params.internal_page_height;
        let blank_init_leaf =
            vec![vec![0; 1 + self.params.idx_len + self.params.data_len]; init_leaf_height];
        let blank_init_internal =
            vec![
                vec![0; 1 + 2 * self.params.idx_len + self.params.commitment_len];
                init_internal_height
            ];
        let blank_final_leaf =
            vec![vec![0; 1 + self.params.idx_len + self.params.data_len]; final_leaf_height];
        let blank_final_internal =
            vec![
                vec![0; 1 + 2 * self.params.idx_len + self.params.commitment_len];
                final_internal_height
            ];

        let mut init_leaf_pages = init_leaf_pages.clone();
        init_leaf_pages.resize(
            self.params.init_tree_params.leaf_cap,
            blank_init_leaf.clone(),
        );

        let mut init_internal_pages = init_internal_pages.clone();
        init_internal_pages.resize(
            self.params.init_tree_params.internal_cap,
            blank_init_internal.clone(),
        );

        let mut final_leaf_pages = final_leaf_pages.clone();
        final_leaf_pages.resize(
            self.params.final_tree_params.leaf_cap,
            blank_final_leaf.clone(),
        );

        let mut final_internal_pages = final_internal_pages.clone();
        final_internal_pages.resize(
            self.params.final_tree_params.internal_cap,
            blank_final_internal.clone(),
        );

        let init_leaf_trace = init_leaf_pages
            .iter()
            .zip(self.init_leaf_chips.iter())
            .map(|(page, chip)| chip.generate_cached_trace::<Val<SC>>(page.clone()))
            .collect::<Vec<_>>();

        let init_internal_trace = init_internal_pages
            .iter()
            .zip(self.init_internal_chips.iter())
            .map(|(page, chip)| chip.generate_cached_trace::<Val<SC>>(page.clone()))
            .collect::<Vec<_>>();

        let final_leaf_trace = final_leaf_pages
            .iter()
            .zip(self.final_leaf_chips.iter())
            .map(|(page, chip)| chip.generate_cached_trace::<Val<SC>>(page.clone()))
            .collect::<Vec<_>>();

        let final_internal_trace = final_internal_pages
            .iter()
            .zip(self.final_internal_chips.iter())
            .map(|(page, chip)| chip.generate_cached_trace::<Val<SC>>(page.clone()))
            .collect::<Vec<_>>();

        let init_leaf_prover_data = init_leaf_trace
            .iter()
            .map(|trace| trace_committer.commit(vec![trace.clone()]))
            .collect::<Vec<_>>();

        let init_internal_prover_data = init_internal_trace
            .iter()
            .map(|trace| trace_committer.commit(vec![trace.clone()]))
            .collect::<Vec<_>>();

        let final_leaf_prover_data = final_leaf_trace
            .iter()
            .map(|trace| trace_committer.commit(vec![trace.clone()]))
            .collect::<Vec<_>>();

        let final_internal_prover_data = final_internal_trace
            .iter()
            .map(|trace| trace_committer.commit(vec![trace.clone()]))
            .collect::<Vec<_>>();

        let init_leaf_commitment = init_leaf_prover_data
            .iter()
            .map(|data| data.commit.clone())
            .collect::<Vec<_>>();

        let init_internal_commitment = init_internal_prover_data
            .iter()
            .map(|data| data.commit.clone())
            .collect::<Vec<_>>();

        let final_leaf_commitment = final_leaf_prover_data
            .iter()
            .map(|data| data.commit.clone())
            .collect::<Vec<_>>();

        let final_internal_commitment = final_internal_prover_data
            .iter()
            .map(|data| data.commit.clone())
            .collect::<Vec<_>>();

        let init_leaf_commitment_arr = init_leaf_commitment
            .iter()
            .map(|data| data.clone().into())
            .collect::<Vec<[Val<SC>; COMMITMENT_LEN]>>();

        let init_internal_commitment_arr = init_internal_commitment
            .iter()
            .map(|data| data.clone().into())
            .collect::<Vec<[Val<SC>; COMMITMENT_LEN]>>();

        let final_leaf_commitment_arr = final_leaf_commitment
            .iter()
            .map(|data| data.clone().into())
            .collect::<Vec<[Val<SC>; COMMITMENT_LEN]>>();

        let final_internal_commitment_arr = final_internal_commitment
            .iter()
            .map(|data| data.clone().into())
            .collect::<Vec<[Val<SC>; COMMITMENT_LEN]>>();

        let prover_data = PageControllerProverData {
            init_leaf_page: init_leaf_prover_data,
            init_internal_page: init_internal_prover_data,
            final_leaf_page: final_leaf_prover_data,
            final_internal_page: final_internal_prover_data,
        };

        let init_tree = PageTreeGraph::<SC, COMMITMENT_LEN>::new(
            &init_leaf_pages,
            &init_internal_pages,
            &init_leaf_commitment,
            &init_internal_commitment,
            (init_root_is_leaf, init_root_idx),
            self.params.idx_len,
        );

        let final_tree = PageTreeGraph::<SC, COMMITMENT_LEN>::new(
            &final_leaf_pages,
            &final_internal_pages,
            &final_leaf_commitment,
            &final_internal_commitment,
            (final_root_is_leaf, final_root_idx),
            self.params.idx_len,
        );

        let mut init_leaf_main_trace = vec![];
        for i in 0..init_leaf_commitment_arr.len() {
            let commit = init_leaf_commitment_arr[i]
                .into_iter()
                .map(|c| c.as_canonical_u64() as u32)
                .collect();
            let page = init_leaf_pages[i].clone();
            let range = init_tree.leaf_ranges[i].clone();
            let tmp = self.init_leaf_chips[i].generate_main_trace::<Val<SC>>(
                page,
                commit,
                range,
                self.range_checker.clone(),
            );
            init_leaf_main_trace.push(tmp);
        }

        let mut init_internal_main_trace = vec![];
        for i in 0..init_internal_commitment_arr.len() {
            let commit = init_internal_commitment_arr[i]
                .into_iter()
                .map(|c| c.as_canonical_u64() as u32)
                .collect();
            let page = init_internal_pages[i].clone();
            let range = init_tree.internal_ranges[i].clone();
            let mults = init_tree.mults[i].clone();
            let tmp = self.init_internal_chips[i].generate_main_trace::<Val<SC>>(
                page,
                commit,
                mults,
                range,
                self.range_checker.clone(),
            );
            init_internal_main_trace.push(tmp);
        }

        let mut final_leaf_main_trace = vec![];
        for i in 0..final_leaf_commitment_arr.len() {
            let commit = final_leaf_commitment_arr[i]
                .into_iter()
                .map(|c| c.as_canonical_u64() as u32)
                .collect();
            let page = final_leaf_pages[i].clone();
            let range = final_tree.leaf_ranges[i].clone();
            let tmp = self.final_leaf_chips[i].generate_main_trace::<Val<SC>>(
                page,
                commit,
                range,
                self.range_checker.clone(),
            );
            final_leaf_main_trace.push(tmp);
        }

        let mut final_internal_main_trace = vec![];
        for i in 0..final_internal_commitment_arr.len() {
            let commit = final_internal_commitment_arr[i]
                .into_iter()
                .map(|c| c.as_canonical_u64() as u32)
                .collect();
            let page = final_internal_pages[i].clone();
            let range = final_tree.internal_ranges[i].clone();
            let mults = final_tree.mults[i].clone();
            let tmp = self.final_internal_chips[i].generate_main_trace::<Val<SC>>(
                page,
                commit,
                mults,
                range,
                self.range_checker.clone(),
            );
            final_internal_main_trace.push(tmp);
        }
        let init_root_commitment = if init_root_is_leaf {
            init_leaf_commitment[init_root_idx].clone()
        } else {
            init_internal_commitment[init_root_idx].clone()
        };
        let init_root_commit = init_root_commitment
            .clone()
            .into()
            .into_iter()
            .map(|c| c.as_canonical_u64() as u32)
            .collect();
        let init_root_signal_trace = self.init_root_signal.generate_trace(
            init_root_commit,
            init_tree.root_mult - 1,
            init_tree.root_range.clone(),
        );

        let final_root_commitment = if final_root_is_leaf {
            final_leaf_commitment[final_root_idx].clone()
        } else {
            final_internal_commitment[final_root_idx].clone()
        };
        let final_root_commit = final_root_commitment
            .clone()
            .into()
            .into_iter()
            .map(|c| c.as_canonical_u64() as u32)
            .collect();
        let final_root_signal_trace = self.final_root_signal.generate_trace(
            final_root_commit,
            final_tree.root_mult - 1,
            final_tree.root_range.clone(),
        );

        let offline_checker_trace = self.gen_ops_trace(&init_tree.map, &ops, trace_degree);

        let commitments = PageControllerCommit {
            init_leaf_page_commitments: init_leaf_commitment,
            init_internal_page_commitments: init_internal_commitment,
            final_leaf_page_commitments: final_leaf_commitment,
            final_internal_page_commitments: final_internal_commitment,
            init_root_commitment: init_root_commitment,
            final_root_commitment: final_root_commitment,
        };

        let main_trace = PageControllerMainTrace {
            init_root_signal_trace,
            init_leaf_chip_main_traces: init_leaf_main_trace,
            init_internal_chip_main_traces: init_internal_main_trace,
            offline_checker_trace,
            final_root_signal_trace,
            final_leaf_chip_main_traces: final_leaf_main_trace,
            final_internal_chip_main_traces: final_internal_main_trace,
        };

        let data_trace = PageControllerDataTrace {
            init_leaf_chip_traces: init_leaf_trace,
            init_internal_chip_traces: init_internal_trace,
            final_leaf_chip_traces: final_leaf_trace,
            final_internal_chip_traces: final_internal_trace,
        };

        // self.main_trace = Some(main_trace);
        // self.data_trace = Some(data_trace);
        // self.commit = Some(commitments);

        // tracing::debug!(
        //     "heights of all traces: {} {} {}",
        //     self.init_chip_trace.as_ref().unwrap().height(),
        //     self.offline_checker_trace.as_ref().unwrap().height(),
        //     self.final_chip_trace.as_ref().unwrap().height()
        // );

        // (
        //     vec![
        //         self.init_chip_trace.clone().unwrap(),
        //         self.final_chip_trace.clone().unwrap(),
        //     ],
        //     prover_data,
        // )
        return (data_trace, main_trace, commitments, prover_data);
    }
}
