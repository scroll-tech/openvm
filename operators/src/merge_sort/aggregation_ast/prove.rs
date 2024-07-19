use std::{sync::Arc, time::Instant};

use afs_chips::{common::page::Page, page_btree::cmp};
use afs_stark_backend::{
    config::{Com, PcsProof, PcsProverData},
    keygen::types::MultiStarkPartialVerifyingKey,
    prover::{
        trace::{ProverTraceData, TraceCommitmentBuilder},
        types::Proof,
    },
    rap::AnyRap,
};
use afs_test_utils::engine::StarkEngine;
use itertools::Itertools;
use p3_field::{AbstractField, PrimeField, PrimeField64};
use p3_uni_stark::{Domain, StarkGenericConfig, Val};
use tracing::info_span;

use crate::merge_sort::{
    get_top_p::page_controller::GetTopProverData,
    split_remaining::page_controller::SplitRemainingProverData,
};

use super::{
    provider::{PageProvider, ProverTraceDataProvider},
    verify::{PageControllers, ProvingKeys},
    DataFrameRow,
};

pub struct VerifierInputs<SC: StarkGenericConfig> {
    pub proof: Proof<SC>,
    pub pis: Vec<Vec<Val<SC>>>,
}

impl<SC: StarkGenericConfig> VerifierInputs<SC> {
    pub fn verify(
        &self,
        engine: &dyn StarkEngine<SC>,
        airs: Vec<&dyn AnyRap<SC>>,
        vk: &MultiStarkPartialVerifyingKey<SC>,
    ) {
        let verifier = engine.verifier();
        let mut challenger = engine.new_challenger();
        let result = verifier.verify(&mut challenger, vk, airs, &self.proof, &self.pis);
        assert!(result.is_ok());
    }
}

pub enum InitRemainingGTP<SC: StarkGenericConfig> {
    Init(VerifierInputs<SC>),
    InitGTP {
        head: Arc<Self>,
        tail: VerifierInputs<SC>,
    },
}

pub struct FinishMerge<SC: StarkGenericConfig> {
    pub init_gtp: InitRemainingGTP<SC>,
    pub split: VerifierInputs<SC>,
}

pub struct MergeSortMain<SC: StarkGenericConfig, const COMMITMENT_LEN: usize> {
    pub merge_sort: MergeSort<SC>,
}

pub enum MergeSort<SC: StarkGenericConfig> {
    Merge {
        pre: Arc<PreMerge<SC>>,
        finish: FinishMerge<SC>,
    },
    Single(),
}

pub struct PreMerge<SC: StarkGenericConfig> {
    pub merge_sorts: Vec<Arc<MergeSort<SC>>>,
}

