use std::collections::BTreeMap;
use std::sync::Arc;

use afs_stark_backend::config::Com;
use afs_stark_backend::prover::trace::{ProverTraceData, TraceCommitter};
use p3_field::{AbstractField, PrimeField64};
use p3_matrix::dense::{DenseMatrix, RowMajorMatrix};

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
    pub _root: Node,
    pub mults: Vec<Vec<u32>>,
    pub leaf_ranges: Vec<(Vec<u32>, Vec<u32>)>,
    pub internal_ranges: Vec<(Vec<u32>, Vec<u32>)>,
    pub root_range: (Vec<u32>, Vec<u32>),
    pub root_mult: u32,
    pub _commitment_to_node: BTreeMap<Vec<Val<SC>>, Node>,
    pub mega_page: Vec<Vec<u32>>,
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
        mega_page: &mut Vec<Vec<u32>>,
        mults: &mut Vec<Vec<u32>>,
        idx_len: usize,
        cur_node: Node,
    ) -> u32 {
        if !cur_node.0 {
            let mut ans = 0;
            let mut mult = vec![];
            for row in &internal_pages[cur_node.1] {
                if row[1] == 0 {
                    mult.push(0);
                } else {
                    let f_row: Vec<Val<SC>> = row
                        .iter()
                        .map(|r| Val::<SC>::from_canonical_u32(*r))
                        .collect();
                    let next_node = commitment_to_node
                        .get(&f_row[2 + 2 * idx_len..2 + 2 * idx_len + COMMITMENT_LEN].to_vec());
                    match next_node {
                        None => {
                            mult.push(1);
                            ans += 1;
                        }
                        Some(n) => {
                            if n.0 {
                                leaf_ranges[n.1].0 = row[2..2 + idx_len].to_vec();
                                leaf_ranges[n.1].1 = row[2 + idx_len..2 + 2 * idx_len].to_vec();
                            } else {
                                internal_ranges[n.1].0 = row[2..2 + idx_len].to_vec();
                                internal_ranges[n.1].1 = row[2 + idx_len..2 + 2 * idx_len].to_vec();
                            }
                            let m = PageTreeGraph::<SC, COMMITMENT_LEN>::dfs(
                                leaf_pages,
                                internal_pages,
                                leaf_ranges,
                                internal_ranges,
                                commitment_to_node,
                                mega_page,
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
                if row[1] == 1 {
                    mega_page.push(row[1..].to_vec());
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
                if row[1] == 1 {
                    if leaf_ranges[root.1].0.len() == 0 {
                        leaf_ranges[root.1] =
                            (row[2..2 + idx_len].to_vec(), row[2..2 + idx_len].to_vec());
                    } else {
                        let idx = row[2..2 + idx_len].to_vec();
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
                if row[1] == 1 {
                    let idx1 = row[2..2 + idx_len].to_vec();
                    let idx2 = row[2 + idx_len..2 + 2 * idx_len].to_vec();
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
        let mut mega_page = vec![];
        let root_mult = PageTreeGraph::<SC, COMMITMENT_LEN>::dfs(
            leaf_pages,
            internal_pages,
            &mut leaf_ranges,
            &mut internal_ranges,
            &commitment_to_node,
            &mut mega_page,
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
            _root: root,
            root_range,
            mults,
            root_mult,
            _commitment_to_node: commitment_to_node,
            leaf_ranges,
            internal_ranges,
            mega_page,
        }
    }
}

struct TreeProducts<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub root: RootProducts<SC, COMMITMENT_LEN>,
    pub leaf: NodeProducts<SC, COMMITMENT_LEN>,
    pub internal: NodeProducts<SC, COMMITMENT_LEN>,
}

struct RootProducts<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub main_traces: DenseMatrix<Val<SC>>,
    pub commitments: Com<SC>,
}

struct NodeProducts<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub data_traces: Vec<DenseMatrix<Val<SC>>>,
    pub main_traces: Vec<DenseMatrix<Val<SC>>>,
    pub prover_data: Vec<ProverTraceData<SC>>,
    pub commitments: Vec<Com<SC>>,
    pub commitments_as_arr: Vec<[Val<SC>; COMMITMENT_LEN]>,
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
    pub range_checker: Arc<RangeCheckerGateChip>,
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
                final_param.leaf_cap
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
                final_param.internal_cap
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

    fn gen_ops_trace(
        &self,
        mega_page: &mut Vec<Vec<u32>>,
        ops: &[Operation],
        trace_degree: usize,
    ) -> RowMajorMatrix<Val<SC>> {
        self.offline_checker
            .generate_trace::<SC>(mega_page, ops.to_owned(), trace_degree)
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

        let mut blank_init_leaf_row = vec![1];
        blank_init_leaf_row.resize(2 + self.params.idx_len + self.params.data_len, 0);
        let blank_init_leaf = vec![blank_init_leaf_row.clone(); init_leaf_height];

        let blank_init_internal =
            vec![
                vec![0; 2 + 2 * self.params.idx_len + self.params.commitment_len];
                init_internal_height
            ];
        let mut blank_final_leaf_row = vec![1];
        blank_final_leaf_row.resize(2 + self.params.idx_len + self.params.data_len, 0);
        let blank_final_leaf = vec![blank_final_leaf_row.clone(); final_leaf_height];
        let blank_final_internal =
            vec![
                vec![0; 2 + 2 * self.params.idx_len + self.params.commitment_len];
                final_internal_height
            ];

        let (init_tree_products, mega_page) = make_tree_products(
            trace_committer,
            &init_leaf_pages,
            &self.init_leaf_chips,
            &blank_init_leaf,
            &init_internal_pages,
            &self.init_internal_chips,
            &blank_init_internal,
            &self.init_root_signal,
            &self.params.init_tree_params,
            init_root_is_leaf,
            init_root_idx,
            self.params.idx_len,
            self.range_checker.clone(),
            true,
        );
        let mut mega_page = mega_page.unwrap();
        mega_page.resize(
            mega_page.len() + 3 * ops.len(),
            vec![0; 2 + self.params.idx_len + self.params.data_len],
        );
        let (final_tree_products, _) = make_tree_products(
            trace_committer,
            &final_leaf_pages,
            &self.final_leaf_chips,
            &blank_final_leaf,
            &final_internal_pages,
            &self.final_internal_chips,
            &blank_final_internal,
            &self.final_root_signal,
            &self.params.final_tree_params,
            final_root_is_leaf,
            final_root_idx,
            self.params.idx_len,
            self.range_checker.clone(),
            false,
        );

        let offline_checker_trace = self.gen_ops_trace(&mut mega_page, &ops, trace_degree);

        let data_trace = PageControllerDataTrace {
            init_leaf_chip_traces: init_tree_products.leaf.data_traces,
            init_internal_chip_traces: init_tree_products.internal.data_traces,
            final_leaf_chip_traces: final_tree_products.leaf.data_traces,
            final_internal_chip_traces: final_tree_products.internal.data_traces,
        };
        let main_trace = PageControllerMainTrace {
            init_root_signal_trace: init_tree_products.root.main_traces,
            init_leaf_chip_main_traces: init_tree_products.leaf.main_traces,
            init_internal_chip_main_traces: init_tree_products.internal.main_traces,
            offline_checker_trace: offline_checker_trace,
            final_root_signal_trace: final_tree_products.root.main_traces,
            final_leaf_chip_main_traces: final_tree_products.leaf.main_traces,
            final_internal_chip_main_traces: final_tree_products.internal.main_traces,
        };
        let commitments = PageControllerCommit {
            init_leaf_page_commitments: init_tree_products.leaf.commitments,
            init_internal_page_commitments: init_tree_products.internal.commitments,
            init_root_commitment: init_tree_products.root.commitments,
            final_leaf_page_commitments: final_tree_products.leaf.commitments,
            final_internal_page_commitments: final_tree_products.internal.commitments,
            final_root_commitment: final_tree_products.root.commitments,
        };
        let prover_data = PageControllerProverData {
            init_leaf_page: init_tree_products.leaf.prover_data,
            init_internal_page: init_tree_products.internal.prover_data,
            final_leaf_page: final_tree_products.leaf.prover_data,
            final_internal_page: final_tree_products.internal.prover_data,
        };
        return (data_trace, main_trace, commitments, prover_data);
    }
}

fn make_tree_products<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>(
    committer: &mut TraceCommitter<SC>,
    leaf_pages: &Vec<Vec<Vec<u32>>>,
    leaf_chips: &Vec<LeafPageChip<COMMITMENT_LEN>>,
    blank_leaf_page: &Vec<Vec<u32>>,
    internal_pages: &Vec<Vec<Vec<u32>>>,
    internal_chips: &Vec<InternalPageChip<COMMITMENT_LEN>>,
    blank_internal_page: &Vec<Vec<u32>>,
    root_signal: &RootSignalChip<COMMITMENT_LEN>,
    params: &PageTreeParams,
    root_is_leaf: bool,
    root_idx: usize,
    idx_len: usize,
    range_checker: Arc<RangeCheckerGateChip>,
    make_mega_page: bool,
) -> (TreeProducts<SC, COMMITMENT_LEN>, Option<Vec<Vec<u32>>>)
where
    Val<SC>: AbstractField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    let mut leaf_pages = leaf_pages.clone();
    let mut internal_pages = internal_pages.clone();
    leaf_pages.resize(params.leaf_cap, blank_leaf_page.clone());
    internal_pages.resize(params.internal_cap, blank_internal_page.clone());
    let leaf_trace = leaf_pages
        .iter()
        .zip(leaf_chips.iter())
        .map(|(page, chip)| chip.generate_cached_trace::<Val<SC>>(page.clone()))
        .collect::<Vec<_>>();

    let internal_trace = internal_pages
        .iter()
        .zip(internal_chips.iter())
        .map(|(page, chip)| chip.generate_cached_trace::<Val<SC>>(page.clone()))
        .collect::<Vec<_>>();

    let mut leaf_prods = gen_products(committer, leaf_trace);
    let mut internal_prods = gen_products(committer, internal_trace);

    let tree = PageTreeGraph::<SC, COMMITMENT_LEN>::new(
        &leaf_pages,
        &internal_pages,
        &leaf_prods.commitments,
        &internal_prods.commitments,
        (root_is_leaf, root_idx),
        idx_len,
    );
    for i in 0..leaf_prods.commitments.len() {
        let commit = leaf_prods.commitments_as_arr[i]
            .into_iter()
            .map(|c| c.as_canonical_u64() as u32)
            .collect();
        let page = leaf_pages[i].clone();
        let range = tree.leaf_ranges[i].clone();
        let tmp = leaf_chips[i].generate_main_trace::<Val<SC>>(
            page,
            commit,
            range,
            range_checker.clone(),
        );
        leaf_prods.main_traces.push(tmp);
    }

    for i in 0..internal_prods.commitments.len() {
        let commit = internal_prods.commitments_as_arr[i]
            .into_iter()
            .map(|c| c.as_canonical_u64() as u32)
            .collect();
        let page = internal_pages[i].clone();
        let range = tree.internal_ranges[i].clone();
        let mults = tree.mults[i].clone();
        let tmp = internal_chips[i].generate_main_trace::<Val<SC>>(
            page,
            commit,
            mults,
            range,
            range_checker.clone(),
        );
        internal_prods.main_traces.push(tmp);
    }
    let root_commitment = if root_is_leaf {
        leaf_prods.commitments[root_idx].clone()
    } else {
        internal_prods.commitments[root_idx].clone()
    };
    let root_commit: Vec<u32> = root_commitment
        .clone()
        .into()
        .into_iter()
        .map(|c| c.as_canonical_u64() as u32)
        .collect();
    let root_signal_trace = root_signal.generate_trace::<Val<SC>>(
        root_commit.clone(),
        tree.root_mult - 1,
        tree.root_range.clone(),
    );
    let root_prods = RootProducts {
        main_traces: root_signal_trace,
        commitments: root_commitment,
    };
    let mega_page = if make_mega_page {
        Some(tree.mega_page)
    } else {
        None
    };
    (
        TreeProducts {
            root: root_prods,
            leaf: leaf_prods,
            internal: internal_prods,
        },
        mega_page,
    )
}

fn gen_products<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>(
    committer: &mut TraceCommitter<SC>,
    trace: Vec<RowMajorMatrix<Val<SC>>>,
) -> NodeProducts<SC, COMMITMENT_LEN>
where
    Val<SC>: AbstractField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    let prover_data = data_from_trace(committer, &trace);

    let commitments = commitment_from_data(&prover_data);

    let commitment_arr = arr_from_commitment::<SC, COMMITMENT_LEN>(&commitments);

    NodeProducts {
        data_traces: trace,
        main_traces: vec![],
        prover_data,
        commitments,
        commitments_as_arr: commitment_arr,
    }
}

fn data_from_trace<SC: StarkGenericConfig>(
    committer: &mut TraceCommitter<SC>,
    traces: &Vec<RowMajorMatrix<Val<SC>>>,
) -> Vec<ProverTraceData<SC>> {
    traces
        .iter()
        .map(|trace| committer.commit(vec![trace.clone()]))
        .collect::<Vec<_>>()
}

fn commitment_from_data<SC: StarkGenericConfig>(data: &Vec<ProverTraceData<SC>>) -> Vec<Com<SC>> {
    data.iter()
        .map(|data| data.commit.clone())
        .collect::<Vec<_>>()
}

fn arr_from_commitment<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>(
    commitment: &Vec<Com<SC>>,
) -> Vec<[Val<SC>; COMMITMENT_LEN]>
where
    Val<SC>: AbstractField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    commitment
        .iter()
        .map(|data| data.clone().into())
        .collect::<Vec<[Val<SC>; COMMITMENT_LEN]>>()
}
