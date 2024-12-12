| group | average_cells_used | average_cycles | average_proof_time_ms | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fib_e2e | <div style='text-align: right'>58,738,364.85714286</div>  | <div style='text-align: right'>1,714,305.2857142857</div>  | <div style='text-align: right'>5,060.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>411,168,554</div>  | <div style='text-align: right'>12,000,137</div>  | <div style='text-align: right'>35,420.0</div>  |
| leaf_verifier | <div style='text-align: right'>217,841,608.0</div>  | <div style='text-align: right'>5,326,193.25</div>  | <div style='text-align: right'>18,860.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>871,366,432</div>  | <div style='text-align: right'>21,304,773</div>  | <div style='text-align: right'>75,440.0</div>  |
| internal_verifier_height_0 | <div style='text-align: right'>283,651,385.5</div>  | <div style='text-align: right'>7,206,610.0</div>  | <div style='text-align: right'>25,797.5</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>567,302,771</div>  | <div style='text-align: right'>14,413,220</div>  | <div style='text-align: right'>51,595.0</div>  |
| internal_verifier_height_1 | <div style='text-align: right'>286,176,835.0</div>  | <div style='text-align: right'>7,276,849.0</div>  | <div style='text-align: right'>26,009.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>286,176,835</div>  | <div style='text-align: right'>7,276,849</div>  | <div style='text-align: right'>26,009.0</div>  |
| root_verifier | <div style='text-align: right'>144,237,137.0</div>  | <div style='text-align: right'>3,636,475.0</div>  | <div style='text-align: right'>73,444.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>144,237,137</div>  | <div style='text-align: right'>3,636,475</div>  | <div style='text-align: right'>73,444.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| fib_e2e | 0 | <div style='text-align: right'>5,238.0</div>  | <div style='text-align: right'>197,682,718</div>  | <div style='text-align: right'>59,910,473</div>  | <div style='text-align: right'>1,747,603</div>  | <div style='text-align: right'>256.0</div>  |
| fib_e2e | 1 | <div style='text-align: right'>4,868.0</div>  | <div style='text-align: right'>197,505,474</div>  | <div style='text-align: right'>59,836,230</div>  | <div style='text-align: right'>1,747,502</div>  | <div style='text-align: right'>277.0</div>  |
| fib_e2e | 2 | <div style='text-align: right'>5,139.0</div>  | <div style='text-align: right'>197,505,474</div>  | <div style='text-align: right'>59,836,221</div>  | <div style='text-align: right'>1,747,502</div>  | <div style='text-align: right'>297.0</div>  |
| fib_e2e | 3 | <div style='text-align: right'>5,080.0</div>  | <div style='text-align: right'>197,505,474</div>  | <div style='text-align: right'>59,836,240</div>  | <div style='text-align: right'>1,747,502</div>  | <div style='text-align: right'>297.0</div>  |
| fib_e2e | 4 | <div style='text-align: right'>5,029.0</div>  | <div style='text-align: right'>197,505,474</div>  | <div style='text-align: right'>59,836,239</div>  | <div style='text-align: right'>1,747,502</div>  | <div style='text-align: right'>292.0</div>  |
| fib_e2e | 5 | <div style='text-align: right'>5,044.0</div>  | <div style='text-align: right'>197,505,474</div>  | <div style='text-align: right'>59,836,229</div>  | <div style='text-align: right'>1,747,502</div>  | <div style='text-align: right'>282.0</div>  |
| fib_e2e | 6 | <div style='text-align: right'>5,022.0</div>  | <div style='text-align: right'>197,514,258</div>  | <div style='text-align: right'>52,076,922</div>  | <div style='text-align: right'>1,515,024</div>  | <div style='text-align: right'>263.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| fib_e2e | ProgramChip | 0 | <div style='text-align: right'>3,335</div>  |
| fib_e2e | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| fib_e2e | Boundary | 0 | <div style='text-align: right'>34</div>  |
| fib_e2e | Merkle | 0 | <div style='text-align: right'>226</div>  |
| fib_e2e | AccessAdapter<8> | 0 | <div style='text-align: right'>34</div>  |
| fib_e2e | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>3</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| fib_e2e | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>9</div>  |
| fib_e2e | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <div style='text-align: right'>12</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>116,508</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>5</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <div style='text-align: right'>233,005</div>  |
| fib_e2e | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <div style='text-align: right'>23</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <div style='text-align: right'>2</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <div style='text-align: right'>349,497</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <div style='text-align: right'>1,048,537</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| fib_e2e | PhantomAir | 0 | <div style='text-align: right'>2</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>260</div>  |
| fib_e2e | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramChip | 1 | <div style='text-align: right'>3,335</div>  |
| fib_e2e | VmConnectorAir | 1 | <div style='text-align: right'>2</div>  |
| fib_e2e | Boundary | 1 | <div style='text-align: right'>16</div>  |
| fib_e2e | Merkle | 1 | <div style='text-align: right'>124</div>  |
| fib_e2e | AccessAdapter<8> | 1 | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 1 | <div style='text-align: right'>524,288</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 1 | <div style='text-align: right'>116,500</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 1 | <div style='text-align: right'>233,001</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 1 | <div style='text-align: right'>349,501</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 1 | <div style='text-align: right'>1,048,500</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 1 | <div style='text-align: right'>65,536</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>140</div>  |
| fib_e2e | VariableRangeCheckerAir | 1 | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramChip | 2 | <div style='text-align: right'>3,335</div>  |
| fib_e2e | VmConnectorAir | 2 | <div style='text-align: right'>2</div>  |
| fib_e2e | Boundary | 2 | <div style='text-align: right'>16</div>  |
| fib_e2e | Merkle | 2 | <div style='text-align: right'>124</div>  |
| fib_e2e | AccessAdapter<8> | 2 | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 2 | <div style='text-align: right'>524,288</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 2 | <div style='text-align: right'>116,501</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 2 | <div style='text-align: right'>233,000</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 2 | <div style='text-align: right'>349,500</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 2 | <div style='text-align: right'>1,048,501</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 2 | <div style='text-align: right'>65,536</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>140</div>  |
| fib_e2e | VariableRangeCheckerAir | 2 | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramChip | 3 | <div style='text-align: right'>3,335</div>  |
| fib_e2e | VmConnectorAir | 3 | <div style='text-align: right'>2</div>  |
| fib_e2e | Boundary | 3 | <div style='text-align: right'>16</div>  |
| fib_e2e | Merkle | 3 | <div style='text-align: right'>124</div>  |
| fib_e2e | AccessAdapter<8> | 3 | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 3 | <div style='text-align: right'>524,288</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 3 | <div style='text-align: right'>116,500</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 3 | <div style='text-align: right'>233,000</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 3 | <div style='text-align: right'>349,501</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 3 | <div style='text-align: right'>1,048,501</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 3 | <div style='text-align: right'>65,536</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>140</div>  |
| fib_e2e | VariableRangeCheckerAir | 3 | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramChip | 4 | <div style='text-align: right'>3,335</div>  |
| fib_e2e | VmConnectorAir | 4 | <div style='text-align: right'>2</div>  |
| fib_e2e | Boundary | 4 | <div style='text-align: right'>16</div>  |
| fib_e2e | Merkle | 4 | <div style='text-align: right'>124</div>  |
| fib_e2e | AccessAdapter<8> | 4 | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 4 | <div style='text-align: right'>524,288</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 4 | <div style='text-align: right'>116,500</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 4 | <div style='text-align: right'>233,000</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 4 | <div style='text-align: right'>349,500</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 4 | <div style='text-align: right'>1,048,502</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 4 | <div style='text-align: right'>65,536</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 4 | <div style='text-align: right'>140</div>  |
| fib_e2e | VariableRangeCheckerAir | 4 | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramChip | 5 | <div style='text-align: right'>3,335</div>  |
| fib_e2e | VmConnectorAir | 5 | <div style='text-align: right'>2</div>  |
| fib_e2e | Boundary | 5 | <div style='text-align: right'>16</div>  |
| fib_e2e | Merkle | 5 | <div style='text-align: right'>124</div>  |
| fib_e2e | AccessAdapter<8> | 5 | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 5 | <div style='text-align: right'>524,288</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 5 | <div style='text-align: right'>116,500</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 5 | <div style='text-align: right'>233,001</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 5 | <div style='text-align: right'>349,500</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 5 | <div style='text-align: right'>1,048,501</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 5 | <div style='text-align: right'>65,536</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 5 | <div style='text-align: right'>140</div>  |
| fib_e2e | VariableRangeCheckerAir | 5 | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramChip | 6 | <div style='text-align: right'>3,335</div>  |
| fib_e2e | VmConnectorAir | 6 | <div style='text-align: right'>2</div>  |
| fib_e2e | Boundary | 6 | <div style='text-align: right'>24</div>  |
| fib_e2e | Merkle | 6 | <div style='text-align: right'>230</div>  |
| fib_e2e | AccessAdapter<8> | 6 | <div style='text-align: right'>24</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 6 | <div style='text-align: right'>524,288</div>  |
| fib_e2e | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 6 | <div style='text-align: right'>1</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 6 | <div style='text-align: right'>101,001</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 6 | <div style='text-align: right'>202,002</div>  |
| fib_e2e | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 6 | <div style='text-align: right'>5</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 6 | <div style='text-align: right'>303,003</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 6 | <div style='text-align: right'>909,012</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 6 | <div style='text-align: right'>65,536</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 6 | <div style='text-align: right'>254</div>  |
| fib_e2e | VariableRangeCheckerAir | 6 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| fib_e2e |  | ADD | 0 | <div style='text-align: right'>1,048,528</div>  |
| fib_e2e |  | AND | 0 | <div style='text-align: right'>2</div>  |
| fib_e2e |  | AUIPC | 0 | <div style='text-align: right'>9</div>  |
| fib_e2e |  | BEQ | 0 | <div style='text-align: right'>116,502</div>  |
| fib_e2e |  | BGEU | 0 | <div style='text-align: right'>3</div>  |
| fib_e2e |  | BLTU | 0 | <div style='text-align: right'>2</div>  |
| fib_e2e |  | BNE | 0 | <div style='text-align: right'>116,503</div>  |
| fib_e2e |  | HINT_STOREW | 0 | <div style='text-align: right'>3</div>  |
| fib_e2e |  | JAL | 0 | <div style='text-align: right'>116,499</div>  |
| fib_e2e |  | JALR | 0 | <div style='text-align: right'>12</div>  |
| fib_e2e |  | LOADW | 0 | <div style='text-align: right'>10</div>  |
| fib_e2e |  | LUI | 0 | <div style='text-align: right'>9</div>  |
| fib_e2e |  | OR | 0 | <div style='text-align: right'>1</div>  |
| fib_e2e |  | PHANTOM | 0 | <div style='text-align: right'>2</div>  |
| fib_e2e |  | SLL | 0 | <div style='text-align: right'>2</div>  |
| fib_e2e |  | SLTU | 0 | <div style='text-align: right'>349,497</div>  |
| fib_e2e |  | STOREW | 0 | <div style='text-align: right'>13</div>  |
| fib_e2e |  | SUB | 0 | <div style='text-align: right'>4</div>  |
| fib_e2e |  | XOR | 0 | <div style='text-align: right'>2</div>  |
| fib_e2e |  | ADD | 1 | <div style='text-align: right'>1,048,500</div>  |
| fib_e2e |  | BEQ | 1 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | BNE | 1 | <div style='text-align: right'>116,501</div>  |
| fib_e2e |  | JAL | 1 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | SLTU | 1 | <div style='text-align: right'>349,501</div>  |
| fib_e2e |  | ADD | 2 | <div style='text-align: right'>1,048,501</div>  |
| fib_e2e |  | BEQ | 2 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | BNE | 2 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | JAL | 2 | <div style='text-align: right'>116,501</div>  |
| fib_e2e |  | SLTU | 2 | <div style='text-align: right'>349,500</div>  |
| fib_e2e |  | ADD | 3 | <div style='text-align: right'>1,048,501</div>  |
| fib_e2e |  | BEQ | 3 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | BNE | 3 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | JAL | 3 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | SLTU | 3 | <div style='text-align: right'>349,501</div>  |
| fib_e2e |  | ADD | 4 | <div style='text-align: right'>1,048,502</div>  |
| fib_e2e |  | BEQ | 4 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | BNE | 4 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | JAL | 4 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | SLTU | 4 | <div style='text-align: right'>349,500</div>  |
| fib_e2e |  | ADD | 5 | <div style='text-align: right'>1,048,501</div>  |
| fib_e2e |  | BEQ | 5 | <div style='text-align: right'>116,501</div>  |
| fib_e2e |  | BNE | 5 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | JAL | 5 | <div style='text-align: right'>116,500</div>  |
| fib_e2e |  | SLTU | 5 | <div style='text-align: right'>349,500</div>  |
| fib_e2e |  | ADD | 6 | <div style='text-align: right'>909,012</div>  |
| fib_e2e |  | BEQ | 6 | <div style='text-align: right'>101,001</div>  |
| fib_e2e |  | BNE | 6 | <div style='text-align: right'>101,001</div>  |
| fib_e2e |  | JAL | 6 | <div style='text-align: right'>101,001</div>  |
| fib_e2e |  | JALR | 6 | <div style='text-align: right'>1</div>  |
| fib_e2e |  | LOADW | 6 | <div style='text-align: right'>3</div>  |
| fib_e2e |  | SLTU | 6 | <div style='text-align: right'>303,003</div>  |
| fib_e2e |  | STOREW | 6 | <div style='text-align: right'>2</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <div style='text-align: right'>37,747,008</div>  |
| fib_e2e | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>68</div>  |
| fib_e2e | Boundary |  | ADD | 0 | <div style='text-align: right'>160</div>  |
| fib_e2e | Merkle |  | ADD | 0 | <div style='text-align: right'>320</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <div style='text-align: right'>72</div>  |
| fib_e2e | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <div style='text-align: right'>189</div>  |
| fib_e2e | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | AUIPC | 0 | <div style='text-align: right'>80</div>  |
| fib_e2e | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,456</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <div style='text-align: right'>3,029,052</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>96</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <div style='text-align: right'>64</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <div style='text-align: right'>3,029,078</div>  |
| fib_e2e | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>78</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <div style='text-align: right'>2,096,982</div>  |
| fib_e2e | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <div style='text-align: right'>336</div>  |
| fib_e2e | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <div style='text-align: right'>400</div>  |
| fib_e2e | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | LOADW | 0 | <div style='text-align: right'>80</div>  |
| fib_e2e | Merkle |  | LOADW | 0 | <div style='text-align: right'>2,304</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <div style='text-align: right'>162</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <div style='text-align: right'>36</div>  |
| fib_e2e | PhantomAir |  | PHANTOM | 0 | <div style='text-align: right'>12</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <div style='text-align: right'>106</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <div style='text-align: right'>12,931,389</div>  |
| fib_e2e | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | SLTU | 0 | <div style='text-align: right'>80</div>  |
| fib_e2e | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <div style='text-align: right'>520</div>  |
| fib_e2e | AccessAdapter<8> |  | STOREW | 0 | <div style='text-align: right'>119</div>  |
| fib_e2e | Boundary |  | STOREW | 0 | <div style='text-align: right'>280</div>  |
| fib_e2e | Merkle |  | STOREW | 0 | <div style='text-align: right'>1,088</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>144</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <div style='text-align: right'>72</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 1 | <div style='text-align: right'>37,746,000</div>  |
| fib_e2e | AccessAdapter<8> |  | ADD | 1 | <div style='text-align: right'>17</div>  |
| fib_e2e | Boundary |  | ADD | 1 | <div style='text-align: right'>40</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 1 | <div style='text-align: right'>3,029,000</div>  |
| fib_e2e | AccessAdapter<8> |  | BEQ | 1 | <div style='text-align: right'>17</div>  |
| fib_e2e | Boundary |  | BEQ | 1 | <div style='text-align: right'>40</div>  |
| fib_e2e | Merkle |  | BEQ | 1 | <div style='text-align: right'>64</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 1 | <div style='text-align: right'>3,029,026</div>  |
| fib_e2e | AccessAdapter<8> |  | BNE | 1 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | BNE | 1 | <div style='text-align: right'>80</div>  |
| fib_e2e | Merkle |  | BNE | 1 | <div style='text-align: right'>3,648</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 1 | <div style='text-align: right'>2,097,000</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 1 | <div style='text-align: right'>12,931,537</div>  |
| fib_e2e | AccessAdapter<8> |  | SLTU | 1 | <div style='text-align: right'>68</div>  |
| fib_e2e | Boundary |  | SLTU | 1 | <div style='text-align: right'>160</div>  |
| fib_e2e | Merkle |  | SLTU | 1 | <div style='text-align: right'>192</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 2 | <div style='text-align: right'>37,746,036</div>  |
| fib_e2e | AccessAdapter<8> |  | ADD | 2 | <div style='text-align: right'>68</div>  |
| fib_e2e | Boundary |  | ADD | 2 | <div style='text-align: right'>160</div>  |
| fib_e2e | Merkle |  | ADD | 2 | <div style='text-align: right'>3,712</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 2 | <div style='text-align: right'>3,029,000</div>  |
| fib_e2e | AccessAdapter<8> |  | BEQ | 2 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | BEQ | 2 | <div style='text-align: right'>80</div>  |
| fib_e2e | Merkle |  | BEQ | 2 | <div style='text-align: right'>64</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 2 | <div style='text-align: right'>3,029,000</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 2 | <div style='text-align: right'>2,097,018</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 2 | <div style='text-align: right'>12,931,500</div>  |
| fib_e2e | AccessAdapter<8> |  | SLTU | 2 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | SLTU | 2 | <div style='text-align: right'>80</div>  |
| fib_e2e | Merkle |  | SLTU | 2 | <div style='text-align: right'>128</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 3 | <div style='text-align: right'>37,746,036</div>  |
| fib_e2e | AccessAdapter<8> |  | ADD | 3 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | ADD | 3 | <div style='text-align: right'>80</div>  |
| fib_e2e | Merkle |  | ADD | 3 | <div style='text-align: right'>64</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 3 | <div style='text-align: right'>3,029,000</div>  |
| fib_e2e | AccessAdapter<8> |  | BEQ | 3 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | BEQ | 3 | <div style='text-align: right'>80</div>  |
| fib_e2e | Merkle |  | BEQ | 3 | <div style='text-align: right'>64</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 3 | <div style='text-align: right'>3,029,000</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 3 | <div style='text-align: right'>2,097,000</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 3 | <div style='text-align: right'>12,931,537</div>  |
| fib_e2e | AccessAdapter<8> |  | SLTU | 3 | <div style='text-align: right'>68</div>  |
| fib_e2e | Boundary |  | SLTU | 3 | <div style='text-align: right'>160</div>  |
| fib_e2e | Merkle |  | SLTU | 3 | <div style='text-align: right'>3,776</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 4 | <div style='text-align: right'>37,746,072</div>  |
| fib_e2e | AccessAdapter<8> |  | ADD | 4 | <div style='text-align: right'>85</div>  |
| fib_e2e | Boundary |  | ADD | 4 | <div style='text-align: right'>200</div>  |
| fib_e2e | Merkle |  | ADD | 4 | <div style='text-align: right'>3,840</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 4 | <div style='text-align: right'>3,029,000</div>  |
| fib_e2e | AccessAdapter<8> |  | BEQ | 4 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | BEQ | 4 | <div style='text-align: right'>80</div>  |
| fib_e2e | Merkle |  | BEQ | 4 | <div style='text-align: right'>64</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 4 | <div style='text-align: right'>3,029,000</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 4 | <div style='text-align: right'>2,097,000</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 4 | <div style='text-align: right'>12,931,500</div>  |
| fib_e2e | AccessAdapter<8> |  | SLTU | 4 | <div style='text-align: right'>17</div>  |
| fib_e2e | Boundary |  | SLTU | 4 | <div style='text-align: right'>40</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 5 | <div style='text-align: right'>37,746,036</div>  |
| fib_e2e | AccessAdapter<8> |  | ADD | 5 | <div style='text-align: right'>85</div>  |
| fib_e2e | Boundary |  | ADD | 5 | <div style='text-align: right'>200</div>  |
| fib_e2e | Merkle |  | ADD | 5 | <div style='text-align: right'>3,712</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 5 | <div style='text-align: right'>3,029,026</div>  |
| fib_e2e | AccessAdapter<8> |  | BEQ | 5 | <div style='text-align: right'>34</div>  |
| fib_e2e | Boundary |  | BEQ | 5 | <div style='text-align: right'>80</div>  |
| fib_e2e | Merkle |  | BEQ | 5 | <div style='text-align: right'>192</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 5 | <div style='text-align: right'>3,029,000</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 5 | <div style='text-align: right'>2,097,000</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 5 | <div style='text-align: right'>12,931,500</div>  |
| fib_e2e | AccessAdapter<8> |  | SLTU | 5 | <div style='text-align: right'>17</div>  |
| fib_e2e | Boundary |  | SLTU | 5 | <div style='text-align: right'>40</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 6 | <div style='text-align: right'>32,724,432</div>  |
| fib_e2e | AccessAdapter<8> |  | ADD | 6 | <div style='text-align: right'>68</div>  |
| fib_e2e | Boundary |  | ADD | 6 | <div style='text-align: right'>160</div>  |
| fib_e2e | Merkle |  | ADD | 6 | <div style='text-align: right'>3,776</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 6 | <div style='text-align: right'>2,626,026</div>  |
| fib_e2e | AccessAdapter<8> |  | BEQ | 6 | <div style='text-align: right'>17</div>  |
| fib_e2e | Boundary |  | BEQ | 6 | <div style='text-align: right'>40</div>  |
| fib_e2e | Merkle |  | BEQ | 6 | <div style='text-align: right'>64</div>  |
| fib_e2e | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 6 | <div style='text-align: right'>2,626,026</div>  |
| fib_e2e | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 6 | <div style='text-align: right'>1,818,018</div>  |
| fib_e2e | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 6 | <div style='text-align: right'>28</div>  |
| fib_e2e | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 6 | <div style='text-align: right'>120</div>  |
| fib_e2e | AccessAdapter<8> |  | LOADW | 6 | <div style='text-align: right'>51</div>  |
| fib_e2e | Boundary |  | LOADW | 6 | <div style='text-align: right'>120</div>  |
| fib_e2e | Merkle |  | LOADW | 6 | <div style='text-align: right'>1,664</div>  |
| fib_e2e | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 6 | <div style='text-align: right'>11,211,111</div>  |
| fib_e2e | AccessAdapter<8> |  | SLTU | 6 | <div style='text-align: right'>51</div>  |
| fib_e2e | Boundary |  | SLTU | 6 | <div style='text-align: right'>120</div>  |
| fib_e2e | Merkle |  | SLTU | 6 | <div style='text-align: right'>64</div>  |
| fib_e2e | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 6 | <div style='text-align: right'>80</div>  |
| fib_e2e | AccessAdapter<8> |  | STOREW | 6 | <div style='text-align: right'>17</div>  |
| fib_e2e | Boundary |  | STOREW | 6 | <div style='text-align: right'>40</div>  |
| fib_e2e | Merkle |  | STOREW | 6 | <div style='text-align: right'>1,728</div>  |

