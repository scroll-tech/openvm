use std::panic;

use afs_chips::common::page::Page;
use afs_stark_backend::{
    keygen::{types::MultiStarkPartialProvingKey, MultiStarkKeygenBuilder},
    prover::{trace::TraceCommitmentBuilder, MultiTraceStarkProver},
    verifier::VerificationError,
};
use afs_test_utils::config;
use afs_test_utils::config::baby_bear_poseidon2::{
    BabyBearPoseidon2Config, BabyBearPoseidon2Engine,
};
use afs_test_utils::utils::create_seeded_rng;
use itertools::Itertools;
use rand::Rng;

use super::page_controller::PageController;

#[allow(clippy::too_many_arguments)]
fn load_page_test(
    engine: &BabyBearPoseidon2Engine,
    init_pages: &[Page],
    final_pages: &[Page],
    page_controller: &mut PageController<BabyBearPoseidon2Config>,
    trace_builder: &mut TraceCommitmentBuilder<BabyBearPoseidon2Config>,
    partial_pk: &MultiStarkPartialProvingKey<BabyBearPoseidon2Config>,
) -> Result<(), VerificationError> {
    let (init_page_pdata, final_page_pdata) = page_controller.load_pages(
        init_pages,
        final_pages,
        None,
        None,
        &mut trace_builder.committer,
    );

    let (proof, _) = page_controller.prove(
        engine,
        partial_pk,
        trace_builder,
        init_page_pdata,
        final_page_pdata,
    );

    page_controller.verify(engine, partial_pk.partial_vk(), proof)
}

#[test]
fn equal_page_content_test() {
    let mut rng = create_seeded_rng();

    let page_bus_index = 0;

    const MAX_VAL: u32 = 0x78000001 / 2; // The prime used by BabyBear / 2

    let log_page_height = 4;
    let log_num_ops = 3;

    let page_width = 6;

    let idx_len = rng.gen::<usize>() % ((page_width - 1) - 1) + 1;
    let data_len = (page_width - 1) - idx_len;
    let idx_limb_bits = 10;
    let idx_decomp = 4;
    let max_idx = 1 << idx_limb_bits;

    // Generating a random page with distinct indices
    let initial_page = Page::random(&mut rng, idx_len, data_len, max_idx, MAX_VAL, 20, 10);

    let mut final_page = initial_page.clone();
    final_page.rows.reverse();

    let init_page_heights = vec![8, 4, 8];
    let final_page_heights = vec![16, 4];
    let mut start = 0;
    let init_pages = init_page_heights
        .clone()
        .into_iter()
        .map(|h| {
            let p = Page {
                rows: initial_page.rows[start..start + h].to_vec(),
            };
            start += h;
            p
        })
        .collect_vec();
    start = 0;
    let final_pages = final_page_heights
        .clone()
        .into_iter()
        .map(|h| {
            let p = Page {
                rows: final_page.rows[start..start + h].to_vec(),
            };
            start += h;
            p
        })
        .collect_vec();
    let mut page_controller: PageController<BabyBearPoseidon2Config> = PageController::new(
        page_bus_index,
        idx_len,
        data_len,
        init_page_heights.clone(),
        final_page_heights.clone(),
    );
    let engine = config::baby_bear_poseidon2::default_engine(
        idx_decomp.max(log_page_height.max(3 + log_num_ops)),
    );
    let mut keygen_builder = MultiStarkKeygenBuilder::new(&engine.config);

    page_controller.set_up_keygen_builder(&mut keygen_builder);

    let partial_pk = keygen_builder.generate_partial_pk();

    let prover = MultiTraceStarkProver::new(&engine.config);
    let mut trace_builder = TraceCommitmentBuilder::new(prover.pcs());

    load_page_test(
        &engine,
        &init_pages,
        &final_pages,
        &mut page_controller,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");

    let mut start = 0;
    let init_pages = init_page_heights
        .clone()
        .into_iter()
        .map(|h| {
            let p = Page {
                rows: initial_page.rows[start..start + h].to_vec(),
            };
            start += h;
            p
        })
        .collect_vec();
    start = 0;
    let final_pages = final_page_heights
        .clone()
        .into_iter()
        .map(|h| {
            let p = Page {
                rows: final_page.rows[start..start + h].to_vec(),
            };
            start += h;
            p
        })
        .collect_vec();

    load_page_test(
        &engine,
        &init_pages,
        &final_pages,
        &mut page_controller,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");

    let initial_page = Page::random(&mut rng, idx_len, data_len, max_idx, MAX_VAL, 20, 10);

    let mut rows = initial_page.rows.clone();
    for row in &mut rows {
        if row.is_alloc == 1 {
            row.idx = vec![0; idx_len];
            break;
        }
    }
    let mut final_page = Page { rows };
    final_page.rows.reverse();

    let mut start = 0;
    let init_pages = init_page_heights
        .into_iter()
        .map(|h| {
            let p = Page {
                rows: initial_page.rows[start..start + h].to_vec(),
            };
            start += h;
            p
        })
        .collect_vec();
    start = 0;
    let final_pages = final_page_heights
        .into_iter()
        .map(|h| {
            let p = Page {
                rows: final_page.rows[start..start + h].to_vec(),
            };
            start += h;
            p
        })
        .collect_vec();

    let engine_ref = &engine;
    let result = panic::catch_unwind(move || {
        let _ = load_page_test(
            engine_ref,
            &init_pages,
            &final_pages,
            &mut page_controller,
            &mut trace_builder,
            &partial_pk,
        );
    });

    assert!(
        result.is_err(),
        "Expected to fail when contents are not equal"
    );
}
