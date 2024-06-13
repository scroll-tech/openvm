use afs_stark_backend::config::Com;
use afs_stark_backend::prover::trace::{ProverTraceData, TraceCommitter};
use itertools::Itertools;
use p3_field::{AbstractField, PrimeField64};
use p3_matrix::dense::DenseMatrix;
use p3_uni_stark::{StarkGenericConfig, Val};
use std::collections::BTreeMap;
use std::sync::Mutex;

use super::internal_page_chip::InternalPageChip;
use super::leaf_page_chip::LeafPageChip;
use super::root_signal_chip::RootSignalChip;

#[derive(Clone)]
struct Node(bool, usize);

struct PageTreeGraph<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub _root: Node,
    pub mults: Vec<Vec<u32>>,
    pub root_mult: u32,
    pub _commitment_to_node: BTreeMap<Vec<Val<SC>>, Node>,
}

impl<SC: StarkGenericConfig, const COMMITMENT_LEN: usize> PageTreeGraph<SC, COMMITMENT_LEN>
where
    Val<SC>: AbstractField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    pub fn dfs(
        leaf_pages: &Vec<Vec<Vec<u32>>>,
        internal_pages: &Vec<Vec<Vec<u32>>>,
        commitment_to_node: &BTreeMap<Vec<Val<SC>>, Node>,
        leaf_mults: &Vec<Vec<u32>>,
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
                            mult.push(0);
                        }
                        Some(n) => {
                            let m = PageTreeGraph::<SC, COMMITMENT_LEN>::dfs(
                                leaf_pages,
                                internal_pages,
                                commitment_to_node,
                                &leaf_mults,
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
            return ans;
        } else {
            let mut ans = 0;
            for (i, row) in leaf_pages[cur_node.1].iter().enumerate() {
                if row[1] == 1 {
                    ans += leaf_mults[cur_node.1][i];
                }
            }
            return ans;
        }
    }

    pub fn new(
        leaf_mults: &Vec<Vec<u32>>,
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
        for (i, c) in leaf_commits.iter().enumerate() {
            commitment_to_node.insert(c.clone().to_vec(), Node(true, i));
        }
        for (i, c) in internal_commits.iter().enumerate() {
            commitment_to_node.insert(c.clone().to_vec(), Node(false, i));
        }
        let mut mults = vec![vec![0; internal_pages[0].len()]; internal_pages.len()];
        commitment_to_node.insert(root_commitment.clone().to_vec(), root.clone());
        let root_mult = PageTreeGraph::<SC, COMMITMENT_LEN>::dfs(
            leaf_pages,
            internal_pages,
            &commitment_to_node,
            &leaf_mults,
            &mut mults,
            idx_len,
            root.clone(),
        );
        PageTreeGraph {
            _root: root,
            mults,
            root_mult,
            _commitment_to_node: commitment_to_node,
        }
    }
}

#[derive(Clone)]
pub struct PageTreeParams {
    pub path_bus_index: usize,
    pub leaf_cap: usize,
    pub internal_cap: usize,
    pub leaf_page_height: usize,
    pub internal_page_height: usize,
}

pub struct TreeProducts<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub root: RootProducts<SC, COMMITMENT_LEN>,
    pub leaf: NodeProducts<SC, COMMITMENT_LEN>,
    pub internal: NodeProducts<SC, COMMITMENT_LEN>,
}

pub struct RootProducts<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: AbstractField + PrimeField64,
{
    pub main_traces: DenseMatrix<Val<SC>>,
    pub commitments: Com<SC>,
}

pub struct NodeProducts<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
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
pub struct PageControllerParams {
    pub idx_len: usize,
    pub data_len: usize,
    pub commitment_len: usize,
    pub tree_params: PageTreeParams,
}

pub struct PageController<const COMMITMENT_LEN: usize> {
    pub root_signal: RootSignalChip<COMMITMENT_LEN>,
    pub leaf_chips: Vec<LeafPageChip<COMMITMENT_LEN>>,
    pub internal_chips: Vec<InternalPageChip<COMMITMENT_LEN>>,
    pub map: BTreeMap<Vec<u32>, Vec<u32>>,
    pub params: PageControllerParams,
    pub request_count: Mutex<BTreeMap<Vec<u32>, u32>>,
}

