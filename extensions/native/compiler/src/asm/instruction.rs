use alloc::{collections::BTreeMap, format};
use core::fmt;

use openvm_stark_backend::p3_field::{ExtensionField, PrimeField32};

use super::A0;

#[derive(Debug, Clone)]
pub enum AsmInstruction<F, EF> {
    /// Load word (dst, src, var_index, size, offset).
    ///
    /// Load a value from the address stored at src(fp) into dst(fp) with given index and offset.
    LoadF(i32, i32, i32, F, F),
    LoadFI(i32, i32, F, F, F),

    /// Store word (val, addr, var_index, size, offset)
    ///
    /// Store a value from val(fp) into the address stored at addr(fp) with given index and offset.
    StoreF(i32, i32, i32, F, F),
    StoreFI(i32, i32, F, F, F),

    /// Set dst = imm.
    ImmF(i32, F),

    /// Copy, dst = src.
    CopyF(i32, i32),

    /// Add, dst = lhs + rhs.
    AddF(i32, i32, i32),

    /// Add immediate, dst = lhs + rhs.
    AddFI(i32, i32, F),

    /// Subtract, dst = lhs - rhs.
    SubF(i32, i32, i32),

    /// Subtract immediate, dst = lhs - rhs.
    SubFI(i32, i32, F),

    /// Subtract value from immediate, dst = lhs - rhs.
    SubFIN(i32, F, i32),

    /// Multiply, dst = lhs * rhs.
    MulF(i32, i32, i32),

    /// Multiply immediate.
    MulFI(i32, i32, F),

    /// Divide, dst = lhs / rhs.
    DivF(i32, i32, i32),

    /// Divide immediate, dst = lhs / rhs.
    DivFI(i32, i32, F),

    /// Divide value from immediate, dst = lhs / rhs.
    DivFIN(i32, F, i32),

    /// Add extension, dst = lhs + rhs.
    AddE(i32, i32, i32),

    /// Subtract extension, dst = lhs - rhs.
    SubE(i32, i32, i32),

    /// Multiply extension, dst = lhs * rhs.
    MulE(i32, i32, i32),

    /// Divide extension, dst = lhs / rhs.
    DivE(i32, i32, i32),

    /// Jump.
    Jump(i32, F),

    /// Branch not equal.
    Bne(F, i32, i32),

    /// Branch not equal immediate.
    BneI(F, i32, F),

    /// Branch equal.
    Beq(F, i32, i32),

    /// Branch equal immediate.
    BeqI(F, i32, F),

    /// Branch not equal extension.
    BneE(F, i32, i32),

    /// Branch not equal immediate extension.
    BneEI(F, i32, EF),

    /// Branch equal extension.
    BeqE(F, i32, i32),

    /// Branch equal immediate extension.
    BeqEI(F, i32, EF),

    /// Trap.
    Trap,

    /// Halt.
    Halt,

    /// Break(label)
    Break(F),

    /// Perform a Poseidon2 permutation on state starting at address `lhs`
    /// and store new state at `rhs`.
    /// (a, b) are pointers to (lhs, rhs).
    Poseidon2Permute(i32, i32),
    /// Perform 2-to-1 cryptographic compression using Poseidon2.
    /// (a, b, c) are memory pointers to (dst, lhs, rhs)
    Poseidon2Compress(i32, i32, i32),

    /// (a, b, res, len, alpha, alpha_pow)
    FriReducedOpening(i32, i32, i32, i32, i32, i32),

    /// Print a variable.
    PrintV(i32),

    /// Print a felt.
    PrintF(i32),

    /// Print an extension element.
    PrintE(i32),

    /// Add next input vector to hint stream.
    HintInputVec(),

    /// HintBits(src, len).
    ///
    /// Bit decompose the field element at pointer `src` to the first `len` little endian bits and add to hint stream.
    HintBits(i32, u32),

    /// Stores the next hint stream word into value stored at addr + value.
    StoreHintWordI(i32, F),

    /// Publish(val, index).
    Publish(i32, i32),

    CycleTrackerStart(),
    CycleTrackerEnd(),
}

impl<F: PrimeField32, EF: ExtensionField<F>> AsmInstruction<F, EF> {
    pub fn j(label: F) -> Self {
        AsmInstruction::Jump(A0, label)
    }