| group | average_cells_used | average_cycles | average_proof_time_ms | execute_time_ms | fri.log_blowup | halo2_proof_time_ms | halo2_total_cells | num_segments_or_nodes | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fib_e2e | <div style='text-align: right'>58,738,364.85714286</div>  | <div style='text-align: right'>1,714,305.2857142857</div>  | <div style='text-align: right'>5,060.0</div>  | <div style='text-align: right'>7,327.0</div>  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>7</div>  |  |  | <div style='text-align: right'>411,168,554</div>  | <div style='text-align: right'>12,000,137</div>  | <div style='text-align: right'>35,420.0</div>  |
| leaf_verifier | <div style='text-align: right'>217,841,608.0</div>  | <div style='text-align: right'>5,326,193.25</div>  | <div style='text-align: right'>18,860.0</div>  |  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>4</div>  |  |  | <div style='text-align: right'>871,366,432</div>  | <div style='text-align: right'>21,304,773</div>  | <div style='text-align: right'>75,440.0</div>  |
| internal_verifier_height_0 | <div style='text-align: right'>283,651,385.5</div>  | <div style='text-align: right'>7,206,610.0</div>  | <div style='text-align: right'>25,797.5</div>  |  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>567,302,771</div>  | <div style='text-align: right'>14,413,220</div>  | <div style='text-align: right'>51,595.0</div>  |
| internal_verifier_height_1 | <div style='text-align: right'>286,176,835.0</div>  | <div style='text-align: right'>7,276,849.0</div>  | <div style='text-align: right'>26,009.0</div>  |  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>1</div>  |  |  | <div style='text-align: right'>286,176,835</div>  | <div style='text-align: right'>7,276,849</div>  | <div style='text-align: right'>26,009.0</div>  |
| root_verifier | <div style='text-align: right'>144,237,137.0</div>  | <div style='text-align: right'>3,636,475.0</div>  | <div style='text-align: right'>73,444.0</div>  |  | <div style='text-align: right'>2</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>73,444.0</div>  | <div style='text-align: right'>383,945,176</div>  | <div style='text-align: right'>144,237,137</div>  | <div style='text-align: right'>3,636,475</div>  | <div style='text-align: right'>73,444.0</div>  |
| halo2_verifier |  |  |  |  |  | <div style='text-align: right'>404,915.0</div>  | <div style='text-align: right'>318,500,970.0</div>  |  |  |  |  |  |  |
| halo2_wrapper |  |  |  |  |  | <div style='text-align: right'>80,277.0</div>  |  |  |  |  |  |  |  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fib_e2e | ProgramAir | 0 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| fib_e2e | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fib_e2e | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| fib_e2e | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| fib_e2e | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,624</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| fib_e2e | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>704</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>8</div>  |
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>3,584</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>32</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>210</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>2</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fib_e2e | PhantomAir | 0 | <div style='text-align: right'>36</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>2</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>321,024</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>512</div>  |
| fib_e2e | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramAir | 1 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| fib_e2e | VmConnectorAir | 1 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fib_e2e | PersistentBoundaryAir<8> | 1 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | MemoryMerkleAir<8> | 1 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fib_e2e | AccessAdapterAir<8> | 1 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 1 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 1 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 1 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 1 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 1 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 1 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fib_e2e | PhantomAir | 1 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fib_e2e | VariableRangeCheckerAir | 1 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramAir | 2 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| fib_e2e | VmConnectorAir | 2 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fib_e2e | PersistentBoundaryAir<8> | 2 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | MemoryMerkleAir<8> | 2 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fib_e2e | AccessAdapterAir<8> | 2 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 2 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 2 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fib_e2e | PhantomAir | 2 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fib_e2e | VariableRangeCheckerAir | 2 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramAir | 3 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| fib_e2e | VmConnectorAir | 3 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fib_e2e | PersistentBoundaryAir<8> | 3 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | MemoryMerkleAir<8> | 3 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fib_e2e | AccessAdapterAir<8> | 3 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 3 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 3 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 3 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 3 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 3 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 3 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fib_e2e | PhantomAir | 3 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fib_e2e | VariableRangeCheckerAir | 3 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramAir | 4 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| fib_e2e | VmConnectorAir | 4 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fib_e2e | PersistentBoundaryAir<8> | 4 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | MemoryMerkleAir<8> | 4 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fib_e2e | AccessAdapterAir<8> | 4 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 4 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 4 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 4 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 4 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 4 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 4 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fib_e2e | PhantomAir | 4 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 4 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fib_e2e | VariableRangeCheckerAir | 4 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramAir | 5 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| fib_e2e | VmConnectorAir | 5 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fib_e2e | PersistentBoundaryAir<8> | 5 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | MemoryMerkleAir<8> | 5 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fib_e2e | AccessAdapterAir<8> | 5 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 5 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 5 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 5 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 5 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 5 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 5 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fib_e2e | PhantomAir | 5 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 5 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fib_e2e | VariableRangeCheckerAir | 5 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | ProgramAir | 6 | <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>4,096</div>  |
| fib_e2e | VmConnectorAir | 6 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fib_e2e | PersistentBoundaryAir<8> | 6 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32</div>  |
| fib_e2e | MemoryMerkleAir<8> | 6 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| fib_e2e | AccessAdapterAir<8> | 6 | <div style='text-align: right'>1,312</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32</div>  |
| fib_e2e | RangeTupleCheckerAir<2> | 6 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 6 | <div style='text-align: right'>64</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>1</div>  |
| fib_e2e | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 6 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fib_e2e | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 6 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fib_e2e | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 6 | <div style='text-align: right'>896</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>8</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 6 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fib_e2e | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 6 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fib_e2e | BitwiseOperationLookupAir<8> | 6 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fib_e2e | PhantomAir | 6 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fib_e2e | Poseidon2VmAir<BabyBearParameters> | 6 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fib_e2e | VariableRangeCheckerAir | 6 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>6,818.0</div>  | <div style='text-align: right'>263,827,753</div>  | <div style='text-align: right'>6,452,518</div>  |
| leaf_verifier | 1 | <div style='text-align: right'>6,642.0</div>  | <div style='text-align: right'>239,357,472</div>  | <div style='text-align: right'>5,859,356</div>  |
| leaf_verifier | 2 | <div style='text-align: right'>6,700.0</div>  | <div style='text-align: right'>239,393,942</div>  | <div style='text-align: right'>5,862,513</div>  |
| leaf_verifier | 3 | <div style='text-align: right'>3,716.0</div>  | <div style='text-align: right'>128,787,265</div>  | <div style='text-align: right'>3,130,386</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>108,928</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>761,706</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>726,362</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>363,602</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>111,074</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>53,183</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>247,128</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>62,340</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,504,330</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>137,616</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,235,961</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,080,011</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>369,885</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 1 | <div style='text-align: right'>108,928</div>  |
| leaf_verifier | VmConnectorAir | 1 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 1 | <div style='text-align: right'>645,325</div>  |
| leaf_verifier | AccessAdapter<2> | 1 | <div style='text-align: right'>666,172</div>  |
| leaf_verifier | AccessAdapter<4> | 1 | <div style='text-align: right'>333,424</div>  |
| leaf_verifier | AccessAdapter<8> | 1 | <div style='text-align: right'>104,684</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>49,988</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | <div style='text-align: right'>54,542</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | <div style='text-align: right'>2,299,710</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 1 | <div style='text-align: right'>128,182</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | <div style='text-align: right'>1,113,924</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 1 | <div style='text-align: right'>1,888,227</div>  |
| leaf_verifier | PhantomAir | 1 | <div style='text-align: right'>317,103</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 2 | <div style='text-align: right'>108,928</div>  |
| leaf_verifier | VmConnectorAir | 2 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 2 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 2 | <div style='text-align: right'>645,325</div>  |
| leaf_verifier | AccessAdapter<2> | 2 | <div style='text-align: right'>666,452</div>  |
| leaf_verifier | AccessAdapter<4> | 2 | <div style='text-align: right'>333,564</div>  |
| leaf_verifier | AccessAdapter<8> | 2 | <div style='text-align: right'>104,684</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>49,988</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2 | <div style='text-align: right'>54,542</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 2 | <div style='text-align: right'>2,299,710</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 2 | <div style='text-align: right'>131,339</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 2 | <div style='text-align: right'>1,113,924</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 2 | <div style='text-align: right'>1,888,227</div>  |
| leaf_verifier | PhantomAir | 2 | <div style='text-align: right'>317,103</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramChip | 3 | <div style='text-align: right'>108,928</div>  |
| leaf_verifier | VmConnectorAir | 3 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 3 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 3 | <div style='text-align: right'>398,921</div>  |
| leaf_verifier | AccessAdapter<2> | 3 | <div style='text-align: right'>366,024</div>  |
| leaf_verifier | AccessAdapter<4> | 3 | <div style='text-align: right'>183,222</div>  |
| leaf_verifier | AccessAdapter<8> | 3 | <div style='text-align: right'>54,594</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>26,014</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | <div style='text-align: right'>117,852</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 3 | <div style='text-align: right'>29,812</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 3 | <div style='text-align: right'>1,219,335</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 3 | <div style='text-align: right'>68,210</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 3 | <div style='text-align: right'>596,433</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 3 | <div style='text-align: right'>1,009,049</div>  |
| leaf_verifier | PhantomAir | 3 | <div style='text-align: right'>177,171</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | ADD | <div style='text-align: right'>2,128,184</div>  |
| leaf_verifier |  | 0 | BBE4DIV | <div style='text-align: right'>10,994</div>  |
| leaf_verifier |  | 0 | BBE4MUL | <div style='text-align: right'>21,182</div>  |
| leaf_verifier |  | 0 | BEQ | <div style='text-align: right'>36,957</div>  |
| leaf_verifier |  | 0 | BNE | <div style='text-align: right'>1,199,004</div>  |
| leaf_verifier |  | 0 | COMP_POS2 | <div style='text-align: right'>34,146</div>  |
| leaf_verifier |  | 0 | DIV | <div style='text-align: right'>214</div>  |
| leaf_verifier |  | 0 | FE4ADD | <div style='text-align: right'>23,310</div>  |
| leaf_verifier |  | 0 | FE4SUB | <div style='text-align: right'>6,854</div>  |
| leaf_verifier |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,156</div>  |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>137,616</div>  |
| leaf_verifier |  | 0 | LOADW | <div style='text-align: right'>294,141</div>  |
| leaf_verifier |  | 0 | LOADW2 | <div style='text-align: right'>638,702</div>  |
| leaf_verifier |  | 0 | MUL | <div style='text-align: right'>269,658</div>  |
| leaf_verifier |  | 0 | PERM_POS2 | <div style='text-align: right'>19,037</div>  |
| leaf_verifier |  | 0 | PHANTOM | <div style='text-align: right'>369,885</div>  |
| leaf_verifier |  | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 0 | SHINTW | <div style='text-align: right'>462,004</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>363,888</div>  |
| leaf_verifier |  | 0 | STOREW2 | <div style='text-align: right'>321,276</div>  |
| leaf_verifier |  | 0 | SUB | <div style='text-align: right'>106,274</div>  |
| leaf_verifier |  | 1 | ADD | <div style='text-align: right'>1,950,560</div>  |
| leaf_verifier |  | 1 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 1 | BBE4MUL | <div style='text-align: right'>18,734</div>  |
| leaf_verifier |  | 1 | BEQ | <div style='text-align: right'>36,549</div>  |
| leaf_verifier |  | 1 | BNE | <div style='text-align: right'>1,077,375</div>  |
| leaf_verifier |  | 1 | COMP_POS2 | <div style='text-align: right'>33,768</div>  |
| leaf_verifier |  | 1 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 1 | FE4ADD | <div style='text-align: right'>19,762</div>  |
| leaf_verifier |  | 1 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 1 | JAL | <div style='text-align: right'>128,182</div>  |
| leaf_verifier |  | 1 | LOADW | <div style='text-align: right'>281,139</div>  |
| leaf_verifier |  | 1 | LOADW2 | <div style='text-align: right'>552,476</div>  |
| leaf_verifier |  | 1 | MUL | <div style='text-align: right'>252,640</div>  |
| leaf_verifier |  | 1 | PERM_POS2 | <div style='text-align: right'>16,220</div>  |
| leaf_verifier |  | 1 | PHANTOM | <div style='text-align: right'>317,103</div>  |
| leaf_verifier |  | 1 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 1 | SHINTW | <div style='text-align: right'>434,578</div>  |
| leaf_verifier |  | 1 | STOREW | <div style='text-align: right'>336,642</div>  |
| leaf_verifier |  | 1 | STOREW2 | <div style='text-align: right'>283,392</div>  |
| leaf_verifier |  | 1 | SUB | <div style='text-align: right'>96,338</div>  |
| leaf_verifier |  | 2 | ADD | <div style='text-align: right'>1,950,560</div>  |
| leaf_verifier |  | 2 | BBE4DIV | <div style='text-align: right'>9,452</div>  |
| leaf_verifier |  | 2 | BBE4MUL | <div style='text-align: right'>18,734</div>  |
| leaf_verifier |  | 2 | BEQ | <div style='text-align: right'>36,549</div>  |
| leaf_verifier |  | 2 | BNE | <div style='text-align: right'>1,077,375</div>  |
| leaf_verifier |  | 2 | COMP_POS2 | <div style='text-align: right'>33,768</div>  |
| leaf_verifier |  | 2 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier |  | 2 | FE4ADD | <div style='text-align: right'>19,762</div>  |
| leaf_verifier |  | 2 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier |  | 2 | JAL | <div style='text-align: right'>131,339</div>  |
| leaf_verifier |  | 2 | LOADW | <div style='text-align: right'>281,139</div>  |
| leaf_verifier |  | 2 | LOADW2 | <div style='text-align: right'>552,476</div>  |
| leaf_verifier |  | 2 | MUL | <div style='text-align: right'>252,640</div>  |
| leaf_verifier |  | 2 | PERM_POS2 | <div style='text-align: right'>16,220</div>  |
| leaf_verifier |  | 2 | PHANTOM | <div style='text-align: right'>317,103</div>  |
| leaf_verifier |  | 2 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 2 | SHINTW | <div style='text-align: right'>434,578</div>  |
| leaf_verifier |  | 2 | STOREW | <div style='text-align: right'>336,642</div>  |
| leaf_verifier |  | 2 | STOREW2 | <div style='text-align: right'>283,392</div>  |
| leaf_verifier |  | 2 | SUB | <div style='text-align: right'>96,338</div>  |
| leaf_verifier |  | 3 | ADD | <div style='text-align: right'>1,036,767</div>  |
| leaf_verifier |  | 3 | BBE4DIV | <div style='text-align: right'>5,240</div>  |
| leaf_verifier |  | 3 | BBE4MUL | <div style='text-align: right'>10,143</div>  |
| leaf_verifier |  | 3 | BEQ | <div style='text-align: right'>18,285</div>  |
| leaf_verifier |  | 3 | BNE | <div style='text-align: right'>578,148</div>  |
| leaf_verifier |  | 3 | COMP_POS2 | <div style='text-align: right'>16,937</div>  |
| leaf_verifier |  | 3 | DIV | <div style='text-align: right'>100</div>  |
| leaf_verifier |  | 3 | FE4ADD | <div style='text-align: right'>11,059</div>  |
| leaf_verifier |  | 3 | FE4SUB | <div style='text-align: right'>3,370</div>  |
| leaf_verifier |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>4,326</div>  |
| leaf_verifier |  | 3 | JAL | <div style='text-align: right'>68,210</div>  |
| leaf_verifier |  | 3 | LOADW | <div style='text-align: right'>147,367</div>  |
| leaf_verifier |  | 3 | LOADW2 | <div style='text-align: right'>303,734</div>  |
| leaf_verifier |  | 3 | MUL | <div style='text-align: right'>131,435</div>  |
| leaf_verifier |  | 3 | PERM_POS2 | <div style='text-align: right'>9,077</div>  |
| leaf_verifier |  | 3 | PHANTOM | <div style='text-align: right'>177,171</div>  |
| leaf_verifier |  | 3 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier |  | 3 | SHINTW | <div style='text-align: right'>228,098</div>  |
| leaf_verifier |  | 3 | STOREW | <div style='text-align: right'>174,430</div>  |
| leaf_verifier |  | 3 | STOREW2 | <div style='text-align: right'>155,420</div>  |
| leaf_verifier |  | 3 | SUB | <div style='text-align: right'>51,033</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | ADD | <div style='text-align: right'>63,845,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | ADD | <div style='text-align: right'>377,762</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | ADD | <div style='text-align: right'>223,223</div>  |
| leaf_verifier | Boundary |  | 0 | ADD | <div style='text-align: right'>146,047</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4DIV | <div style='text-align: right'>439,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4DIV | <div style='text-align: right'>207,460</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4DIV | <div style='text-align: right'>122,590</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4DIV | <div style='text-align: right'>704</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | BBE4MUL | <div style='text-align: right'>847,280</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BBE4MUL | <div style='text-align: right'>484,440</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BBE4MUL | <div style='text-align: right'>286,260</div>  |
| leaf_verifier | Boundary |  | 0 | BBE4MUL | <div style='text-align: right'>139,304</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BEQ | <div style='text-align: right'>850,011</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | BNE | <div style='text-align: right'>27,577,092</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | BNE | <div style='text-align: right'>2,640</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | BNE | <div style='text-align: right'>1,560</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | COMP_POS2 | <div style='text-align: right'>1,374,912</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | COMP_POS2 | <div style='text-align: right'>812,448</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | COMP_POS2 | <div style='text-align: right'>531,216</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | COMP_POS2 | <div style='text-align: right'>19,087,614</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | DIV | <div style='text-align: right'>6,420</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4ADD | <div style='text-align: right'>932,400</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4ADD | <div style='text-align: right'>395,296</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4ADD | <div style='text-align: right'>233,584</div>  |
| leaf_verifier | Boundary |  | 0 | FE4ADD | <div style='text-align: right'>114,532</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | FE4SUB | <div style='text-align: right'>274,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FE4SUB | <div style='text-align: right'>234,036</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FE4SUB | <div style='text-align: right'>138,294</div>  |
| leaf_verifier | Boundary |  | 0 | FE4SUB | <div style='text-align: right'>26,092</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>258,808</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>152,932</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>15,816,192</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>1,376,160</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | JAL | <div style='text-align: right'>693</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | JAL | <div style='text-align: right'>819</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW | <div style='text-align: right'>12,059,781</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW | <div style='text-align: right'>512,105</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW | <div style='text-align: right'>243,919</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW | <div style='text-align: right'>21,681</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | LOADW2 | <div style='text-align: right'>26,186,782</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 0 | LOADW2 | <div style='text-align: right'>1,397</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | MUL | <div style='text-align: right'>8,089,740</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | MUL | <div style='text-align: right'>45,826</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | MUL | <div style='text-align: right'>27,118</div>  |
| leaf_verifier | Boundary |  | 0 | MUL | <div style='text-align: right'>32,824</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | PERM_POS2 | <div style='text-align: right'>1,040,039</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | PERM_POS2 | <div style='text-align: right'>617,305</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | PERM_POS2 | <div style='text-align: right'>412,913</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 0 | PERM_POS2 | <div style='text-align: right'>10,641,683</div>  |
| leaf_verifier | PhantomAir |  | 0 | PHANTOM | <div style='text-align: right'>2,219,310</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 0 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | SHINTW | <div style='text-align: right'>18,942,164</div>  |
| leaf_verifier | Boundary |  | 0 | SHINTW | <div style='text-align: right'>5,068,910</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>14,919,408</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW | <div style='text-align: right'>116,600</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW | <div style='text-align: right'>67,041</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>1,409,639</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | STOREW2 | <div style='text-align: right'>13,172,316</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | STOREW2 | <div style='text-align: right'>835,109</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | STOREW2 | <div style='text-align: right'>496,210</div>  |
| leaf_verifier | AccessAdapter<8> |  | 0 | STOREW2 | <div style='text-align: right'>233,410</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW2 | <div style='text-align: right'>1,402,357</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | SUB | <div style='text-align: right'>3,188,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 0 | SUB | <div style='text-align: right'>101,706</div>  |
| leaf_verifier | AccessAdapter<4> |  | 0 | SUB | <div style='text-align: right'>120,198</div>  |
| leaf_verifier | Boundary |  | 0 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | ADD | <div style='text-align: right'>58,516,800</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | ADD | <div style='text-align: right'>323,994</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | ADD | <div style='text-align: right'>191,451</div>  |
| leaf_verifier | Boundary |  | 1 | ADD | <div style='text-align: right'>136,895</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | BBE4MUL | <div style='text-align: right'>749,360</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BBE4MUL | <div style='text-align: right'>422,730</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BBE4MUL | <div style='text-align: right'>249,795</div>  |
| leaf_verifier | Boundary |  | 1 | BBE4MUL | <div style='text-align: right'>145,684</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BEQ | <div style='text-align: right'>840,627</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | BNE | <div style='text-align: right'>24,779,625</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | COMP_POS2 | <div style='text-align: right'>1,374,912</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | COMP_POS2 | <div style='text-align: right'>812,448</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | COMP_POS2 | <div style='text-align: right'>531,216</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 1 | COMP_POS2 | <div style='text-align: right'>18,876,312</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | FE4ADD | <div style='text-align: right'>790,480</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FE4ADD | <div style='text-align: right'>347,160</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FE4ADD | <div style='text-align: right'>205,140</div>  |
| leaf_verifier | Boundary |  | 1 | FE4ADD | <div style='text-align: right'>120,868</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FE4SUB | <div style='text-align: right'>229,438</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FE4SUB | <div style='text-align: right'>135,577</div>  |
| leaf_verifier | Boundary |  | 1 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 1 | JAL | <div style='text-align: right'>1,281,820</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 1 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | LOADW | <div style='text-align: right'>11,526,699</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW | <div style='text-align: right'>456,016</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW | <div style='text-align: right'>220,623</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | LOADW2 | <div style='text-align: right'>22,651,516</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 1 | LOADW2 | <div style='text-align: right'>1,397</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | MUL | <div style='text-align: right'>7,579,200</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | MUL | <div style='text-align: right'>42,900</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | MUL | <div style='text-align: right'>25,389</div>  |
| leaf_verifier | Boundary |  | 1 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | PERM_POS2 | <div style='text-align: right'>902,198</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | PERM_POS2 | <div style='text-align: right'>535,314</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | PERM_POS2 | <div style='text-align: right'>358,598</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 1 | PERM_POS2 | <div style='text-align: right'>9,066,980</div>  |
| leaf_verifier | PhantomAir |  | 1 | PHANTOM | <div style='text-align: right'>1,902,618</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 1 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | SHINTW | <div style='text-align: right'>17,817,698</div>  |
| leaf_verifier | Boundary |  | 1 | SHINTW | <div style='text-align: right'>4,765,695</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | STOREW | <div style='text-align: right'>13,802,322</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW | <div style='text-align: right'>260,183</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW | <div style='text-align: right'>152,022</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | STOREW | <div style='text-align: right'>62,832</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW | <div style='text-align: right'>687,753</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | STOREW2 | <div style='text-align: right'>11,619,072</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | STOREW2 | <div style='text-align: right'>686,180</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | STOREW2 | <div style='text-align: right'>407,667</div>  |
| leaf_verifier | AccessAdapter<8> |  | 1 | STOREW2 | <div style='text-align: right'>191,947</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW2 | <div style='text-align: right'>1,145,441</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | SUB | <div style='text-align: right'>2,890,140</div>  |
| leaf_verifier | AccessAdapter<2> |  | 1 | SUB | <div style='text-align: right'>84,942</div>  |
| leaf_verifier | AccessAdapter<4> |  | 1 | SUB | <div style='text-align: right'>100,386</div>  |
| leaf_verifier | Boundary |  | 1 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | ADD | <div style='text-align: right'>58,516,800</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | ADD | <div style='text-align: right'>325,534</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | ADD | <div style='text-align: right'>192,361</div>  |
| leaf_verifier | Boundary |  | 2 | ADD | <div style='text-align: right'>136,895</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4DIV | <div style='text-align: right'>378,080</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4DIV | <div style='text-align: right'>173,052</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4DIV | <div style='text-align: right'>102,258</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | BBE4MUL | <div style='text-align: right'>749,360</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BBE4MUL | <div style='text-align: right'>424,270</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BBE4MUL | <div style='text-align: right'>250,705</div>  |
| leaf_verifier | Boundary |  | 2 | BBE4MUL | <div style='text-align: right'>145,684</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BEQ | <div style='text-align: right'>840,627</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 2 | BNE | <div style='text-align: right'>24,779,625</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | COMP_POS2 | <div style='text-align: right'>1,374,912</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | COMP_POS2 | <div style='text-align: right'>812,448</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | COMP_POS2 | <div style='text-align: right'>531,216</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 2 | COMP_POS2 | <div style='text-align: right'>18,876,312</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | FE4ADD | <div style='text-align: right'>790,480</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FE4ADD | <div style='text-align: right'>347,160</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FE4ADD | <div style='text-align: right'>205,140</div>  |
| leaf_verifier | Boundary |  | 2 | FE4ADD | <div style='text-align: right'>120,868</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 2 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FE4SUB | <div style='text-align: right'>229,438</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FE4SUB | <div style='text-align: right'>135,577</div>  |
| leaf_verifier | Boundary |  | 2 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 2 | JAL | <div style='text-align: right'>1,313,390</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | JAL | <div style='text-align: right'>561</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | JAL | <div style='text-align: right'>663</div>  |
| leaf_verifier | Boundary |  | 2 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | LOADW | <div style='text-align: right'>11,526,699</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW | <div style='text-align: right'>456,016</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW | <div style='text-align: right'>220,623</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW | <div style='text-align: right'>21,395</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | LOADW2 | <div style='text-align: right'>22,651,516</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | LOADW2 | <div style='text-align: right'>114,422</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | LOADW2 | <div style='text-align: right'>67,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary |  | 2 | LOADW2 | <div style='text-align: right'>1,397</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | MUL | <div style='text-align: right'>7,579,200</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | MUL | <div style='text-align: right'>42,900</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | MUL | <div style='text-align: right'>25,389</div>  |
| leaf_verifier | Boundary |  | 2 | MUL | <div style='text-align: right'>32,032</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | PERM_POS2 | <div style='text-align: right'>902,198</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | PERM_POS2 | <div style='text-align: right'>535,314</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | PERM_POS2 | <div style='text-align: right'>358,598</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 2 | PERM_POS2 | <div style='text-align: right'>9,066,980</div>  |
| leaf_verifier | PhantomAir |  | 2 | PHANTOM | <div style='text-align: right'>1,902,618</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 2 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | Boundary |  | 2 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | SHINTW | <div style='text-align: right'>17,817,698</div>  |
| leaf_verifier | Boundary |  | 2 | SHINTW | <div style='text-align: right'>4,765,695</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | STOREW | <div style='text-align: right'>13,802,322</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW | <div style='text-align: right'>260,183</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW | <div style='text-align: right'>152,022</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | STOREW | <div style='text-align: right'>62,832</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW | <div style='text-align: right'>687,753</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 2 | STOREW2 | <div style='text-align: right'>11,619,072</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | STOREW2 | <div style='text-align: right'>686,180</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | STOREW2 | <div style='text-align: right'>407,667</div>  |
| leaf_verifier | AccessAdapter<8> |  | 2 | STOREW2 | <div style='text-align: right'>191,947</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW2 | <div style='text-align: right'>1,145,441</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 2 | SUB | <div style='text-align: right'>2,890,140</div>  |
| leaf_verifier | AccessAdapter<2> |  | 2 | SUB | <div style='text-align: right'>84,942</div>  |
| leaf_verifier | AccessAdapter<4> |  | 2 | SUB | <div style='text-align: right'>100,386</div>  |
| leaf_verifier | Boundary |  | 2 | SUB | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | ADD | <div style='text-align: right'>31,103,010</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | ADD | <div style='text-align: right'>170,104</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | ADD | <div style='text-align: right'>100,516</div>  |
| leaf_verifier | Boundary |  | 3 | ADD | <div style='text-align: right'>137,555</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4DIV | <div style='text-align: right'>209,600</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4DIV | <div style='text-align: right'>98,098</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4DIV | <div style='text-align: right'>57,967</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4DIV | <div style='text-align: right'>616</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | BBE4MUL | <div style='text-align: right'>405,720</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BBE4MUL | <div style='text-align: right'>265,012</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BBE4MUL | <div style='text-align: right'>156,598</div>  |
| leaf_verifier | Boundary |  | 3 | BBE4MUL | <div style='text-align: right'>145,904</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BEQ | <div style='text-align: right'>420,555</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 3 | BNE | <div style='text-align: right'>13,297,404</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | BNE | <div style='text-align: right'>1,298</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | BNE | <div style='text-align: right'>767</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | COMP_POS2 | <div style='text-align: right'>694,452</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | COMP_POS2 | <div style='text-align: right'>410,358</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | COMP_POS2 | <div style='text-align: right'>268,311</div>  |
| leaf_verifier | Boundary |  | 3 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 3 | COMP_POS2 | <div style='text-align: right'>9,467,783</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | DIV | <div style='text-align: right'>3,000</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | FE4ADD | <div style='text-align: right'>442,360</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FE4ADD | <div style='text-align: right'>215,798</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FE4ADD | <div style='text-align: right'>127,517</div>  |
| leaf_verifier | Boundary |  | 3 | FE4ADD | <div style='text-align: right'>119,768</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 3 | FE4SUB | <div style='text-align: right'>134,800</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FE4SUB | <div style='text-align: right'>122,012</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FE4SUB | <div style='text-align: right'>72,098</div>  |
| leaf_verifier | Boundary |  | 3 | FE4SUB | <div style='text-align: right'>25,520</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>122,716</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>72,514</div>  |
| leaf_verifier | FriReducedOpeningAir |  | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,542,528</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 3 | JAL | <div style='text-align: right'>682,100</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | JAL | <div style='text-align: right'>330</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | JAL | <div style='text-align: right'>390</div>  |
| leaf_verifier | Boundary |  | 3 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | LOADW | <div style='text-align: right'>6,042,047</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW | <div style='text-align: right'>249,436</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW | <div style='text-align: right'>119,613</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW | <div style='text-align: right'>20,893</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW | <div style='text-align: right'>21,329</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | LOADW2 | <div style='text-align: right'>12,453,094</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | LOADW2 | <div style='text-align: right'>57,200</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | LOADW2 | <div style='text-align: right'>33,800</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | LOADW2 | <div style='text-align: right'>493</div>  |
| leaf_verifier | Boundary |  | 3 | LOADW2 | <div style='text-align: right'>1,397</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | MUL | <div style='text-align: right'>3,943,050</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | MUL | <div style='text-align: right'>22,165</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | MUL | <div style='text-align: right'>13,117</div>  |
| leaf_verifier | Boundary |  | 3 | MUL | <div style='text-align: right'>32,208</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | PERM_POS2 | <div style='text-align: right'>495,044</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | PERM_POS2 | <div style='text-align: right'>293,891</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | PERM_POS2 | <div style='text-align: right'>195,738</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> |  | 3 | PERM_POS2 | <div style='text-align: right'>5,074,043</div>  |
| leaf_verifier | PhantomAir |  | 3 | PHANTOM | <div style='text-align: right'>1,063,026</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 3 | PUBLISH | <div style='text-align: right'>828</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | SHINTW | <div style='text-align: right'>9,352,018</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | SHINTW | <div style='text-align: right'>22</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | SHINTW | <div style='text-align: right'>26</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | SHINTW | <div style='text-align: right'>17</div>  |
| leaf_verifier | Boundary |  | 3 | SHINTW | <div style='text-align: right'>2,505,206</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | STOREW | <div style='text-align: right'>7,151,630</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW | <div style='text-align: right'>57,893</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW | <div style='text-align: right'>33,384</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW | <div style='text-align: right'>688,721</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 3 | STOREW2 | <div style='text-align: right'>6,372,220</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | STOREW2 | <div style='text-align: right'>401,786</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | STOREW2 | <div style='text-align: right'>238,784</div>  |
| leaf_verifier | AccessAdapter<8> |  | 3 | STOREW2 | <div style='text-align: right'>112,404</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW2 | <div style='text-align: right'>694,628</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 3 | SUB | <div style='text-align: right'>1,530,990</div>  |
| leaf_verifier | AccessAdapter<2> |  | 3 | SUB | <div style='text-align: right'>48,059</div>  |
| leaf_verifier | AccessAdapter<4> |  | 3 | SUB | <div style='text-align: right'>56,797</div>  |
| leaf_verifier | Boundary |  | 3 | SUB | <div style='text-align: right'>15,180</div>  |

