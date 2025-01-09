use alloc::{collections::BTreeMap, format};
use core::{fmt, fmt::Display};

use openvm_circuit::arch::instructions::instruction::DebugInfo;
use openvm_stark_backend::p3_field::{ExtensionField, PrimeField32};

use super::AsmInstruction;

/// A basic block of assembly instructions.
#[derive(Debug, Clone, Default)]
pub struct BasicBlock<F, EF>(
    pub(crate) Vec<AsmInstruction<F, EF>>,
    pub(crate) Vec<Option<DebugInfo>>,
);

impl<F: PrimeField32, EF: ExtensionField<F>> BasicBlock<F, EF> {
    /// Creates a new basic block.
    pub fn new() -> Self {
        Self(Vec::new(), Vec::new())
    }

    /// Pushes an instruction to a basic block.
    pub(crate) fn push(
        &mut self,
        instruction: AsmInstruction<F, EF>,
        debug_info: Option<DebugInfo>,
    ) {
        self.0.push(instruction);
        self.1.push(debug_info);
    }
}

/// Assembly code for a program.
pub struct AssemblyCode<F, EF> {
    pub blocks: Vec<BasicBlock<F, EF>>,
    pub labels: BTreeMap<F, String>,
}

impl<F: PrimeField32, EF: ExtensionField<F>> AssemblyCode<F, EF> {
    /// Creates a new assembly code.
    pub fn new(blocks: Vec<BasicBlock<F, EF>>, labels: BTreeMap<F, String>) -> Self {
        Self { blocks, labels }
    }

    pub fn size(&self) -> usize {
        self.blocks.iter().map(|block| block.0.len()).sum()
    }
}

impl<F: PrimeField32, EF: ExtensionField<F>> Display for AssemblyCode<F, EF> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, block) in self.blocks.iter().enumerate() {
            // TODO: should we skip empty blocks?
            writeln!(
                f,
                "{}:",
                self.labels
                    .get(&F::from_canonical_u32(i as u32))
                    .unwrap_or(&format!(".L{}", i))
            )?;
            for instruction in &block.0 {
                write!(f, "        ")?;
                instruction.fmt(&self.labels, f)?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<F: PrimeField32, EF: ExtensionField<F>> AssemblyCode<F, EF> {
    pub fn parse_asm(s: &str) -> Result<Self, String> {
        let mut blocks = Vec::new();
        let mut labels = BTreeMap::new();
        let mut current_block = BasicBlock::new();
        let mut current_label = None;

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.ends_with(':') {
                if let Some(label) = current_label.take() {
                    labels.insert(F::from_canonical_u32(blocks.len() as u32), label);
                    blocks.push(current_block);
                    current_block = BasicBlock::new();
                }
                current_label = Some(line.trim_end_matches(':').to_string());
            } else if let Some(instruction) = AsmInstruction::parse_instruction(line, &labels) {
                current_block.push(instruction, None);
            } else {
                return Err(format!("Failed to parse instruction: {}", line));
            }
        }

        if let Some(label) = current_label {
            labels.insert(F::from_canonical_u32(blocks.len() as u32), label);
        }
        if !current_block.0.is_empty() {
            blocks.push(current_block);
        }

        let code = AssemblyCode { blocks, labels };
        //debug_assert_eq!(s, code.to_string());
        Ok(code)
    }
}
