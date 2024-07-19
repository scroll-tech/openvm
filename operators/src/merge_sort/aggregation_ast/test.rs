use std::{
    fs::{create_dir_all, remove_dir_all},
    sync::Arc,
    time::Instant,
};

use afs_chips::{
    common::page::{merge_pages, Page},
    multitier_page_rw_checker::page_controller::MyLessThanTupleParams,
};
use afs_stark_backend::prover::{trace::TraceCommitmentBuilder, MultiTraceStarkProver};
use afs_test_utils::{
    config::{self, baby_bear_poseidon2::BabyBearPoseidon2Config},
    utils::create_seeded_rng,
};
use itertools::Itertools;
use p3_baby_bear::BabyBear;
use p3_field::PrimeField32;
use p3_util::log2_strict_usize;

use super::{
    prove::MergeSortMain,
    provider::{DiskPageLoader, PageProvider, ProverTraceDataProvider},
    verify::{
        verify_merge_sort_main, PageControllerParams, PageControllers, ProvingKeys, VerifyingKeys,
    },
    DataFrameRow,
};

const BABYBEAR_COMMITMENT_LEN: usize = 8;
const LIMB_BITS: usize = 20;

pub fn test<P: PageProvider + ProverTraceDataProvider<BabyBearPoseidon2Config>>(
    provider: &mut P,
    params: PageControllerParams,
    num_pages: usize,
    fullness_fraction_denom: usize,
) {
    let blank_commit = params.blank_commit.clone();
    let mut rng = create_seeded_rng();
    let page_height = params.page_height;
    let idx_len = params.idx_len;
    let data_len = params.data_len;
    let limb_bits = params.limb_bits;
    let max_idx = (1 << limb_bits) - 1;
    let k = params.k;
    let pages = (0..num_pages)
        .map(|_| {
            Page::random(
                &mut rng,
                idx_len,
                data_len,
                max_idx,
                max_idx,
                page_height,
                page_height / fullness_fraction_denom,
            )
        })
        .collect_vec();
    let sort_time = Instant::now();
    let mut merge_page = merge_pages(&pages);
    merge_page.rows.sort();
    let duration = sort_time.elapsed();
    println!("Normal Sorting took {:?}", duration);
    let deg = log2_strict_usize((k - 1).next_power_of_two() * page_height).max(8);
    let engine = config::baby_bear_poseidon2::default_engine(deg);

    let prover = MultiTraceStarkProver::new(&engine.config);
    let mut trace_builder = TraceCommitmentBuilder::<BabyBearPoseidon2Config>::new(prover.pcs());
    let end_commits = (0..num_pages)
        .map(|i| {
            let page = Page {
                rows: merge_page.rows[i * page_height..(i + 1) * page_height].to_vec(),
            };
            let commit: [BabyBear; BABYBEAR_COMMITMENT_LEN] = trace_builder
                .committer
                .commit(vec![page.gen_trace()])
                .commit
                .into();
            commit.iter().map(|f| f.as_canonical_u32()).collect_vec()
        })
        .collect_vec();
    let init_table = pages
        .iter()
        .map(|p| {
            let pdata = trace_builder.committer.commit(vec![p.gen_trace()]);
            let commit: [BabyBear; BABYBEAR_COMMITMENT_LEN] = pdata.commit.into();
            let commit = commit.iter().map(|f| f.as_canonical_u32()).collect_vec();
            provider.add_page_with_commitment(&commit, p);
            provider.add_pdata_with_commitment(&commit, Arc::new(pdata));
            commit
        })
        .collect_vec();
    let mut page_controllers = PageControllers::new(
        k,
        page_height,
        idx_len,
        data_len,
        MyLessThanTupleParams {
            limb_bits,
            decomp: 8,
        },
        &trace_builder.committer,
    );
    let pks = ProvingKeys::new(&engine, &page_controllers);
    let vks = VerifyingKeys::new(&engine, &page_controllers);
    let prove_time = Instant::now();
    let (main, mut indexed_table) = MergeSortMain::generate_main(
        provider,
        &engine,
        &mut trace_builder,
        &mut page_controllers,
        &pks,
        init_table.clone(),
    );
    let duration = prove_time.elapsed();
    println!("Proving took {:?}", duration);
    let verify_time = Instant::now();
    verify_merge_sort_main(&engine, &page_controllers, &vks, init_table, main);
    let duration = verify_time.elapsed();
    println!("Verifying took {:?}", duration);
    indexed_table.resize(
        num_pages,
        DataFrameRow {
            start: vec![0; idx_len],
            end: vec![0; idx_len],
            commitment: blank_commit.clone(),
        },
    );
    for (com, row) in end_commits.iter().zip(indexed_table.iter()) {
        for (c, r) in com.iter().zip(row.commitment.iter()) {
            assert_eq!(*c, *r);
        }
    }
}