| group | air_name | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf_verifier | ProgramAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 1 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 2 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 2 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 2 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 2 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | PhantomAir | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | ProgramAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 3 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 3 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 3 | 0 | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<2> | 3 | 0 | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<4> | 3 | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | AccessAdapterAir<8> | 3 | 0 | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | 0 | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | 0 | <div style='text-align: right'>18,350,080</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 3 | 0 | <div style='text-align: right'>1,966,080</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>32,768</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 3 | 0 | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 3 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 3 | 0 | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 3 | 0 | <div style='text-align: right'>68,157,440</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | PhantomAir | 3 | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | index | segment | stark_prove_excluding_trace_time_ms | total_cells | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | 0 | <div style='text-align: right'>21,356.0</div>  | <div style='text-align: right'>618,203,608</div>  | <div style='text-align: right'>1,554.0</div>  |
| leaf_verifier | 1 | 0 | <div style='text-align: right'>21,525.0</div>  | <div style='text-align: right'>615,320,024</div>  | <div style='text-align: right'>1,501.0</div>  |
| leaf_verifier | 2 | 0 | <div style='text-align: right'>21,362.0</div>  | <div style='text-align: right'>618,203,608</div>  | <div style='text-align: right'>1,486.0</div>  |
| leaf_verifier | 3 | 0 | <div style='text-align: right'>11,197.0</div>  | <div style='text-align: right'>311,462,360</div>  | <div style='text-align: right'>979.0</div>  |