impl<const COMMITMENT_LEN: usize> PageController<COMMITMENT_LEN> {
    pub fn new(
        data_bus_index: usize,
        idx_len: usize,
        data_len: usize,
        tree_param: PageTreeParams,
    ) -> Self {
        PageController {
            root_signal: RootSignalChip::new(tree_param.path_bus_index),
            leaf_chips: vec![
                LeafPageChip::new(
                    data_bus_index,
                    tree_param.path_bus_index,
                    idx_len,
                    data_len
                );
                tree_param.leaf_cap
            ],
            internal_chips: vec![
                InternalPageChip::new(tree_param.path_bus_index, idx_len);
                tree_param.internal_cap
            ],
            params: PageControllerParams {
                idx_len,
                data_len,
                commitment_len: COMMITMENT_LEN,
                tree_params: tree_param,
            },
            request_count: Mutex::new(BTreeMap::new()),
            map: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.request_count.lock().unwrap().clear();
        self.map.clear();
    }

    pub fn load_pages(&mut self, leaf_pages: &Vec<Vec<Vec<u32>>>) {
        for leaf_page in leaf_pages {
            for row in leaf_page {
                if row[1] == 1 {
                    self.map.insert(
                        row[2..2 + self.params.idx_len].to_vec(),
                        row[2 + self.params.idx_len
                            ..2 + self.params.idx_len + self.params.data_len]
                            .to_vec(),
                    );
                }
            }
        }
    }

    pub fn generate_trace_data<SC: StarkGenericConfig>(
        &mut self,
        trace_committer: &mut TraceCommitter<SC>,
        leaf_pages: Vec<Vec<Vec<u32>>>,
        internal_pages: Vec<Vec<Vec<u32>>>,
        root_is_leaf: bool,
        root_idx: usize,
    ) -> TreeProducts<SC, COMMITMENT_LEN>
    where
        Val<SC>: AbstractField + PrimeField64,
        Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
    {
        let mut leaf_pages = leaf_pages.clone();
        let mut internal_pages = internal_pages.clone();
        let mut blank_leaf_row = vec![1];
        blank_leaf_row.resize(2 + self.params.idx_len + self.params.data_len, 0);
        let blank_internal_row = vec![0; 2 + 2 * self.params.idx_len + COMMITMENT_LEN];
        let blank_leaf_page =
            vec![blank_leaf_row.clone(); self.params.tree_params.leaf_page_height];
        let blank_internal_page =
            vec![blank_internal_row.clone(); self.params.tree_params.internal_page_height];
        leaf_pages.resize(self.params.tree_params.leaf_cap, blank_leaf_page.clone());
        internal_pages.resize(
            self.params.tree_params.internal_cap,
            blank_internal_page.clone(),
        );
        let mut leaf_mults = vec![];
        for leaf in &leaf_pages {
            let mut mult = vec![];
            for row in leaf {
                if row[1] == 0 {
                    mult.push(0);
                } else {
                    let idx = row[2..2 + self.params.idx_len].to_vec();
                    let request_count = self.request_count.lock().unwrap();
                    let count = request_count.get(&idx);
                    if let Some(count) = count {
                        mult.push(*count);
                    } else {
                        mult.push(0);
                    }
                }
            }
            leaf_mults.push(mult);
        }
        let leaf_trace = leaf_pages
            .iter()
            .zip(&self.leaf_chips)
            .map(|(page, chip)| chip.generate_cached_trace::<SC>(page.clone()))
            .collect_vec();
        let internal_trace = internal_pages
            .iter()
            .zip(&self.internal_chips)
            .map(|(page, chip)| chip.generate_cached_trace::<SC>(page.clone()))
            .collect_vec();
        let leaf_prover_data: Vec<ProverTraceData<SC>> = leaf_trace
            .iter()
            .map(|trace| trace_committer.commit(vec![trace.clone()]))
            .collect_vec();
        let internal_prover_data: Vec<ProverTraceData<SC>> = internal_trace
            .iter()
            .map(|trace| trace_committer.commit(vec![trace.clone()]))
            .collect_vec();
        let leaf_commits = leaf_prover_data
            .iter()
            .map(|d| d.commit.clone())
            .collect::<Vec<_>>();
        let internal_commits = internal_prover_data
            .iter()
            .map(|d| d.commit.clone())
            .collect::<Vec<_>>();
        let tree = PageTreeGraph::<SC, COMMITMENT_LEN>::new(
            &leaf_mults,
            &leaf_pages,
            &internal_pages,
            &leaf_commits,
            &internal_commits,
            (root_is_leaf, root_idx),
            self.params.idx_len,
        );
        let root_mult = tree.root_mult;
        let mults = tree.mults;
        let root_commit = if root_is_leaf {
            leaf_commits[root_idx].clone()
        } else {
            internal_commits[root_idx].clone()
        };
        let leaf_commit_arr: Vec<[Val<SC>; COMMITMENT_LEN]> =
            leaf_commits.iter().map(|c| c.clone().into()).collect_vec();
        let leaf_commit_u32 = leaf_commit_arr
            .iter()
            .map(|f| {
                f.into_iter()
                    .map(|f| f.as_canonical_u64() as u32)
                    .collect_vec()
            })
            .collect_vec();

        let internal_commit_arr: Vec<[Val<SC>; COMMITMENT_LEN]> = internal_commits
            .iter()
            .map(|c| c.clone().into())
            .collect_vec();
        let internal_commit_u32 = internal_commit_arr
            .iter()
            .map(|f| {
                f.into_iter()
                    .map(|f| f.as_canonical_u64() as u32)
                    .collect_vec()
            })
            .collect_vec();

        let root_commit_u32 = if root_is_leaf {
            leaf_commit_u32[root_idx].clone()
        } else {
            internal_commit_u32[root_idx].clone()
        };
        let root_trace = self
            .root_signal
            .generate_trace::<Val<SC>>(root_commit_u32, root_mult);
        let root_prods: RootProducts<SC, COMMITMENT_LEN> = RootProducts {
            main_traces: root_trace,
            commitments: root_commit,
        };
        let mut leaf_main_trace = vec![];
        for i in 0..leaf_pages.len() {
            let main_trace = self.leaf_chips[i].generate_main_trace::<SC>(
                leaf_pages[i].clone(),
                leaf_commit_u32[i].clone(),
                leaf_mults[i].clone(),
            );
            leaf_main_trace.push(main_trace);
        }
        let mut internal_main_trace = vec![];
        for i in 0..internal_pages.len() {
            let main_trace = self.internal_chips[i].generate_main_trace::<SC>(
                internal_pages[i].clone(),
                internal_commit_u32[i].clone(),
                mults[i].clone(),
            );
            internal_main_trace.push(main_trace);
        }
        let leaf_prods: NodeProducts<SC, COMMITMENT_LEN> = NodeProducts {
            data_traces: leaf_trace,
            main_traces: leaf_main_trace,
            prover_data: leaf_prover_data,
            commitments: leaf_commits,
            commitments_as_arr: leaf_commit_arr,
        };
        let internal_prods: NodeProducts<SC, COMMITMENT_LEN> = NodeProducts {
            data_traces: internal_trace,
            main_traces: internal_main_trace,
            prover_data: internal_prover_data,
            commitments: internal_commits,
            commitments_as_arr: internal_commit_arr,
        };
        TreeProducts {
            root: root_prods,
            leaf: leaf_prods,
            internal: internal_prods,
        }
    }

    pub fn request(&self, idx: &Vec<u32>) -> Option<Vec<u32>> {
        let mut request_count = self.request_count.lock().unwrap();
        let data = self.map.get(idx);
        if let Some(d) = data {
            if let Some(c) = request_count.get(idx).cloned() {
                request_count.insert(idx.clone(), c + 1);
            } else {
                request_count.insert(idx.clone(), 1);
            }
            Some(d.clone())
        } else {
            None
        }
    }
}