pub fn setup_test(
    page_height: usize,
    idx_len: usize,
    data_len: usize,
    limb_bits: usize,
    k: usize,
) -> PageControllerParams {
    let deg = log2_strict_usize((k - 1).next_power_of_two() * page_height).max(8);

    let engine = config::baby_bear_poseidon2::default_engine(deg);

    let prover = MultiTraceStarkProver::new(&engine.config);
    let trace_builder = TraceCommitmentBuilder::<BabyBearPoseidon2Config>::new(prover.pcs());
    let blank_page = Page::blank(idx_len, data_len, page_height);
    let blank_commit: [BabyBear; BABYBEAR_COMMITMENT_LEN] = trace_builder
        .committer
        .commit(vec![blank_page.gen_trace()])
        .commit
        .into();
    let blank_commit = blank_commit
        .iter()
        .map(|b| b.as_canonical_u32())
        .collect_vec();
    PageControllerParams {
        page_height,
        idx_len,
        data_len,
        limb_bits,
        k,
        blank_commit,
    }
}

#[test]
pub fn small_test() {
    create_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
    let params = setup_test(32, 2, 2, LIMB_BITS, 2);
    // let mut provider = BTreeMapPageLoader::new(params.blank_commit.clone());
    let mut provider = DiskPageLoader::new(
        "src/merge_sort/aggregation_ast/data".to_string(),
        2,
        2,
        params.blank_commit.clone(),
    );
    test(&mut provider, params, 10, 2);
}

#[test]
pub fn k_is_3_test() {
    create_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
    let params = setup_test(32, 2, 2, LIMB_BITS, 3);
    // let mut provider = BTreeMapPageLoader::new(params.blank_commit.clone());
    let mut provider = DiskPageLoader::new(
        "src/merge_sort/aggregation_ast/data".to_string(),
        2,
        2,
        params.blank_commit.clone(),
    );
    test(&mut provider, params, 10, 2);
    remove_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
}

#[test]
pub fn quite_empty_test() {
    create_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
    let params = setup_test(32, 2, 2, LIMB_BITS, 2);
    // let mut provider = BTreeMapPageLoader::new(params.blank_commit.clone());
    let mut provider = DiskPageLoader::new(
        "src/merge_sort/aggregation_ast/data".to_string(),
        2,
        2,
        params.blank_commit.clone(),
    );
    test(&mut provider, params, 10, 5);
    remove_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
}

#[test]
pub fn actually_empty_test() {
    create_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
    let params = setup_test(32, 2, 2, LIMB_BITS, 2);
    // let mut provider = BTreeMapPageLoader::new(params.blank_commit.clone());
    let mut provider = DiskPageLoader::new(
        "src/merge_sort/aggregation_ast/data".to_string(),
        2,
        2,
        params.blank_commit.clone(),
    );
    test(&mut provider, params, 10, 33);
    remove_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
}

#[test]
pub fn quite_full_test() {
    create_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
    let params = setup_test(32, 2, 2, LIMB_BITS, 2);
    // let mut provider = BTreeMapPageLoader::new(params.blank_commit.clone());
    let mut provider = DiskPageLoader::new(
        "src/merge_sort/aggregation_ast/data".to_string(),
        2,
        2,
        params.blank_commit.clone(),
    );
    test(&mut provider, params, 10, 1);
    remove_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
}

#[test]
pub fn large_page_test() {
    create_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
    // let params = setup_test(1048576, 16, 32, LIMB_BITS, 2);
    let params = setup_test(1_048_576, 16, 32, LIMB_BITS, 2);

    // let mut provider = BTreeMapPageLoader::new(params.blank_commit.clone());
    let mut provider = DiskPageLoader::new(
        "src/merge_sort/aggregation_ast/data".to_string(),
        16,
        32,
        params.blank_commit.clone(),
    );
    test(&mut provider, params, 10, 2);
    remove_dir_all("src/merge_sort/aggregation_ast/data").unwrap();
}
