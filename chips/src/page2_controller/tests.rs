use crate::page2_requester::Page2Requester;
use crate::MAX_COMMITMENT_LEN;
use afs_stark_backend::prover::MultiTraceStarkProver;
use afs_stark_backend::rap::AnyRap;
use afs_stark_backend::verifier::VerificationError;
use afs_stark_backend::{
    keygen::{types::MultiStarkPartialProvingKey, MultiStarkKeygenBuilder},
    prover::trace::TraceCommitmentBuilder,
};
use afs_test_utils::config::{
    self,
    baby_bear_poseidon2::{BabyBearPoseidon2Config, BabyBearPoseidon2Engine},
};
use afs_test_utils::{engine::StarkEngine, utils::create_seeded_rng};
use p3_baby_bear::BabyBear;
use rand::Rng;

use super::types::PageNode;
use super::Page2Controller;

fn load_pages_test(
    engine: &BabyBearPoseidon2Engine,
    pages: &Vec<Vec<Vec<u32>>>,
    page_controller: &mut Page2Controller<BabyBearPoseidon2Config>,
    known_structure: bool,
    page_requester: &Page2Requester,
    trace_builder: &mut TraceCommitmentBuilder<BabyBearPoseidon2Config>,
    partial_pk: &MultiStarkPartialProvingKey<BabyBearPoseidon2Config>,
) -> Result<(), VerificationError> {
    page_controller.clear();
    let (page_trace, prover_data) =
        page_controller.load_pages(&mut trace_builder.committer, known_structure, &pages);
    let requester_trace = page_requester.generate_trace(&page_controller);
    let page_metadata_trace = page_controller.generate_trace();
    trace_builder.clear();
    for (trace, data) in page_trace.into_iter().zip(prover_data.into_iter()) {
        trace_builder.load_cached_trace(trace, data);
    }
    for mtrace in page_metadata_trace.into_iter() {
        trace_builder.load_trace(mtrace);
    }
    trace_builder.load_trace(requester_trace);

    trace_builder.commit_current();

    let partial_vk = partial_pk.partial_vk();
    let mut vec: Vec<&dyn AnyRap<BabyBearPoseidon2Config>> = Vec::new();
    for chip in &page_controller.page_read_chip {
        vec.push(match chip.as_ref() {
            PageNode::PageTerminal(p) => p,
            PageNode::PageBranch(p) => p,
            PageNode::PageMixed(p) => p,
        });
    }

    vec.push(page_requester);

    let main_trace_data = trace_builder.view(&partial_vk, vec.clone());

    let mut pis: Vec<Vec<BabyBear>> = vec![vec![]; partial_vk.per_air.len()];

    for i in 0..page_controller.page_commitments.len() {
        let commit: [BabyBear; MAX_COMMITMENT_LEN] = page_controller.page_commitments[i].into();
        pis[i] = commit.to_vec();
    }

    let prover = engine.prover();
    let verifier = engine.verifier();

    let mut challenger = engine.new_challenger();
    let proof = prover.prove(&mut challenger, &partial_pk, main_trace_data, &pis);

    let mut challenger = engine.new_challenger();
    let result = verifier.verify(&mut challenger, partial_vk, vec, proof, &pis);

    result
}