    pub fn fmt(&self, labels: &BTreeMap<F, String>, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AsmInstruction::Break(_) => panic!("Unresolved break instruction"),
            AsmInstruction::LoadF(dst, src, var_index, size, offset) => {
                write!(
                    f,
                    "lw    ({})fp, ({})fp, ({})fp, {}, {}",
                    dst, src, var_index, size, offset
                )
            }
            AsmInstruction::LoadFI(dst, src, var_index, size, offset) => {
                write!(
                    f,
                    "lwi   ({})fp, ({})fp, {}, {}, {}",
                    dst, src, var_index, size, offset
                )
            }
            AsmInstruction::StoreF(dst, src, var_index, size, offset) => {
                write!(
                    f,
                    "sw    ({})fp, ({})fp, ({})fp, {}, {}",
                    dst, src, var_index, size, offset
                )
            }
            AsmInstruction::StoreFI(dst, src, var_index, size, offset) => {
                write!(
                    f,
                    "swi   ({})fp, ({})fp, {}, {}, {}",
                    dst, src, var_index, size, offset
                )
            }
            AsmInstruction::ImmF(dst, src) => {
                // TODO: why "()" here?
                write!(f, "imm   ({})fp, ({})", dst, src)
            }
            AsmInstruction::CopyF(dst, src) => {
                write!(f, "copy  ({})fp, ({})", dst, src)
            }
            AsmInstruction::AddF(dst, lhs, rhs) => {
                write!(f, "add   ({})fp, ({})fp, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::AddFI(dst, lhs, rhs) => {
                write!(f, "addi  ({})fp, ({})fp, {}", dst, lhs, rhs)
            }
            AsmInstruction::SubF(dst, lhs, rhs) => {
                write!(f, "sub   ({})fp, ({})fp, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::SubFI(dst, lhs, rhs) => {
                write!(f, "subi  ({})fp, ({})fp, {}", dst, lhs, rhs)
            }
            AsmInstruction::SubFIN(dst, lhs, rhs) => {
                write!(f, "subin ({})fp, {}, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::MulF(dst, lhs, rhs) => {
                write!(f, "mul   ({})fp, ({})fp, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::MulFI(dst, lhs, rhs) => {
                write!(f, "muli  ({})fp, ({})fp, {}", dst, lhs, rhs)
            }
            AsmInstruction::DivF(dst, lhs, rhs) => {
                write!(f, "div   ({})fp, ({})fp, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::DivFI(dst, lhs, rhs) => {
                write!(f, "divi  ({})fp, ({})fp, {}", dst, lhs, rhs)
            }
            AsmInstruction::DivFIN(dst, lhs, rhs) => {
                write!(f, "divin  ({})fp, {}, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::AddE(dst, lhs, rhs) => {
                write!(f, "eadd ({})fp, ({})fp, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::SubE(dst, lhs, rhs) => {
                write!(f, "esub  ({})fp, ({})fp, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::MulE(dst, lhs, rhs) => {
                write!(f, "emul  ({})fp, ({})fp, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::DivE(dst, lhs, rhs) => {
                write!(f, "ediv  ({})fp, ({})fp, ({})fp", dst, lhs, rhs)
            }
            AsmInstruction::Jump(dst, label) => {
                write!(
                    f,
                    "j     ({})fp, {}",
                    dst,
                    labels.get(label).unwrap_or(&format!(".L{}", label))
                )
            }
            AsmInstruction::Bne(label, lhs, rhs) => {
                write!(
                    f,
                    "bne   {}, ({})fp, ({})fp",
                    labels.get(label).unwrap_or(&format!(".L{}", label)),
                    lhs,
                    rhs
                )
            }
            AsmInstruction::BneI(label, lhs, rhs) => {
                write!(
                    f,
                    "bnei  {}, ({})fp, {}",
                    labels.get(label).unwrap_or(&format!(".L{}", label)),
                    lhs,
                    rhs
                )
            }
            AsmInstruction::Beq(label, lhs, rhs) => {
                write!(
                    f,
                    "beq  {}, ({})fp, ({})fp",
                    labels.get(label).unwrap_or(&format!(".L{}", label)),
                    lhs,
                    rhs
                )
            }
            AsmInstruction::BeqI(label, lhs, rhs) => {
                write!(
                    f,
                    "beqi {}, ({})fp, {}",
                    labels.get(label).unwrap_or(&format!(".L{}", label)),
                    lhs,
                    rhs
                )
            }
            AsmInstruction::BneE(label, lhs, rhs) => {
                write!(
                    f,
                    "ebne  {}, ({})fp, ({})fp",
                    labels.get(label).unwrap_or(&format!(".L{}", label)),
                    lhs,
                    rhs
                )
            }
            AsmInstruction::BneEI(label, lhs, rhs) => {
                write!(
                    f,
                    "ebnei {}, ({})fp, {}",
                    labels.get(label).unwrap_or(&format!(".L{}", label)),
                    lhs,
                    rhs
                )
            }
            AsmInstruction::BeqE(label, lhs, rhs) => {
                write!(
                    f,
                    "ebeq  {}, ({})fp, ({})fp",
                    labels.get(label).unwrap_or(&format!(".L{}", label)),
                    lhs,
                    rhs
                )
            }
            AsmInstruction::BeqEI(label, lhs, rhs) => {
                write!(
                    f,
                    "ebeqi {}, ({})fp, {}",
                    labels.get(label).unwrap_or(&format!(".L{}", label)),
                    lhs,
                    rhs
                )
            }
            AsmInstruction::Trap => write!(f, "trap"),
            AsmInstruction::Halt => write!(f, "halt"),
            AsmInstruction::HintBits(src, len) => write!(f, "hint_bits ({})fp, {}", src, len),
            AsmInstruction::Poseidon2Permute(dst, lhs) => {
                write!(f, "poseidon2_permute ({})fp, ({})fp", dst, lhs)
            }
            AsmInstruction::Poseidon2Compress(result, src1, src2) => {
                write!(
                    f,
                    "poseidon2_compress ({})fp, ({})fp, ({})fp",
                    result, src1, src2
                )
            }
            AsmInstruction::PrintF(dst) => {
                write!(f, "print_f ({})fp", dst)
            }
            AsmInstruction::PrintV(dst) => {
                write!(f, "print_v ({})fp", dst)
            }
            AsmInstruction::PrintE(dst) => {
                write!(f, "print_e ({})fp", dst)
            }
            AsmInstruction::HintInputVec() => write!(f, "hint_vec"),
            AsmInstruction::StoreHintWordI(dst, offset) => {
                write!(f, "shintw ({})fp {}", dst, offset)
            }
            AsmInstruction::Publish(val, index) => {
                write!(f, "commit ({})fp ({})fp", val, index)
            }
            AsmInstruction::CycleTrackerStart() => {
                write!(f, "cycle_tracker_start")
            }
            AsmInstruction::CycleTrackerEnd() => {
                write!(f, "cycle_tracker_end")
            }
            AsmInstruction::FriReducedOpening(a, b, res, len, alpha, alpha_pow) => {
                write!(
                    f,
                    "fri_mat_opening ({})fp, ({})fp, ({})fp, ({})fp, ({})fp, ({})fp",
                    a, b, res, len, alpha, alpha_pow
                )
            }
        }
    }
}

impl<F: PrimeField32, EF: ExtensionField<F>> AsmInstruction<F, EF> {
    pub fn parse_instruction(line: &str, labels: &BTreeMap<F, String>) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        // Helper function to parse i32 from formatted string
        let parse_i32 = |s: &str| -> Option<i32> {
            s.trim_end_matches(',')
                .trim_end_matches("fp")
                .trim_end_matches(')')
                .trim_start_matches('(')
                .parse()
                .ok()
        };

        // Helper function to parse field element F
        let parse_f_strict = |s: &str| -> Option<F> { s.parse().ok().map(F::from_canonical_u32) };
        let parse_f = |s: &str| -> Option<F> {
            parse_f_strict(
                s.trim_end_matches(',')
                    .trim_end_matches(')')
                    .trim_start_matches('('),
            )
        };

        // Helper function to parse extension field element EF
        let parse_ef = |s: &str| -> Option<EF> {
            // TODO: is it enough?
            s.trim_end_matches(',')
                .parse()
                .ok()
                .map(EF::from_canonical_u64)
        };

        let parse_label = |label: &str| -> Option<F> {
            let label = label.trim_end_matches(',');
            let label_key = labels
                .iter()
                .find(|&(_, v)| v == label)
                .map(|(k, _)| *k)
                .unwrap_or_else(|| parse_f_strict(label.trim_start_matches(".L")).expect(label));
            Some(label_key)
        };

        match parts[0] {
            "lw" => Some(AsmInstruction::LoadF(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
                parse_f(parts[4])?,
                parse_f(parts[5])?,
            )),
            "lwi" => Some(AsmInstruction::LoadFI(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_f(parts[3])?,
                parse_f(parts[4])?,
                parse_f(parts[5])?,
            )),
            "sw" => Some(AsmInstruction::StoreF(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
                parse_f(parts[4])?,
                parse_f(parts[5])?,
            )),
            "swi" => Some(AsmInstruction::StoreFI(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_f(parts[3])?,
                parse_f(parts[4])?,
                parse_f(parts[5])?,
            )),
            "imm" => Some(AsmInstruction::ImmF(
                parse_i32(parts[1])?,
                parse_f(parts[2])?,
            )),
            "copy" => Some(AsmInstruction::CopyF(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
            )),
            "add" => Some(AsmInstruction::AddF(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "addi" => Some(AsmInstruction::AddFI(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_f(parts[3])?,
            )),
            "sub" => Some(AsmInstruction::SubF(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "subi" => Some(AsmInstruction::SubFI(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_f(parts[3])?,
            )),
            "subin" => Some(AsmInstruction::SubFIN(
                parse_i32(parts[1])?,
                parse_f(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "mul" => Some(AsmInstruction::MulF(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "muli" => Some(AsmInstruction::MulFI(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_f(parts[3])?,
            )),
            "div" => Some(AsmInstruction::DivF(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "divi" => Some(AsmInstruction::DivFI(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_f(parts[3])?,
            )),
            "divin" => Some(AsmInstruction::DivFIN(
                parse_i32(parts[1])?,
                parse_f(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "eadd" => Some(AsmInstruction::AddE(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "esub" => Some(AsmInstruction::SubE(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "emul" => Some(AsmInstruction::MulE(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "ediv" => Some(AsmInstruction::DivE(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "j" => {
                let dst = parse_i32(parts[1])?;
                let label_key = parse_label(parts[2])?;
                Some(AsmInstruction::Jump(dst, label_key))
            }
            "bne" => {
                let label_key = parse_label(parts[1])?;
                Some(AsmInstruction::Bne(
                    label_key,
                    parse_i32(parts[2])?,
                    parse_i32(parts[3])?,
                ))
            }
            "bnei" => {
                let label_key = parse_label(parts[1])?;
                Some(AsmInstruction::BneI(
                    label_key,
                    parse_i32(parts[2])?,
                    parse_f(parts[3])?,
                ))
            }
            "beq" => {
                let label_key = parse_label(parts[1])?;
                Some(AsmInstruction::Beq(
                    label_key,
                    parse_i32(parts[2])?,
                    parse_i32(parts[3])?,
                ))
            }
            "beqi" => {
                let label_key = parse_label(parts[1])?;
                Some(AsmInstruction::BeqI(
                    label_key,
                    parse_i32(parts[2])?,
                    parse_f(parts[3])?,
                ))
            }
            "ebne" => {
                let label_key = parse_label(parts[1])?;
                Some(AsmInstruction::BneE(
                    label_key,
                    parse_i32(parts[2])?,
                    parse_i32(parts[3])?,
                ))
            }
            "ebnei" => {
                let label_key = parse_label(parts[1])?;
                Some(AsmInstruction::BneEI(
                    label_key,
                    parse_i32(parts[2])?,
                    parse_ef(parts[3])?,
                ))
            }
            "ebeq" => {
                let label_key = parse_label(parts[1])?;
                Some(AsmInstruction::BeqE(
                    label_key,
                    parse_i32(parts[2])?,
                    parse_i32(parts[3])?,
                ))
            }
            "ebeqi" => {
                let label_key = parse_label(parts[1])?;
                Some(AsmInstruction::BeqEI(
                    label_key,
                    parse_i32(parts[2])?,
                    parse_ef(parts[3])?,
                ))
            }
            "trap" => Some(AsmInstruction::Trap),
            "halt" => Some(AsmInstruction::Halt),
            "hint_bits" => Some(AsmInstruction::HintBits(
                parse_i32(parts[1])?,
                parts[2].parse().ok()?,
            )),
            "poseidon2_permute" => Some(AsmInstruction::Poseidon2Permute(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
            )),
            "poseidon2_compress" => Some(AsmInstruction::Poseidon2Compress(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
            )),
            "print_f" => Some(AsmInstruction::PrintF(parse_i32(parts[1])?)),
            "print_v" => Some(AsmInstruction::PrintV(parse_i32(parts[1])?)),
            "print_e" => Some(AsmInstruction::PrintE(parse_i32(parts[1])?)),
            "hint_vec" => Some(AsmInstruction::HintInputVec()),
            "shintw" => Some(AsmInstruction::StoreHintWordI(
                parse_i32(parts[1])?,
                parse_f(parts[2])?,
            )),
            "commit" => Some(AsmInstruction::Publish(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
            )),
            "cycle_tracker_start" => Some(AsmInstruction::CycleTrackerStart()),
            "cycle_tracker_end" => Some(AsmInstruction::CycleTrackerEnd()),
            "fri_mat_opening" => Some(AsmInstruction::FriReducedOpening(
                parse_i32(parts[1])?,
                parse_i32(parts[2])?,
                parse_i32(parts[3])?,
                parse_i32(parts[4])?,
                parse_i32(parts[5])?,
                parse_i32(parts[6])?,
            )),
            _ => None,
        }
    }
}