impl<SC: StarkGenericConfig, const COMMITMENT_LEN: usize> MergeSortMain<SC, COMMITMENT_LEN>
where
    Val<SC>: PrimeField + PrimeField64,
    Domain<SC>: Send + Sync,
    SC::Pcs: Sync,
    PcsProverData<SC>: Send + Sync,
    Com<SC>: Send + Sync,
    SC::Challenge: Send + Sync,
    PcsProof<SC>: Send + Sync,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    // if number of pages is 1, pad to 2
    // Essentially, we want to tell our page_controllers whether their input pages will be sorted to avoid redundant sorting
    pub fn generate_main<P: PageProvider + ProverTraceDataProvider<SC>>(
        provider: &mut P,
        engine: &impl StarkEngine<SC>,
        trace_builder: &mut TraceCommitmentBuilder<SC>,
        page_controllers: &mut PageControllers<SC, COMMITMENT_LEN>,
        pks: &ProvingKeys<SC>,
        table: Vec<Vec<u32>>,
    ) -> (Self, Vec<DataFrameRow<u32>>) {
        let idx_len = page_controllers.params.idx_len;
        let data_len = page_controllers.params.data_len;
        let height = page_controllers.params.page_height;
        let limb_bits = page_controllers.params.limb_bits;
        let mut new_table = table
            .iter()
            .map(|p| DataFrameRow {
                start: vec![0; idx_len],
                end: vec![(1 << limb_bits) - 1; idx_len],
                commitment: p.clone(),
            })
            .collect_vec();
        let blank_page = Page::blank(idx_len, data_len, height);
        let blank_pdata = Arc::new(trace_builder.committer.commit(vec![blank_page.gen_trace()]));
        let blank_commit = pdata_to_commit_u32(&blank_pdata);
        provider.add_page_with_commitment(&blank_commit, &blank_page);
        provider.add_pdata_with_commitment(&blank_commit, blank_pdata);
        if new_table.len() == 1 {
            new_table.push(DataFrameRow {
                start: vec![0; idx_len],
                end: vec![(1 << limb_bits) - 1; idx_len],
                commitment: blank_commit.clone(),
            });
        }
        let (merge_sort, indexed_table, _) = Self::generate_merge_sort(
            provider,
            engine,
            trace_builder,
            page_controllers,
            pks,
            &new_table,
            &blank_commit,
        );
        (MergeSortMain { merge_sort }, indexed_table)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn generate_merge_sort<P: PageProvider + ProverTraceDataProvider<SC>>(
        provider: &mut P,
        engine: &impl StarkEngine<SC>,
        trace_builder: &mut TraceCommitmentBuilder<SC>,
        page_controllers: &mut PageControllers<SC, COMMITMENT_LEN>,
        pks: &ProvingKeys<SC>,
        table_slice: &[DataFrameRow<u32>],
        blank_commit: &Vec<u32>,
    ) -> (MergeSort<SC>, Vec<DataFrameRow<u32>>, bool) {
        let k = page_controllers.params.k;
        let idx_len = page_controllers.params.idx_len;
        let limb_bits = page_controllers.params.limb_bits;
        let len = table_slice.len();
        if len == 1 || len == 0 {
            (MergeSort::Single(), table_slice.to_vec(), false)
        } else {
            let (pre_merge, mut buf) = Self::generate_pre_merge(
                provider,
                engine,
                trace_builder,
                page_controllers,
                pks,
                table_slice,
                blank_commit,
            );
            let cur_len = buf.len();
            for _ in cur_len..k {
                buf.push((
                    DataFrameRow {
                        start: vec![(1 << limb_bits) - 1; idx_len],
                        end: vec![(1 << limb_bits) - 1; idx_len],
                        commitment: blank_commit.clone(),
                    },
                    true,
                ));
            }
            let (finish_merge, indexed_table) = Self::generate_finish_merge(
                provider,
                engine,
                trace_builder,
                page_controllers,
                pks,
                &buf,
            );
            (
                MergeSort::Merge {
                    pre: Arc::new(pre_merge),
                    finish: finish_merge,
                },
                indexed_table,
                true,
            )
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn generate_pre_merge<P: PageProvider + ProverTraceDataProvider<SC>>(
        provider: &mut P,
        engine: &impl StarkEngine<SC>,
        trace_builder: &mut TraceCommitmentBuilder<SC>,
        page_controllers: &mut PageControllers<SC, COMMITMENT_LEN>,
        pks: &ProvingKeys<SC>,
        table_slice: &[DataFrameRow<u32>],
        blank_commit: &Vec<u32>,
    ) -> (PreMerge<SC>, Vec<(DataFrameRow<u32>, bool)>) {
        let k = page_controllers.params.k;
        let len = table_slice.len();
        let idx_len = page_controllers.params.idx_len;
        let parts = len.min(k);
        let mut merge_sorts = (0..parts)
            .map(|i| {
                Self::generate_merge_sort(
                    provider,
                    engine,
                    trace_builder,
                    page_controllers,
                    pks,
                    &table_slice[i * len / parts..(i + 1) * len / parts],
                    blank_commit,
                )
            })
            .collect_vec();
        let mut tables = vec![vec![]; k];
        for (i, (_, table, is_sorted)) in merge_sorts.iter_mut().enumerate() {
            tables[i].append(&mut table.iter().map(|r| (r.clone(), *is_sorted)).collect_vec());
        }
        tables.resize(
            k,
            vec![(
                DataFrameRow {
                    start: vec![0; idx_len],
                    end: vec![0; idx_len],
                    commitment: blank_commit.clone(),
                },
                true,
            )],
        );
        let buf = deterministic_table_sort_with_bool(tables);
        let merge_sorts = merge_sorts
            .into_iter()
            .map(|(m, _, _)| Arc::new(m))
            .collect_vec();
        (PreMerge { merge_sorts }, buf)
    }

    pub fn generate_finish_merge<P: PageProvider + ProverTraceDataProvider<SC>>(
        provider: &mut P,
        engine: &impl StarkEngine<SC>,
        trace_builder: &mut TraceCommitmentBuilder<SC>,
        page_controllers: &mut PageControllers<SC, COMMITMENT_LEN>,
        pks: &ProvingKeys<SC>,
        buf: &[(DataFrameRow<u32>, bool)],
    ) -> (FinishMerge<SC>, Vec<DataFrameRow<u32>>) {
        let k = page_controllers.params.k;
        let m = buf.len() - k;
        let (init_gtp, remaining_commitment, mut indexed_table) = Self::generate_init_remaining(
            provider,
            engine,
            trace_builder,
            page_controllers,
            pks,
            buf,
            m,
        );
        let split_remaining = Instant::now();
        let page_gen = Instant::now();
        let input_page = provider
            .load_page_by_commitment(&buf[buf.len() - 1].0.commitment)
            .unwrap();
        let init_remaining = provider
            .load_page_by_commitment(&remaining_commitment)
            .unwrap();
        let input_pdata = provider
            .load_pdata_by_commitment(&buf[buf.len() - 1].0.commitment)
            .unwrap();
        let init_remaining_pdata = provider
            .load_pdata_by_commitment(&remaining_commitment)
            .unwrap();
        provider.remove_page_by_commitment(&buf[buf.len() - 1].0.commitment);
        provider.remove_page_by_commitment(&remaining_commitment);
        provider.remove_pdata_by_commitment(&buf[buf.len() - 1].0.commitment);
        provider.remove_pdata_by_commitment(&remaining_commitment);
        let output_pages = page_controllers.split_remaining.generate_output_pages(
            &init_remaining,
            &input_page,
            buf[buf.len() - 1].1,
        );
        let duration = page_gen.elapsed();
        println!("Split Remaining Page gen took {:?}", duration);
        let pdata = SplitRemainingProverData {
            remaining_pdata: Some(init_remaining_pdata),
            input_page_pdata: Some(input_pdata),
            output_page_pdata: vec![None; k],
        };
        let page_load = Instant::now();
        let mut pdata = page_controllers.split_remaining.load_pages(
            &init_remaining,
            &input_page,
            &output_pages,
            pdata,
            &mut trace_builder.committer,
        );
        let duration = page_load.elapsed();
        println!("Split Remaining Page load took {:?}", duration);
        let output_commits = pdata
            .output_page_pdata
            .iter()
            .map(|pdata| pdata_to_commit_u32(pdata.as_ref().unwrap()))
            .collect_vec();
        for ((c, p), pdata) in output_commits
            .iter()
            .zip(output_pages.iter())
            .zip(pdata.output_page_pdata.iter_mut())
        {
            provider.add_page_with_commitment(c, p);
            *pdata = Some(provider.add_pdata_with_commitment(c, pdata.take().unwrap()));
        }
        let prove_time = Instant::now();
        let (proof, pis) = page_controllers.split_remaining.prove(
            engine,
            &pks.split_remaining,
            trace_builder,
            pdata,
        );
        let duration = prove_time.elapsed();
        println!("Split Remaining proving took {:?}", duration);
        let mut kept = k;
        while kept >= 2 && pis[kept][0] == Val::<SC>::zero() {
            // if pis[1 + kept][0] == Val::<SC>::zero() {
            //     kept -= 1;
            // } else {
            //     break;
            // }
            kept -= 1;
        }
        // remove the commitments we don't actually care about
        for c in output_commits.iter().skip(kept) {
            provider.remove_page_by_commitment(c);
            provider.remove_pdata_by_commitment(c);
        }
        // push rest of the pages onto indexed_table
        let idx_len = page_controllers.get_top_p.idx_len();
        for j in 0..kept {
            let start = pis[2 + j][1..1 + idx_len]
                .iter()
                .map(|f| f.as_canonical_u64() as u32)
                .collect_vec();
            let end = pis[2 + j][1 + idx_len..1 + 2 * idx_len]
                .iter()
                .map(|f| f.as_canonical_u64() as u32)
                .collect_vec();
            indexed_table.push(DataFrameRow {
                start,
                end,
                commitment: output_commits[j].clone(),
            });
        }
        let split = VerifierInputs { proof, pis };
        let duration = split_remaining.elapsed();
        println!("Split Remaining took {:?}", duration);
        (FinishMerge { init_gtp, split }, indexed_table)
    }

    // assume buf.len() >= k
    pub fn generate_init_remaining<P: PageProvider + ProverTraceDataProvider<SC>>(
        provider: &mut P,
        engine: &impl StarkEngine<SC>,
        trace_builder: &mut TraceCommitmentBuilder<SC>,
        page_controllers: &mut PageControllers<SC, COMMITMENT_LEN>,
        pks: &ProvingKeys<SC>,
        buf: &[(DataFrameRow<u32>, bool)],
        m: usize,
    ) -> (InitRemainingGTP<SC>, Vec<u32>, Vec<DataFrameRow<u32>>) {
        let k = page_controllers.params.k;
        if m == 0 {
            let init_remaining_gtp = Instant::now();
            let init_pages = buf
                .iter()
                .take(k - 1)
                .map(|c| {
                    let page = provider.load_page_by_commitment(&c.0.commitment).unwrap();
                    provider.remove_page_by_commitment(&c.0.commitment);
                    page
                })
                .collect_vec();
            let init_page_pdata = buf
                .iter()
                .take(k - 1)
                .map(|c| {
                    let pdata = provider.load_pdata_by_commitment(&c.0.commitment).unwrap();
                    provider.remove_page_by_commitment(&c.0.commitment);
                    Some(pdata)
                })
                .collect_vec();
            let inputs_are_sorted = buf[0..k - 1].iter().map(|r| r.1).collect_vec();
            let remaining_page = page_controllers
                .init_remaining
                .gen_remaining(&init_pages, &inputs_are_sorted);
            let (mut init_page_pdata, mut remaining_pdata) =
                page_controllers.init_remaining.load_pages(
                    &init_pages,
                    remaining_page.clone(),
                    init_page_pdata,
                    vec![None],
                    &mut trace_builder.committer,
                );
            let init_page_pdata = init_page_pdata
                .iter_mut()
                .map(|p| p.take().unwrap())
                .collect_vec();
            let remaining_commit = pdata_to_commit_u32(remaining_pdata[0].as_ref().unwrap());
            provider.add_page_with_commitment(&remaining_commit, &remaining_page);
            let remaining_pdata = remaining_pdata[0].take().unwrap();
            let remaining_pdata =
                provider.add_pdata_with_commitment(&remaining_commit, remaining_pdata);
            let (proof, pis) = page_controllers.init_remaining.prove(
                engine,
                &pks.init_remaining,
                trace_builder,
                init_page_pdata,
                remaining_pdata,
            );
            let init = VerifierInputs { proof, pis };
            let duration = init_remaining_gtp.elapsed();
            println!("Init Remaining took {:?}", duration);
            (InitRemainingGTP::Init(init), remaining_commit, vec![])
        } else {
            let (prev, init_remaining_commit, mut indexed_table) = Self::generate_init_remaining(
                provider,
                engine,
                trace_builder,
                page_controllers,
                pks,
                buf,
                m - 1,
            );
            let gtp = Instant::now();
            let tail_idx = k - 2 + m;
            let input_page = provider
                .load_page_by_commitment(&buf[tail_idx].0.commitment)
                .unwrap();
            let input_page_is_sorted = buf[tail_idx].1;
            let init_remaining = provider
                .load_page_by_commitment(&init_remaining_commit)
                .unwrap();
            let (final_remaining_page, top_p_page) = page_controllers
                .get_top_p
                .generate_output_pages(&init_remaining, &input_page, input_page_is_sorted);
            let input_page_pdata = provider.load_pdata_by_commitment(&buf[tail_idx].0.commitment);
            let init_remaining_pdata = provider.load_pdata_by_commitment(&init_remaining_commit);
            provider.remove_page_by_commitment(&init_remaining_commit);
            provider.remove_page_by_commitment(&buf[tail_idx].0.commitment);
            provider.remove_pdata_by_commitment(&buf[tail_idx].0.commitment);
            provider.remove_pdata_by_commitment(&init_remaining_commit);
            let pdata = GetTopProverData {
                init_remaining_pdata,
                input_page_pdata,
                final_remaining_pdata: None,
                top_p_pdata: None,
            };
            let mut pdata = page_controllers.get_top_p.load_pages(
                &init_remaining,
                &input_page,
                &final_remaining_page,
                &top_p_page,
                pdata,
                &mut trace_builder.committer,
            );
            let top_p_commit = pdata_to_commit_u32(pdata.top_p_pdata.as_ref().unwrap());
            let remaining = pdata_to_commit_u32(pdata.final_remaining_pdata.as_ref().unwrap());

            pdata.final_remaining_pdata = Some(provider.add_pdata_with_commitment(
                &remaining,
                pdata.final_remaining_pdata.take().unwrap(),
            ));
            provider.add_page_with_commitment(&remaining, &final_remaining_page);
            pdata.top_p_pdata = Some(
                provider
                    .add_pdata_with_commitment(&top_p_commit, pdata.top_p_pdata.take().unwrap()),
            );
            provider.add_page_with_commitment(&top_p_commit, &top_p_page);

            let (proof, pis) =
                page_controllers
                    .get_top_p
                    .prove(engine, &pks.get_top_p, trace_builder, pdata);
            let idx_len = page_controllers.get_top_p.idx_len();
            let start = pis[3][0..idx_len]
                .iter()
                .map(|f| f.as_canonical_u64() as u32)
                .collect_vec();
            let end = pis[3][idx_len..2 * idx_len]
                .iter()
                .map(|f| f.as_canonical_u64() as u32)
                .collect_vec();
            indexed_table.push(DataFrameRow {
                start,
                end,
                commitment: top_p_commit,
            });
            let tail = VerifierInputs { proof, pis };
            let duration = gtp.elapsed();
            println!("Get Top P took {:?}", duration);
            (
                InitRemainingGTP::InitGTP {
                    head: Arc::new(prev),
                    tail,
                },
                remaining,
                indexed_table,
            )
        }
    }
}

pub fn pdata_to_commit_u32<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>(
    pdata: &Arc<ProverTraceData<SC>>,
) -> Vec<u32>
where
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
    Val<SC>: PrimeField + PrimeField64,
{
    let commit: [Val<SC>; COMMITMENT_LEN] = pdata.commit.clone().into();
    commit
        .into_iter()
        .map(|f| f.as_canonical_u64() as u32)
        .collect_vec()
}

// not going to use a heap because k usually is equal to 2
pub fn deterministic_table_sort(tables: Vec<Vec<DataFrameRow<u32>>>) -> Vec<DataFrameRow<u32>> {
    let k = tables.len();
    let mut buf = vec![];
    let mut cur_idx = vec![0; k];
    let sort_span = info_span!("Sort k indexed tables").entered();
    loop {
        let mut best_idx = -1;
        let mut smallest_start = &vec![];
        for (i, (table, idx)) in tables.iter().zip(cur_idx.iter()).enumerate() {
            if cur_idx[i] < table.len()
                && (best_idx == -1 || cmp(&table[*idx].start, smallest_start) < 0)
            {
                best_idx = i as i32;
                smallest_start = &table[*idx].start;
            }
        }
        if best_idx == -1 {
            break;
        } else {
            buf.push(tables[best_idx as usize][cur_idx[best_idx as usize]].clone());
            cur_idx[best_idx as usize] += 1;
        }
    }
    sort_span.exit();
    buf
}

// not going to use a heap because k usually is equal to 2
pub fn deterministic_table_sort_with_bool(
    tables: Vec<Vec<(DataFrameRow<u32>, bool)>>,
) -> Vec<(DataFrameRow<u32>, bool)> {
    let k = tables.len();
    let mut buf = vec![];
    let mut cur_idx = vec![0; k];
    let sort_span = info_span!("Sort k indexed tables").entered();
    loop {
        let mut best_idx = -1;
        let mut smallest_start = &vec![];
        for (i, (table, idx)) in tables.iter().zip(cur_idx.iter()).enumerate() {
            if cur_idx[i] < table.len()
                && (best_idx == -1 || cmp(&table[*idx].0.start, smallest_start) < 0)
            {
                best_idx = i as i32;
                smallest_start = &table[*idx].0.start;
            }
        }
        if best_idx == -1 {
            break;
        } else {
            buf.push(tables[best_idx as usize][cur_idx[best_idx as usize]].clone());
            cur_idx[best_idx as usize] += 1;
        }
    }
    sort_span.exit();
    buf
}