| group | height | index | execute_time_ms | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | <div style='text-align: right'>7,508.0</div>  | <div style='text-align: right'>285,671,534</div>  | <div style='text-align: right'>7,262,999</div>  |
| internal_verifier_height_0 | 0 | 1 | <div style='text-align: right'>7,543.0</div>  | <div style='text-align: right'>281,631,237</div>  | <div style='text-align: right'>7,150,221</div>  |
| internal_verifier_height_1 | 1 | 2 | <div style='text-align: right'>7,765.0</div>  | <div style='text-align: right'>286,176,835</div>  | <div style='text-align: right'>7,276,849</div>  |

| group | chip_name | height | index | rows_used |
| --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramChip | 0 | 0 | <div style='text-align: right'>157,054</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 0 | <div style='text-align: right'>690,155</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 0 | <div style='text-align: right'>756,580</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 0 | <div style='text-align: right'>378,626</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 0 | <div style='text-align: right'>108,980</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>52,136</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>78,493</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>2,971,339</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 0 | <div style='text-align: right'>183,395</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 0 | <div style='text-align: right'>1,384,982</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 0 | <div style='text-align: right'>2,228,617</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | ProgramChip | 0 | 1 | <div style='text-align: right'>157,054</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 1 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 1 | <div style='text-align: right'>714,658</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 1 | <div style='text-align: right'>745,456</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 1 | <div style='text-align: right'>373,064</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 1 | <div style='text-align: right'>106,542</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | <div style='text-align: right'>50,959</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1 | <div style='text-align: right'>77,811</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 1 | <div style='text-align: right'>2,917,678</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 1 | <div style='text-align: right'>180,583</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 1 | <div style='text-align: right'>1,361,242</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | 1 | <div style='text-align: right'>2,199,466</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | <div style='text-align: right'>351,846</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | ProgramChip | 1 | 2 | <div style='text-align: right'>157,054</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | 2 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 | Boundary | 1 | 2 | <div style='text-align: right'>691,387</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | 1 | 2 | <div style='text-align: right'>757,960</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | 1 | 2 | <div style='text-align: right'>379,232</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | 1 | 2 | <div style='text-align: right'>109,072</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | <div style='text-align: right'>52,182</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | <div style='text-align: right'>218,064</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | 2 | <div style='text-align: right'>78,716</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | 2 | <div style='text-align: right'>2,982,246</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | 1 | 2 | <div style='text-align: right'>182,562</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | 2 | <div style='text-align: right'>1,386,495</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 1 | 2 | <div style='text-align: right'>2,230,527</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | <div style='text-align: right'>353,485</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | height | index | opcode | frequency |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 |  | 0 | 0 | ADD | <div style='text-align: right'>2,371,193</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>33,433</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BEQ | <div style='text-align: right'>37,525</div>  |
| internal_verifier_height_0 |  | 0 | 0 | BNE | <div style='text-align: right'>1,347,457</div>  |
| internal_verifier_height_0 |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_0 |  | 0 | 0 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4ADD | <div style='text-align: right'>24,936</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_0 |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 0 | JAL | <div style='text-align: right'>183,395</div>  |
| internal_verifier_height_0 |  | 0 | 0 | LOADW | <div style='text-align: right'>308,457</div>  |
| internal_verifier_height_0 |  | 0 | 0 | LOADW2 | <div style='text-align: right'>768,020</div>  |
| internal_verifier_height_0 |  | 0 | 0 | MUL | <div style='text-align: right'>418,726</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>17,444</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PHANTOM | <div style='text-align: right'>353,401</div>  |
| internal_verifier_height_0 |  | 0 | 0 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 |  | 0 | 0 | SHINTW | <div style='text-align: right'>464,520</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW | <div style='text-align: right'>344,202</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW2 | <div style='text-align: right'>343,418</div>  |
| internal_verifier_height_0 |  | 0 | 0 | SUB | <div style='text-align: right'>180,692</div>  |
| internal_verifier_height_0 |  | 0 | 1 | ADD | <div style='text-align: right'>2,328,667</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>12,802</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>33,006</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BEQ | <div style='text-align: right'>36,387</div>  |
| internal_verifier_height_0 |  | 0 | 1 | BNE | <div style='text-align: right'>1,324,855</div>  |
| internal_verifier_height_0 |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>33,600</div>  |
| internal_verifier_height_0 |  | 0 | 1 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4ADD | <div style='text-align: right'>24,849</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FE4SUB | <div style='text-align: right'>7,154</div>  |
| internal_verifier_height_0 |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 |  | 0 | 1 | JAL | <div style='text-align: right'>180,583</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW | <div style='text-align: right'>302,772</div>  |
| internal_verifier_height_0 |  | 0 | 1 | LOADW2 | <div style='text-align: right'>761,168</div>  |
| internal_verifier_height_0 |  | 0 | 1 | MUL | <div style='text-align: right'>410,493</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>17,359</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PHANTOM | <div style='text-align: right'>351,846</div>  |
| internal_verifier_height_0 |  | 0 | 1 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SHINTW | <div style='text-align: right'>454,305</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW | <div style='text-align: right'>340,799</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW2 | <div style='text-align: right'>340,422</div>  |
| internal_verifier_height_0 |  | 0 | 1 | SUB | <div style='text-align: right'>177,790</div>  |
| internal_verifier_height_1 |  | 1 | 2 | ADD | <div style='text-align: right'>2,377,144</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>12,844</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>33,624</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BEQ | <div style='text-align: right'>37,609</div>  |
| internal_verifier_height_1 |  | 1 | 2 | BNE | <div style='text-align: right'>1,348,886</div>  |
| internal_verifier_height_1 |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>34,776</div>  |
| internal_verifier_height_1 |  | 1 | 2 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4ADD | <div style='text-align: right'>24,968</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FE4SUB | <div style='text-align: right'>7,280</div>  |
| internal_verifier_height_1 |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_1 |  | 1 | 2 | JAL | <div style='text-align: right'>182,562</div>  |
| internal_verifier_height_1 |  | 1 | 2 | LOADW | <div style='text-align: right'>308,805</div>  |
| internal_verifier_height_1 |  | 1 | 2 | LOADW2 | <div style='text-align: right'>768,304</div>  |
| internal_verifier_height_1 |  | 1 | 2 | MUL | <div style='text-align: right'>422,086</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>17,406</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PHANTOM | <div style='text-align: right'>353,485</div>  |
| internal_verifier_height_1 |  | 1 | 2 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 |  | 1 | 2 | SHINTW | <div style='text-align: right'>465,308</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW | <div style='text-align: right'>344,492</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW2 | <div style='text-align: right'>343,618</div>  |
| internal_verifier_height_1 |  | 1 | 2 | SUB | <div style='text-align: right'>182,288</div>  |

