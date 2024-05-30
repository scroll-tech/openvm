use afs_stark_backend::rap::AnyRap;
use p3_uni_stark::StarkGenericConfig;

use crate::{page1_read::Page1ReadChip, page2_read::Page2ReadChip, page_node::PageNodeReadChip};

pub enum PageNode {
    PageTerminal(Page1ReadChip), // Assume partition is data | commitment | mult_t | mult_b | mult_p | mult
    PageBranch(Page2ReadChip), // Assume partition is data | commitment | mult_t | mult_b | mult_p | mult
    PageMixed(PageNodeReadChip), // Assume partition is data | commitment | mult_t | mult_b | mult_p | mult
}

// impl PageNode {
//     pub fn to_any_air<SC: StarkGenericConfig>(&self) -> &dyn AnyRap<SC>
//     where
//         <<<SC as StarkGenericConfig>::Pcs as p3_commit::pcs::Pcs<
//             <SC as StarkGenericConfig>::Challenge,
//             <SC as StarkGenericConfig>::Challenger,
//         >>::Domain as p3_commit::domain::PolynomialSpace>::Val: PrimeField64,
//     {
//         match self {
//             PageNode::PageTerminal(p) => p,
//             PageNode::PageBranch(p) => p,
//             PageNode::PageMixed(p) => p,
//         }
//     }
// }