#[test]
fn page2_read_chip_test_known_structure() {
    let mut rng = create_seeded_rng();
    let val_bus_index = 0;
    let path_bus_index = 1;
    let limb_bits = 20;
    let key_len = 3;
    let val_len = 5;

    let log_page_height = 3;
    let log_num_requests = 5;

    let page_height = 1 << log_page_height;
    let num_requests: usize = 1 << log_num_requests;
    let blank_row = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let terminal = |key: [u32; 3], value: [u32; 5]| {
        let mut ans = vec![1, 1];
        ans.append(&mut key.to_vec());
        ans.append(&mut value.to_vec());
        ans
    };
    let branch = |key_start: [u32; 3], key_end: [u32; 3], c_com: [u32; MAX_COMMITMENT_LEN]| {
        let mut ans = vec![1, 0];
        ans.append(&mut key_start.to_vec());
        ans.append(&mut key_end.to_vec());
        ans.append(&mut c_com.to_vec());
        ans
    };
    let leaf_com = [
        985898477, 247250916, 562999272, 1443815223, 747441248, 959094834, 532757203, 680207284,
    ];
    let leaf_com2 = [
        185806796, 1416399212, 1976521025, 804407964, 1444409902, 1621285524, 935044135, 1444008045,
    ];
    let tier2_com = [
        222297491, 1376978040, 1409755820, 1447813549, 630530852, 833973000, 1400496302, 1604954841,
    ];
    let max_len = MAX_COMMITMENT_LEN + 2 * key_len + 2;
    // note that max_len = branch len, so we pad branches and intermediates
    let pad = |page: Vec<Vec<u32>>| {
        let mut new_page = vec![];
        for row in &page {
            let mut new_row = vec![];
            for i in 0..max_len {
                if i < row.len() {
                    new_row.push(row[i]);
                } else {
                    new_row.push(0);
                }
            }
            new_page.push(new_row);
        }
        new_page
    };
    let branch_page = pad(vec![
        blank_row.clone(),
        blank_row.clone(),
        branch([0, 1, 1], [3, 1, 1], leaf_com),
        branch([0, 1, 1], [3, 1, 1], tier2_com),
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
    ]);
    let leaf_page = vec![
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        terminal([3, 1, 1], [0, 1, 1, 1, 1]),
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
    ];
    let leaf_page2 = vec![
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        terminal([2, 1, 1], [0, 1, 1, 1, 1]),
        blank_row.clone(),
        blank_row.clone(),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
    ];
    let tier2_node = pad(vec![
        branch([0, 1, 1], [3, 1, 1], leaf_com.clone()),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([3, 1, 1], [0, 1, 1, 1, 1]),
        blank_row.clone(),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
    ]);
    let tier2_node2 = pad(vec![
        branch([0, 1, 1], [3, 1, 1], leaf_com2.clone()),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 0, 0], [0, 0, 0, 0, 0]),
        terminal([3, 1, 1], [0, 1, 1, 1, 1]),
        blank_row.clone(),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
    ]);
    let blank_page = vec![blank_row.clone(); page_height];
    let pages = vec![tier2_node2.clone(), leaf_page2.clone(), blank_page.clone()];
    let good_keys: Vec<Vec<u32>> = vec![vec![0, 1, 1], vec![2, 1, 1], vec![0, 0, 0], vec![3, 1, 1]];
    let mut page_controller = Page2Controller::new(
        val_bus_index,
        path_bus_index,
        key_len,
        val_len,
        limb_bits,
        true,
    );
    let good_requests = (0..num_requests)
        .map(|_| good_keys[rng.gen::<usize>() % good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    let mut page_requester = Page2Requester::new(
        path_bus_index,
        val_bus_index,
        key_len,
        val_len,
        good_requests,
    );
    let engine = config::baby_bear_poseidon2::default_engine(log_page_height.max(log_num_requests));
    let prover = MultiTraceStarkProver::new(&engine.config);
    let mut trace_builder = TraceCommitmentBuilder::new(prover.pcs());
    page_controller.load_pages(&mut trace_builder.committer, true, &pages);
    let mut keygen_builder = MultiStarkKeygenBuilder::new(&engine.config);
    let mut page_data_ptrs = vec![];
    for i in 0..pages.len() {
        let page_data_ptr = keygen_builder.add_cached_main_matrix(pages[i][0].len());
        page_data_ptrs.push(page_data_ptr);
    }
    for i in 0..pages.len() {
        let page_metadata_ptr = keygen_builder.add_main_matrix(MAX_COMMITMENT_LEN + 4);
        match page_controller.page_read_chip[i].as_ref() {
            PageNode::PageTerminal(p) => {
                keygen_builder.add_partitioned_air(
                    p,
                    page_height,
                    MAX_COMMITMENT_LEN,
                    vec![page_data_ptrs[i], page_metadata_ptr],
                );
            }
            PageNode::PageBranch(p) => {
                keygen_builder.add_partitioned_air(
                    p,
                    page_height,
                    MAX_COMMITMENT_LEN,
                    vec![page_data_ptrs[i], page_metadata_ptr],
                );
            }
            PageNode::PageMixed(p) => {
                keygen_builder.add_partitioned_air(
                    p,
                    page_height,
                    MAX_COMMITMENT_LEN,
                    vec![page_data_ptrs[i], page_metadata_ptr],
                );
            }
        }
    }

    keygen_builder.add_air(&page_requester, num_requests, 0);

    let partial_pk = keygen_builder.generate_partial_pk();
    let mut test_pages = vec![tier2_node.clone(), leaf_page2.clone(), blank_page.clone()];
    let mut test_good_keys: Vec<Vec<u32>> = vec![vec![0, 1, 1], vec![3, 1, 1]];
    let mut test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        true,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");

    test_pages = vec![branch_page.clone(), leaf_page.clone(), tier2_node.clone()];
    test_good_keys = vec![vec![0, 1, 1], vec![3, 1, 1]];
    test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        true,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");

    test_pages = vec![tier2_node.clone(), blank_page.clone(), leaf_page.clone()];
    test_good_keys = vec![vec![0, 1, 1], vec![3, 1, 1]];
    test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        true,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");

    test_pages = vec![tier2_node2.clone(), blank_page.clone(), leaf_page2.clone()];
    test_good_keys = vec![vec![0, 1, 1], vec![2, 1, 1], vec![0, 0, 0], vec![3, 1, 1]];
    test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        true,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");

    test_pages = vec![tier2_node.clone(), blank_page.clone(), leaf_page.clone()];
    test_good_keys = vec![vec![0, 0, 0], vec![2, 1, 1]];
    test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    let result = load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        true,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    );

    assert_eq!(
        result,
        Err(VerificationError::NonZeroCumulativeSum),
        "Verification failed"
    );

    test_pages = vec![tier2_node.clone(), blank_page.clone(), leaf_page2.clone()];
    test_good_keys = vec![vec![0, 0, 0], vec![2, 1, 1]];
    test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    let result = load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        true,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    );

    assert_eq!(
        result,
        Err(VerificationError::NonZeroCumulativeSum),
        "Verification failed"
    );
}