| group | air_name | dsl_ir | height | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | ADD | <div style='text-align: right'>71,135,790</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | ADD | <div style='text-align: right'>478,632</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | ADD | <div style='text-align: right'>282,828</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | ADD | <div style='text-align: right'>166,298</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>251,944</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>148,876</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>1,337,320</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>553,960</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>327,340</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | BBE4MUL | <div style='text-align: right'>154,968</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 0 | BEQ | <div style='text-align: right'>863,075</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 0 | BNE | <div style='text-align: right'>30,991,511</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>1,437,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>849,576</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>555,492</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 0 | COMP_POS2 | <div style='text-align: right'>19,392,828</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>997,440</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>443,212</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FE4ADD | <div style='text-align: right'>261,898</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | FE4ADD | <div style='text-align: right'>107,316</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>252,450</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FE4SUB | <div style='text-align: right'>149,175</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | FE4SUB | <div style='text-align: right'>25,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir |  | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 0 | JAL | <div style='text-align: right'>1,833,950</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW | <div style='text-align: right'>12,646,737</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW | <div style='text-align: right'>499,191</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW | <div style='text-align: right'>228,254</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW | <div style='text-align: right'>22,407</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>31,488,820</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | MUL | <div style='text-align: right'>12,561,780</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | MUL | <div style='text-align: right'>56,826</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | MUL | <div style='text-align: right'>33,618</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>939,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>557,336</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>370,838</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 0 | PERM_POS2 | <div style='text-align: right'>9,751,196</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 0 | PHANTOM | <div style='text-align: right'>2,120,406</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 0 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | SHINTW | <div style='text-align: right'>19,045,320</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SHINTW | <div style='text-align: right'>5,095,057</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW | <div style='text-align: right'>14,112,282</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW | <div style='text-align: right'>256,036</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW | <div style='text-align: right'>148,304</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | STOREW | <div style='text-align: right'>54,264</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW | <div style='text-align: right'>659,153</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>14,080,138</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>889,702</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>527,917</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 0 | STOREW2 | <div style='text-align: right'>221,255</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW2 | <div style='text-align: right'>1,308,582</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 0 | SUB | <div style='text-align: right'>5,420,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 0 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 0 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | ADD | <div style='text-align: right'>69,860,010</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | ADD | <div style='text-align: right'>475,882</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | ADD | <div style='text-align: right'>281,203</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | ADD | <div style='text-align: right'>166,298</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>512,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>251,944</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>148,876</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>1,320,240</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>551,078</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>325,637</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | BBE4MUL | <div style='text-align: right'>154,968</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BEQ | <div style='text-align: right'>836,901</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 0 | 1 | BNE | <div style='text-align: right'>30,471,665</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>1,389,696</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>821,184</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>536,928</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 1 | COMP_POS2 | <div style='text-align: right'>18,782,400</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>993,960</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>441,320</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FE4ADD | <div style='text-align: right'>260,780</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | FE4ADD | <div style='text-align: right'>107,316</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>286,160</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>247,830</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FE4SUB | <div style='text-align: right'>146,445</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | FE4SUB | <div style='text-align: right'>25,080</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir |  | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 1 | JAL | <div style='text-align: right'>1,805,830</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | JAL | <div style='text-align: right'>671</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | JAL | <div style='text-align: right'>793</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW | <div style='text-align: right'>12,413,652</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW | <div style='text-align: right'>495,451</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW | <div style='text-align: right'>226,018</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW | <div style='text-align: right'>39,270</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW | <div style='text-align: right'>22,407</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>31,207,888</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>122,804</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>72,566</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,037</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | MUL | <div style='text-align: right'>12,314,790</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | MUL | <div style='text-align: right'>55,902</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | MUL | <div style='text-align: right'>33,072</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>935,748</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>555,126</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>368,679</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> |  | 0 | 1 | PERM_POS2 | <div style='text-align: right'>9,703,681</div>  |
| internal_verifier_height_0 | PhantomAir |  | 0 | 1 | PHANTOM | <div style='text-align: right'>2,111,076</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 0 | 1 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | SHINTW | <div style='text-align: right'>18,626,505</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SHINTW | <div style='text-align: right'>4,986,476</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW | <div style='text-align: right'>13,972,759</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW | <div style='text-align: right'>158,884</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW | <div style='text-align: right'>90,922</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | STOREW | <div style='text-align: right'>17,136</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW | <div style='text-align: right'>952,193</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>13,957,302</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>889,680</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>527,904</div>  |
| internal_verifier_height_0 | AccessAdapter<8> |  | 0 | 1 | STOREW2 | <div style='text-align: right'>220,541</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW2 | <div style='text-align: right'>1,393,656</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 0 | 1 | SUB | <div style='text-align: right'>5,333,700</div>  |
| internal_verifier_height_0 | AccessAdapter<2> |  | 0 | 1 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_0 | AccessAdapter<4> |  | 0 | 1 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | SUB | <div style='text-align: right'>15,268</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | ADD | <div style='text-align: right'>71,314,320</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | ADD | <div style='text-align: right'>479,270</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | ADD | <div style='text-align: right'>283,205</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | ADD | <div style='text-align: right'>167,354</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>513,760</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>251,658</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>148,707</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>1,344,960</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>557,084</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>329,186</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | BBE4MUL | <div style='text-align: right'>154,308</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | 2 | BEQ | <div style='text-align: right'>865,007</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | 1 | 2 | BNE | <div style='text-align: right'>31,024,378</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | BNE | <div style='text-align: right'>2,596</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | BNE | <div style='text-align: right'>1,534</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>1,441,440</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>851,760</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>556,920</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> |  | 1 | 2 | COMP_POS2 | <div style='text-align: right'>19,439,784</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>998,720</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>443,322</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FE4ADD | <div style='text-align: right'>261,963</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | FE4ADD | <div style='text-align: right'>106,172</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>291,200</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>252,296</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FE4SUB | <div style='text-align: right'>149,084</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | FE4SUB | <div style='text-align: right'>24,860</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,392</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>166,868</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir |  | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,956,096</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> |  | 1 | 2 | JAL | <div style='text-align: right'>1,825,620</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | JAL | <div style='text-align: right'>682</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | JAL | <div style='text-align: right'>806</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW | <div style='text-align: right'>12,661,005</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW | <div style='text-align: right'>500,214</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW | <div style='text-align: right'>228,995</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW | <div style='text-align: right'>22,759</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>31,500,464</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>125,598</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>74,217</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | LOADW2 | <div style='text-align: right'>1,760</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | MUL | <div style='text-align: right'>12,662,580</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | MUL | <div style='text-align: right'>57,090</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | MUL | <div style='text-align: right'>33,774</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | MUL | <div style='text-align: right'>33,924</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>940,588</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>557,440</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>370,192</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> |  | 1 | 2 | PERM_POS2 | <div style='text-align: right'>9,729,954</div>  |
| internal_verifier_height_1 | PhantomAir |  | 1 | 2 | PHANTOM | <div style='text-align: right'>2,120,910</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | 1 | 2 | PUBLISH | <div style='text-align: right'>1,196</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | SHINTW | <div style='text-align: right'>19,077,628</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | SHINTW | <div style='text-align: right'>5,103,725</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW | <div style='text-align: right'>14,124,172</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | STOREW | <div style='text-align: right'>256,905</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | STOREW | <div style='text-align: right'>148,681</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | STOREW | <div style='text-align: right'>54,264</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW | <div style='text-align: right'>660,033</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>14,088,338</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>890,802</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>528,021</div>  |
| internal_verifier_height_1 | AccessAdapter<8> |  | 1 | 2 | STOREW2 | <div style='text-align: right'>220,609</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW2 | <div style='text-align: right'>1,313,202</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | 1 | 2 | SUB | <div style='text-align: right'>5,468,640</div>  |
| internal_verifier_height_1 | AccessAdapter<2> |  | 1 | 2 | SUB | <div style='text-align: right'>117,238</div>  |
| internal_verifier_height_1 | AccessAdapter<4> |  | 1 | 2 | SUB | <div style='text-align: right'>138,554</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | SUB | <div style='text-align: right'>15,268</div>  |

