use crate::{
    page1_read::Page1ReadChip, page2_read::Page2ReadChip, page_node::PageNodeReadChip,
    MAX_COMMITMENT_LEN,
};
use afs_stark_backend::{
    config::Com,
    prover::trace::{ProverTraceData, TraceCommitter},
};
use afs_test_utils::utils::create_seeded_rng;
use p3_field::{AbstractField, PrimeField32};
use p3_matrix::dense::DenseMatrix;
use p3_uni_stark::{StarkGenericConfig, Val};
use rand::Rng;
use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicU32, Arc},
};
use types::PageNode;

pub mod tests;
pub mod trace;
pub mod types;

pub struct Page2Controller<SC: StarkGenericConfig> {
    val_bus_index: usize,
    path_bus_index: usize,
    pub page_read_chip: Vec<Arc<PageNode>>,
    key_len: usize,
    val_len: usize,
    limb_bits: usize,
    request_counts: Vec<Vec<Arc<AtomicU32>>>,
    is_p: Vec<Vec<u32>>,
    is_t: Vec<Vec<u32>>,
    page_commitments: Vec<Com<SC>>,
    adj: Vec<Vec<usize>>,
    parent: Vec<(usize, usize)>,
    find_node: BTreeMap<Vec<u32>, Vec<(usize, usize)>>,
    find_val: BTreeMap<Vec<u32>, Vec<Vec<u32>>>,
    root_commitment: Vec<u32>,
    known_structure: bool,
}

impl<SC: StarkGenericConfig> Page2Controller<SC> {
    pub fn new(
        val_bus_index: usize,
        path_bus_index: usize,
        key_len: usize,
        val_len: usize,
        limb_bits: usize,
        known_structure: bool,
    ) -> Self {
        Page2Controller {
            val_bus_index,
            path_bus_index,
            page_read_chip: vec![],
            is_p: vec![],
            is_t: vec![],
            key_len,
            val_len,
            limb_bits,
            request_counts: vec![],
            page_commitments: vec![],
            adj: vec![],
            parent: vec![],
            find_node: BTreeMap::new(),
            find_val: BTreeMap::new(),
            root_commitment: vec![],
            known_structure,
        }
    }

    pub fn path_bus_index(&self) -> usize {
        self.path_bus_index
    }

    pub fn val_bus_index(&self) -> usize {
        self.val_bus_index
    }

    pub fn key_len(&self) -> usize {
        self.key_len
    }

    pub fn val_len(&self) -> usize {
        self.val_len
    }

    pub fn clear(&mut self) {
        self.request_counts.clear();
        self.is_p.clear();
        self.is_t.clear();
        self.page_commitments.clear();
        self.adj.clear();
        self.parent.clear();
        self.find_node.clear();
        self.find_val.clear();
        self.root_commitment.clear();
    }

    pub fn load_pages(
        &mut self,
        trace_committer: &mut TraceCommitter<SC>,
        // assume pages are topologically sorted or something
        known_structure: bool,
        pages: &Vec<Vec<Vec<u32>>>,
    ) -> (Vec<DenseMatrix<Val<SC>>>, Vec<ProverTraceData<SC>>)
    where
        Com<SC>: Into<[Val<SC>; MAX_COMMITMENT_LEN]>,
        Val<SC>: PrimeField32,
    {
        println!("START");
        self.known_structure = known_structure;
        let mut nodes = Vec::new();
        let mut commitments = Vec::new();
        let mut traces = Vec::new();
        let mut commitment_as_f: Vec<[Val<SC>; MAX_COMMITMENT_LEN]> = Vec::new();
        for j in 0..pages.len() {
            let page = &pages[j];
            let mut is_t = true;
            let mut is_b = true;
            self.request_counts.push(
                (0..page.len())
                    .map(|_| Arc::new(AtomicU32::new(0)))
                    .collect(),
            );
            self.is_p.push(vec![]);
            self.is_t.push(vec![]);
            for i in 0..page.len() {
                self.is_p[j].push(page[i][0]);
                self.is_t[j].push(page[i][1]);
                if page[i][0] == 1 && page[i][1] == 0 {
                    is_t = false;
                }
                if page[i][0] == 1 && page[i][1] == 1 {
                    is_b = false;
                }
            }
            let page_read_chip = if is_t && known_structure {
                println!("TERMINAL");
                PageNode::PageTerminal(Page1ReadChip::new(
                    self.val_bus_index,
                    self.path_bus_index,
                    self.key_len,
                    self.limb_bits,
                    self.val_len,
                    page.clone(),
                ))
            } else if is_b && known_structure {
                println!("BRANCH");
                PageNode::PageBranch(Page2ReadChip::new(
                    self.path_bus_index,
                    self.key_len,
                    self.limb_bits,
                    page.clone(),
                ))
            } else {
                println!("MIXED");
                PageNode::PageMixed(PageNodeReadChip::new(
                    self.val_bus_index,
                    self.path_bus_index,
                    self.key_len,
                    self.val_len,
                    self.limb_bits,
                    page.clone(),
                ))
            };
            nodes.push(Arc::new(page_read_chip));
            let page_trace = match nodes.last().unwrap().to_owned().as_ref() {
                PageNode::PageTerminal(p) => p.get_page_trace::<SC>(),
                PageNode::PageBranch(p) => p.get_page_trace::<SC>(),
                PageNode::PageMixed(p) => p.get_page_trace::<SC>(),
            };
            let commitment = trace_committer.commit(vec![page_trace.clone()]);
            traces.push(page_trace);
            commitment_as_f.push(commitment.commit.clone().into());
            commitments.push(commitment);
        }
        self.parent = vec![(usize::MAX, usize::MAX); pages.len()];
        self.setup_graph(&pages, &commitment_as_f);
        // self.requests = vec![];
        // create the graph?
        self.page_commitments = commitments.iter().map(|c| c.commit.clone()).collect();
        let com: [Val<SC>; MAX_COMMITMENT_LEN] = self.page_commitments[0].clone().into();
        let com: Vec<u32> = com.into_iter().map(|c| c.as_canonical_u32()).collect();
        self.root_commitment = com;
        self.page_read_chip = nodes;
        (traces, commitments)
    }

