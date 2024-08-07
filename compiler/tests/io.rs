use afs_compiler::{
    asm::{AsmBuilder, AsmCompiler},
    conversion::{convert_program, CompilerOptions},
    util::execute_program_and_generate_traces,
};
use p3_baby_bear::BabyBear;
use p3_field::{extension::BinomialExtensionField, AbstractField};

type F = BabyBear;
type EF = BinomialExtensionField<BabyBear, 4>;

#[test]
fn test_io() {
    let mut builder = AsmBuilder::<F, EF>::default();

    let vars = builder.hint_vars();
    builder.range(0, vars.len()).for_each(|i, builder| {
        let el = builder.get(&vars, i);
        builder.print_v(el);
    });

    let felts = builder.hint_felts();
    builder.range(0, felts.len()).for_each(|i, builder| {
        let el = builder.get(&felts, i);
        builder.print_f(el);
    });

    let exts = builder.hint_exts();
    builder.range(0, exts.len()).for_each(|i, builder| {
        let el = builder.get(&exts, i);
        builder.print_e(el);
    });

    builder.halt();

    let emb = |x| [x, F::zero(), F::zero(), F::zero()];

    let witness_stream: Vec<Vec<[F; 4]>> = vec![
        vec![emb(F::zero()), emb(F::zero()), emb(F::one())],
        vec![emb(F::zero()), emb(F::zero()), emb(F::one())],
        vec![
            [F::one(), F::zero(), F::zero(), F::zero()],
            [F::one(), F::zero(), F::one(), F::zero()],
            [F::two(), F::zero(), F::zero(), F::zero()],
        ],
    ];

    const WORD_SIZE: usize = 4;

    let mut compiler = AsmCompiler::new(WORD_SIZE);
    compiler.build(builder.operations);
    let asm_code = compiler.code();
    println!("{}", asm_code);

    let program = convert_program::<WORD_SIZE, F, EF>(asm_code, CompilerOptions::default());
    execute_program_and_generate_traces::<WORD_SIZE>(program, witness_stream);
}