#[test]
fn page2_read_chip_test_unknown_structure() {
    let mut rng = create_seeded_rng();
    let val_bus_index = 0;
    let path_bus_index = 1;
    let limb_bits = 20;
    let key_len = 3;
    let val_len = 5;

    let log_page_height = 3;
    let log_num_requests = 5;

    let page_height = 1 << log_page_height;
    let num_requests: usize = 1 << log_num_requests;
    let blank_row = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let terminal = |key: [u32; 3], value: [u32; 5]| {
        let mut ans = vec![1, 1];
        ans.append(&mut key.to_vec());
        ans.append(&mut value.to_vec());
        ans
    };
    let branch = |key_start: [u32; 3], key_end: [u32; 3], c_com: [u32; MAX_COMMITMENT_LEN]| {
        let mut ans = vec![1, 0];
        ans.append(&mut key_start.to_vec());
        ans.append(&mut key_end.to_vec());
        ans.append(&mut c_com.to_vec());
        ans
    };
    let leaf_com = [
        735866684, 122052378, 1169046940, 842530491, 1188434811, 47918533, 776027805, 633481103,
    ];
    let leaf_com2 = [
        441971113, 1642090721, 1041393637, 1407056607, 1936201440, 242063937, 810354545, 750040442,
    ];
    let tier2_com = [
        1066552960, 619340561, 458960951, 1761437705, 1747619853, 1342650098, 925191448, 1838952520,
    ];
    let max_len = MAX_COMMITMENT_LEN + 2 * key_len + 2;
    // note that max_len = branch len, so we pad branches and intermediates
    let pad = |page: Vec<Vec<u32>>| {
        let mut new_page = vec![];
        for row in &page {
            let mut new_row = vec![];
            for i in 0..max_len {
                if i < row.len() {
                    new_row.push(row[i]);
                } else {
                    new_row.push(0);
                }
            }
            new_page.push(new_row);
        }
        new_page
    };
    let branch_page = pad(vec![
        blank_row.clone(),
        blank_row.clone(),
        branch([0, 1, 1], [3, 1, 1], leaf_com),
        branch([0, 1, 1], [3, 1, 1], tier2_com),
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
    ]);
    let leaf_page = pad(vec![
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        terminal([3, 1, 1], [0, 1, 1, 1, 1]),
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
    ]);
    let leaf_page2 = pad(vec![
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        blank_row.clone(),
        terminal([2, 1, 1], [0, 1, 1, 1, 1]),
        blank_row.clone(),
        blank_row.clone(),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
    ]);
    let tier2_node = pad(vec![
        branch([0, 1, 1], [3, 1, 1], leaf_com.clone()),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([3, 1, 1], [0, 1, 1, 1, 1]),
        blank_row.clone(),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
    ]);
    let tier2_node2 = pad(vec![
        branch([0, 1, 1], [3, 1, 1], leaf_com2.clone()),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 0, 0], [0, 0, 0, 0, 0]),
        terminal([3, 1, 1], [0, 1, 1, 1, 1]),
        blank_row.clone(),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
        terminal([0, 1, 1], [1, 1, 1, 1, 1]),
    ]);
    let blank_page = pad(vec![blank_row.clone(); page_height]);
    let pages = vec![tier2_node2.clone(), leaf_page2.clone(), blank_page.clone()];
    let good_keys: Vec<Vec<u32>> = vec![vec![0, 1, 1], vec![2, 1, 1], vec![0, 0, 0], vec![3, 1, 1]];

    let mut page_controller = Page2Controller::new(
        val_bus_index,
        path_bus_index,
        key_len,
        val_len,
        limb_bits,
        false,
    );
    let good_requests = (0..num_requests)
        .map(|_| good_keys[rng.gen::<usize>() % good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    let mut page_requester = Page2Requester::new(
        path_bus_index,
        val_bus_index,
        key_len,
        val_len,
        good_requests,
    );
    let engine = config::baby_bear_poseidon2::default_engine(log_page_height.max(log_num_requests));
    let prover = MultiTraceStarkProver::new(&engine.config);
    let mut trace_builder = TraceCommitmentBuilder::new(prover.pcs());
    page_controller.load_pages(&mut trace_builder.committer, false, &pages);
    let mut keygen_builder = MultiStarkKeygenBuilder::new(&engine.config);
    let mut page_data_ptrs = vec![];
    for i in 0..pages.len() {
        let page_data_ptr = keygen_builder.add_cached_main_matrix(pages[i][0].len());
        page_data_ptrs.push(page_data_ptr);
    }
    for i in 0..pages.len() {
        let page_metadata_ptr = keygen_builder.add_main_matrix(MAX_COMMITMENT_LEN + 4);
        match page_controller.page_read_chip[i].as_ref() {
            PageNode::PageTerminal(p) => {
                keygen_builder.add_partitioned_air(
                    p,
                    page_height,
                    MAX_COMMITMENT_LEN,
                    vec![page_data_ptrs[i], page_metadata_ptr],
                );
            }
            PageNode::PageBranch(p) => {
                keygen_builder.add_partitioned_air(
                    p,
                    page_height,
                    MAX_COMMITMENT_LEN,
                    vec![page_data_ptrs[i], page_metadata_ptr],
                );
            }
            PageNode::PageMixed(p) => {
                keygen_builder.add_partitioned_air(
                    p,
                    page_height,
                    MAX_COMMITMENT_LEN,
                    vec![page_data_ptrs[i], page_metadata_ptr],
                );
            }
        }
    }

    keygen_builder.add_air(&page_requester, num_requests, 0);

    let partial_pk = keygen_builder.generate_partial_pk();
    let mut test_pages = vec![leaf_page2.clone(), blank_page.clone(), blank_page.clone()];
    let mut test_good_keys: Vec<Vec<u32>> = vec![vec![0, 1, 1], vec![2, 1, 1]];
    let mut test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        false,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");

    test_pages = vec![branch_page.clone(), tier2_node.clone(), leaf_page.clone()];
    test_good_keys = vec![vec![0, 1, 1], vec![3, 1, 1]];
    test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        false,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");

    test_pages = vec![tier2_node2.clone(), blank_page.clone(), leaf_page2.clone()];
    test_good_keys = vec![vec![0, 1, 1], vec![2, 1, 1], vec![0, 0, 0], vec![3, 1, 1]];
    test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        false,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");

    test_pages = vec![tier2_node.clone(), blank_page.clone(), leaf_page.clone()];
    test_good_keys = vec![vec![0, 0, 0], vec![2, 1, 1]];
    test_good_requests = (0..num_requests)
        .map(|_| test_good_keys[rng.gen::<usize>() % test_good_keys.len()].clone())
        .collect::<Vec<Vec<u32>>>();
    page_requester.reset_request(test_good_requests);
    let result = load_pages_test(
        &engine,
        &test_pages,
        &mut page_controller,
        false,
        &page_requester,
        &mut trace_builder,
        &partial_pk,
    );

    assert_eq!(
        result,
        Err(VerificationError::NonZeroCumulativeSum),
        "Verification failed"
    );
}