    fn setup_graph(
        &mut self,
        pages: &Vec<Vec<Vec<u32>>>,
        commitment_as_f: &Vec<[Val<SC>; MAX_COMMITMENT_LEN]>,
    ) {
        for k in 0..pages.len() {
            self.adj.push(vec![]);
            for l in 0..pages[k].len() {
                let row = &pages[k][l];
                if row[0] == 1 && row[1] == 0 {
                    let mut match_found = false;
                    for j in 0..pages.len() {
                        let mut is_match = true;
                        for i in 0..MAX_COMMITMENT_LEN {
                            if commitment_as_f[j][i]
                                != Val::<SC>::from_canonical_u32(row[2 + 2 * self.key_len + i])
                            {
                                is_match = false;
                            }
                        }
                        if is_match {
                            match_found = true;
                            self.adj[k].push(j);
                            self.parent[j] = (k, l);
                            break;
                        }
                    }
                    if !match_found {
                        self.adj[k].push(usize::MAX);
                    }
                } else {
                    self.adj[k].push(usize::MAX);
                }
                if row[0] == 1 && row[1] == 1 {
                    if !self
                        .find_node
                        .contains_key(&row[2..2 + self.key_len].to_vec())
                    {
                        self.find_node
                            .insert(row[2..2 + self.key_len].to_vec(), Vec::new());
                        self.find_val
                            .insert(row[2..2 + self.key_len].to_vec(), Vec::new());
                    }
                    self.find_node
                        .get_mut(&row[2..2 + self.key_len].to_vec())
                        .unwrap()
                        .push((k, l));
                    self.find_val
                        .get_mut(&row[2..2 + self.key_len].to_vec())
                        .unwrap()
                        .push(row[2 + self.key_len..2 + self.key_len + self.val_len].to_vec());
                }
            }
        }
    }

    pub fn root_commitment(&self) -> Vec<u32> {
        return self.root_commitment.clone();
    }

    // returns val
    pub fn request(&self, key: &Vec<u32>) -> Vec<u32>
    where
        Com<SC>: Into<[Val<SC>; MAX_COMMITMENT_LEN]>,
        Val<SC>: PrimeField32,
    {
        let mut rng = create_seeded_rng();
        let nodes = match self.find_node.get(key) {
            Some(n) => n,
            None => return vec![0; self.val_len],
        };
        let idx = rng.gen::<usize>() % nodes.len();
        let mut cur = nodes[idx];
        let vals = self.find_val.get(key).unwrap();
        let val = &vals[idx];
        while cur.0 != usize::MAX {
            self.request_counts[cur.0][cur.1].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            // println!("OLD CUR: {:?}", cur.0);
            cur = self.parent[cur.0];
            // println!("NEW CUR: {:?}", cur.0);
        }
        return val.clone();
        // let mut path_cols = Vec::new();
        // let mut val_cols = Vec::new();
        // for k in key {
        //     val_cols.push(VirtualPairCol::single_main(k as usize));
        // }
        // for v in val {
        //     val_cols.push(VirtualPairCol::single_main(v as usize));
        // }
        // let com: [Val<SC>; MAX_COMMITMENT_LEN] = self.page_commitments[0].clone().into();
        // let com: Vec<u32> = com.into_iter().map(|c| c.as_canonical_u32()).collect();
        // for c in com {
        //     path_cols.push(VirtualPairCol::single_main(c as usize));
        // }
        // return (
        //     Interaction {
        //         fields: path_cols,
        //         count: VirtualPairCol::constant(1),
        //         argument_index: self.path_bus_index(),
        //     },
        //     Interaction {
        //         fields: val_cols,
        //         count: VirtualPairCol::constant(1),
        //         argument_index: self.path_bus_index(),
        //     },
        // );
    }
}