| group | air_name | height | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramAir | 0 | 0 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | ProgramAir | 0 | 1 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | ProgramAir | 1 | 2 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_1 | VolatileBoundaryAir | 1 | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<2> | 1 | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<4> | 1 | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | AccessAdapterAir<8> | 1 | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| group | height | index | segment | stark_prove_excluding_trace_time_ms | total_cells | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | 0 | <div style='text-align: right'>25,866.0</div>  | <div style='text-align: right'>760,809,944</div>  | <div style='text-align: right'>1,712.0</div>  |
| internal_verifier_height_0 | 0 | 1 | 0 | <div style='text-align: right'>25,729.0</div>  | <div style='text-align: right'>760,809,944</div>  | <div style='text-align: right'>1,764.0</div>  |
| internal_verifier_height_1 | 1 | 2 | 0 | <div style='text-align: right'>26,009.0</div>  | <div style='text-align: right'>760,809,944</div>  | <div style='text-align: right'>1,736.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| root_verifier | ProgramChip | <div style='text-align: right'>157,327</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>2</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | <div style='text-align: right'>48</div>  |
| root_verifier | Boundary | <div style='text-align: right'>412,490</div>  |
| root_verifier | AccessAdapter<2> | <div style='text-align: right'>390,012</div>  |
| root_verifier | AccessAdapter<4> | <div style='text-align: right'>195,132</div>  |
| root_verifier | AccessAdapter<8> | <div style='text-align: right'>54,602</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>26,103</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>109,032</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>39,358</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>1,491,234</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | <div style='text-align: right'>88,514</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>693,273</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | <div style='text-align: right'>1,115,908</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>176,745</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| root_verifier |  | ADD | <div style='text-align: right'>1,188,671</div>  |
| root_verifier |  | BBE4DIV | <div style='text-align: right'>6,422</div>  |
| root_verifier |  | BBE4MUL | <div style='text-align: right'>16,812</div>  |
| root_verifier |  | BEQ | <div style='text-align: right'>18,805</div>  |
| root_verifier |  | BNE | <div style='text-align: right'>674,468</div>  |
| root_verifier |  | COMP_POS2 | <div style='text-align: right'>17,400</div>  |
| root_verifier |  | DIV | <div style='text-align: right'>364</div>  |
| root_verifier |  | FE4ADD | <div style='text-align: right'>12,484</div>  |
| root_verifier |  | FE4SUB | <div style='text-align: right'>3,640</div>  |
| root_verifier |  | FRI_REDUCED_OPENING | <div style='text-align: right'>5,292</div>  |
| root_verifier |  | JAL | <div style='text-align: right'>88,514</div>  |
| root_verifier |  | LOADW | <div style='text-align: right'>154,542</div>  |
| root_verifier |  | LOADW2 | <div style='text-align: right'>384,152</div>  |
| root_verifier |  | MUL | <div style='text-align: right'>211,055</div>  |
| root_verifier |  | PERM_POS2 | <div style='text-align: right'>8,703</div>  |
| root_verifier |  | PHANTOM | <div style='text-align: right'>176,745</div>  |
| root_verifier |  | PUBLISH | <div style='text-align: right'>48</div>  |
| root_verifier |  | SHINTW | <div style='text-align: right'>232,680</div>  |
| root_verifier |  | STOREW | <div style='text-align: right'>172,725</div>  |
| root_verifier |  | STOREW2 | <div style='text-align: right'>171,809</div>  |
| root_verifier |  | SUB | <div style='text-align: right'>91,144</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | <div style='text-align: right'>35,660,130</div>  |
| root_verifier | AccessAdapter<2> |  | ADD | <div style='text-align: right'>225,368</div>  |
| root_verifier | AccessAdapter<4> |  | ADD | <div style='text-align: right'>133,172</div>  |
| root_verifier | Boundary |  | ADD | <div style='text-align: right'>167,266</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | <div style='text-align: right'>256,880</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4DIV | <div style='text-align: right'>126,170</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4DIV | <div style='text-align: right'>74,555</div>  |
| root_verifier | Boundary |  | BBE4DIV | <div style='text-align: right'>1,584</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | <div style='text-align: right'>672,480</div>  |
| root_verifier | AccessAdapter<2> |  | BBE4MUL | <div style='text-align: right'>310,662</div>  |
| root_verifier | AccessAdapter<4> |  | BBE4MUL | <div style='text-align: right'>183,573</div>  |
| root_verifier | Boundary |  | BBE4MUL | <div style='text-align: right'>154,308</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | <div style='text-align: right'>432,515</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | <div style='text-align: right'>15,512,764</div>  |
| root_verifier | AccessAdapter<2> |  | BNE | <div style='text-align: right'>1,298</div>  |
| root_verifier | AccessAdapter<4> |  | BNE | <div style='text-align: right'>767</div>  |
| root_verifier | AccessAdapter<2> |  | COMP_POS2 | <div style='text-align: right'>722,172</div>  |
| root_verifier | AccessAdapter<4> |  | COMP_POS2 | <div style='text-align: right'>426,738</div>  |
| root_verifier | AccessAdapter<8> |  | COMP_POS2 | <div style='text-align: right'>279,021</div>  |
| root_verifier | Boundary |  | COMP_POS2 | <div style='text-align: right'>88</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | <div style='text-align: right'>9,726,600</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | <div style='text-align: right'>10,920</div>  |
| root_verifier | Boundary |  | DIV | <div style='text-align: right'>297</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | <div style='text-align: right'>499,360</div>  |
| root_verifier | AccessAdapter<2> |  | FE4ADD | <div style='text-align: right'>242,418</div>  |
| root_verifier | AccessAdapter<4> |  | FE4ADD | <div style='text-align: right'>143,247</div>  |
| root_verifier | Boundary |  | FE4ADD | <div style='text-align: right'>106,172</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | <div style='text-align: right'>145,600</div>  |
| root_verifier | AccessAdapter<2> |  | FE4SUB | <div style='text-align: right'>132,154</div>  |
| root_verifier | AccessAdapter<4> |  | FE4SUB | <div style='text-align: right'>78,091</div>  |
| root_verifier | Boundary |  | FE4SUB | <div style='text-align: right'>24,860</div>  |
| root_verifier | AccessAdapter<2> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>141,196</div>  |
| root_verifier | AccessAdapter<4> |  | FRI_REDUCED_OPENING | <div style='text-align: right'>83,434</div>  |
| root_verifier | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | <div style='text-align: right'>6,978,048</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>885,140</div>  |
| root_verifier | AccessAdapter<2> |  | JAL | <div style='text-align: right'>341</div>  |
| root_verifier | AccessAdapter<4> |  | JAL | <div style='text-align: right'>403</div>  |
| root_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW | <div style='text-align: right'>6,336,222</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW | <div style='text-align: right'>249,557</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW | <div style='text-align: right'>113,958</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>20,196</div>  |
| root_verifier | Boundary |  | LOADW | <div style='text-align: right'>22,407</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW2 | <div style='text-align: right'>15,750,232</div>  |
| root_verifier | AccessAdapter<2> |  | LOADW2 | <div style='text-align: right'>62,788</div>  |
| root_verifier | AccessAdapter<4> |  | LOADW2 | <div style='text-align: right'>37,102</div>  |
| root_verifier | AccessAdapter<8> |  | LOADW2 | <div style='text-align: right'>527</div>  |
| root_verifier | Boundary |  | LOADW2 | <div style='text-align: right'>1,815</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | <div style='text-align: right'>6,331,650</div>  |
| root_verifier | AccessAdapter<2> |  | MUL | <div style='text-align: right'>27,929</div>  |
| root_verifier | AccessAdapter<4> |  | MUL | <div style='text-align: right'>16,523</div>  |
| root_verifier | Boundary |  | MUL | <div style='text-align: right'>33,924</div>  |
| root_verifier | AccessAdapter<2> |  | PERM_POS2 | <div style='text-align: right'>470,294</div>  |
| root_verifier | AccessAdapter<4> |  | PERM_POS2 | <div style='text-align: right'>278,720</div>  |
| root_verifier | AccessAdapter<8> |  | PERM_POS2 | <div style='text-align: right'>185,096</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | <div style='text-align: right'>4,864,977</div>  |
| root_verifier | PhantomAir |  | PHANTOM | <div style='text-align: right'>1,060,470</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> |  | PUBLISH | <div style='text-align: right'>1,104</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SHINTW | <div style='text-align: right'>9,539,880</div>  |
| root_verifier | Boundary |  | SHINTW | <div style='text-align: right'>2,559,480</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>7,081,725</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW | <div style='text-align: right'>55,715</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW | <div style='text-align: right'>31,577</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>187</div>  |
| root_verifier | Boundary |  | STOREW | <div style='text-align: right'>660,517</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW2 | <div style='text-align: right'>7,044,169</div>  |
| root_verifier | AccessAdapter<2> |  | STOREW2 | <div style='text-align: right'>445,368</div>  |
| root_verifier | AccessAdapter<4> |  | STOREW2 | <div style='text-align: right'>263,991</div>  |
| root_verifier | AccessAdapter<8> |  | STOREW2 | <div style='text-align: right'>110,296</div>  |
| root_verifier | Boundary |  | STOREW2 | <div style='text-align: right'>789,393</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | <div style='text-align: right'>2,734,320</div>  |
| root_verifier | AccessAdapter<2> |  | SUB | <div style='text-align: right'>58,619</div>  |
| root_verifier | AccessAdapter<4> |  | SUB | <div style='text-align: right'>69,277</div>  |
| root_verifier | Boundary |  | SUB | <div style='text-align: right'>15,268</div>  |

| group | air_name | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| root_verifier | VolatileBoundaryAir | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | AccessAdapterAir<2> | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | ProgramAir | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | AccessAdapterAir<4> | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>18,350,080</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | AccessAdapterAir<8> | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | <div style='text-align: right'>2,496</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-fib_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-fib_e2e.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-fib_e2e.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-fib_e2e.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-fib_e2e.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-fib_e2e.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-fib_e2e.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-fib_e2e.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-leaf_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-root_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-root_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-root_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/b8954a2b14570974512ac34d109f7c5ccf4f257d/fib_e2e-90dd1eded61bfd85acbb5cc8ed114b68c42120bf1d8afa3fab4d7bb38b3ae4d4-root_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/b8954a2b14570974512ac34d109f7c5ccf4f257d

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12287197760)
