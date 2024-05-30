use crate::{page1_read::Page1ReadChip, page2_read::Page2ReadChip, page_node::PageNodeReadChip};

pub enum PageNode {
    PageTerminal(Page1ReadChip), // Assume partition is data | commitment | mult_t | mult_b | mult_p | mult
    PageBranch(Page2ReadChip), // Assume partition is data | commitment | mult_t | mult_b | mult_p | mult
    PageMixed(PageNodeReadChip), // Assume partition is data | commitment | mult_t | mult_b | mult_p | mult
}
