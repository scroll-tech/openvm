| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <div style='text-align: right'>2</div>  | <div style='text-align: right'>12,161,128</div>  | <div style='text-align: right'>12,000,219</div>  | <div style='text-align: right'>36,663.0</div>  |
| root_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>143,993,815</div>  | <div style='text-align: right'>3,633,127</div>  | <div style='text-align: right'>74,773.0</div>  |
| leaf_verifier | <div style='text-align: right'>2</div>  | <div style='text-align: right'>987,045,095</div>  | <div style='text-align: right'>24,119,888</div>  | <div style='text-align: right'>94,638.0</div>  |
| internal_verifier_height_0 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>572,069,604</div>  | <div style='text-align: right'>14,506,569</div>  | <div style='text-align: right'>56,083.0</div>  |
| internal_verifier_height_1 | <div style='text-align: right'>2</div>  | <div style='text-align: right'>286,727,063</div>  | <div style='text-align: right'>7,270,241</div>  | <div style='text-align: right'>28,131.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <div style='text-align: right'>8,270.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>12,161,128</div>  | <div style='text-align: right'>12,000,219</div>  | <div style='text-align: right'>36,663.0</div>  |
| root_verifier | <div style='text-align: right'>4,530.0</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>143,993,815</div>  | <div style='text-align: right'>3,633,127</div>  | <div style='text-align: right'>74,773.0</div>  |
| leaf_verifier |  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>987,045,095</div>  | <div style='text-align: right'>24,119,888</div>  | <div style='text-align: right'>94,638.0</div>  |
| internal_verifier_height_0 |  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>572,069,604</div>  | <div style='text-align: right'>14,506,569</div>  | <div style='text-align: right'>56,083.0</div>  |
| internal_verifier_height_1 |  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>286,727,063</div>  | <div style='text-align: right'>7,270,241</div>  | <div style='text-align: right'>28,131.0</div>  |

| group | chip_name | rows_used |
| --- | --- | --- |
| fibonacci_continuation_program | ProgramChip | <div style='text-align: right'>6,549</div>  |
| fibonacci_continuation_program | VmConnectorAir | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | Boundary | <div style='text-align: right'>54</div>  |
| fibonacci_continuation_program | Merkle | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | AccessAdapter<8> | <div style='text-align: right'>54</div>  |
| fibonacci_continuation_program | PhantomAir | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | <div style='text-align: right'>1,000,039</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | <div style='text-align: right'>333,341</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | <div style='text-align: right'>51</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | <div style='text-align: right'>222,227</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | <div style='text-align: right'>11</div>  |
| fibonacci_continuation_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | <div style='text-align: right'>111,118</div>  |
| fibonacci_continuation_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | <div style='text-align: right'>11</div>  |
| fibonacci_continuation_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>310</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | <div style='text-align: right'>131,072</div>  |
| root_verifier | ProgramChip | <div style='text-align: right'>155,298</div>  |
| root_verifier | VmConnectorAir | <div style='text-align: right'>2</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | <div style='text-align: right'>48</div>  |
| root_verifier | Boundary | <div style='text-align: right'>411,598</div>  |
| root_verifier | AccessAdapter<2> | <div style='text-align: right'>387,560</div>  |
| root_verifier | AccessAdapter<4> | <div style='text-align: right'>193,906</div>  |
| root_verifier | AccessAdapter<8> | <div style='text-align: right'>54,682</div>  |
| root_verifier | PhantomAir | <div style='text-align: right'>177,123</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | <div style='text-align: right'>1,115,599</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | <div style='text-align: right'>694,607</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | <div style='text-align: right'>87,585</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | <div style='text-align: right'>1,487,719</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | <div style='text-align: right'>39,011</div>  |
| root_verifier | FriReducedOpeningAir | <div style='text-align: right'>109,452</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>26,143</div>  |
| root_verifier | VariableRangeCheckerAir | <div style='text-align: right'>131,072</div>  |

| group | dsl_ir | opcode | frequency |
| --- | --- | --- | --- |
| fibonacci_continuation_program |  | ADD | <div style='text-align: right'>1,000,022</div>  |
| fibonacci_continuation_program |  | AND | <div style='text-align: right'>5</div>  |
| fibonacci_continuation_program |  | AUIPC | <div style='text-align: right'>11</div>  |
| fibonacci_continuation_program |  | BEQ | <div style='text-align: right'>111,114</div>  |
| fibonacci_continuation_program |  | BGEU | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | BLT | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program |  | BLTU | <div style='text-align: right'>7</div>  |
| fibonacci_continuation_program |  | BNE | <div style='text-align: right'>111,114</div>  |
| fibonacci_continuation_program |  | HINT_STOREW | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | JAL | <div style='text-align: right'>111,114</div>  |
| fibonacci_continuation_program |  | JALR | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program |  | LOADBU | <div style='text-align: right'>6</div>  |
| fibonacci_continuation_program |  | LOADW | <div style='text-align: right'>18</div>  |
| fibonacci_continuation_program |  | LUI | <div style='text-align: right'>10</div>  |
| fibonacci_continuation_program |  | OR | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program |  | PHANTOM | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | SLL | <div style='text-align: right'>3</div>  |
| fibonacci_continuation_program |  | SLTU | <div style='text-align: right'>333,341</div>  |
| fibonacci_continuation_program |  | SRL | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program |  | STOREB | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program |  | STOREW | <div style='text-align: right'>26</div>  |
| fibonacci_continuation_program |  | SUB | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program |  | XOR | <div style='text-align: right'>4</div>  |
| root_verifier |  | JAL | <div style='text-align: right'>1</div>  |
| root_verifier |  | STOREW | <div style='text-align: right'>2</div>  |
| root_verifier | AddE | FE4ADD | <div style='text-align: right'>12,146</div>  |
| root_verifier | AddEFFI | LOADW | <div style='text-align: right'>360</div>  |
| root_verifier | AddEFFI | STOREW | <div style='text-align: right'>1,080</div>  |
| root_verifier | AddEFI | ADD | <div style='text-align: right'>332</div>  |
| root_verifier | AddEI | ADD | <div style='text-align: right'>32,692</div>  |
| root_verifier | AddFI | ADD | <div style='text-align: right'>76,231</div>  |
| root_verifier | AddV | ADD | <div style='text-align: right'>17,239</div>  |
| root_verifier | AddVI | ADD | <div style='text-align: right'>358,871</div>  |
| root_verifier | Alloc | ADD | <div style='text-align: right'>58,302</div>  |
| root_verifier | Alloc | LOADW | <div style='text-align: right'>58,302</div>  |
| root_verifier | Alloc | MUL | <div style='text-align: right'>34,374</div>  |
| root_verifier | AssertEqE | BNE | <div style='text-align: right'>232</div>  |
| root_verifier | AssertEqEI | BNE | <div style='text-align: right'>4</div>  |
| root_verifier | AssertEqF | BNE | <div style='text-align: right'>9,467</div>  |
| root_verifier | AssertEqFI | BNE | <div style='text-align: right'>5</div>  |
| root_verifier | AssertEqV | BNE | <div style='text-align: right'>2,469</div>  |
| root_verifier | AssertEqVI | BNE | <div style='text-align: right'>221</div>  |
| root_verifier | AssertNeVI | BEQ | <div style='text-align: right'>1</div>  |
| root_verifier | CT-ExtractPublicValues | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-InitializePcsConst | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-ReadProofsFromInput | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-VerifyProofs | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>504</div>  |
| root_verifier | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>7,644</div>  |
| root_verifier | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>2,520</div>  |
| root_verifier | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>1,848</div>  |
| root_verifier | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>118,860</div>  |
| root_verifier | CT-single-reduced-opening-eval | PHANTOM | <div style='text-align: right'>10,584</div>  |
| root_verifier | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>2</div>  |
| root_verifier | CT-verify-batch | PHANTOM | <div style='text-align: right'>504</div>  |
| root_verifier | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>1,848</div>  |
| root_verifier | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>4,368</div>  |
| root_verifier | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>4,368</div>  |
| root_verifier | CT-verify-query | PHANTOM | <div style='text-align: right'>84</div>  |
| root_verifier | CastFV | ADD | <div style='text-align: right'>8</div>  |
| root_verifier | DivE | BBE4DIV | <div style='text-align: right'>6,248</div>  |
| root_verifier | DivEIN | BBE4DIV | <div style='text-align: right'>174</div>  |
| root_verifier | DivEIN | STOREW | <div style='text-align: right'>696</div>  |
| root_verifier | DivFIN | DIV | <div style='text-align: right'>364</div>  |
| root_verifier | For | ADD | <div style='text-align: right'>434,515</div>  |
| root_verifier | For | BNE | <div style='text-align: right'>480,043</div>  |
| root_verifier | For | JAL | <div style='text-align: right'>45,528</div>  |
| root_verifier | For | LOADW | <div style='text-align: right'>2,226</div>  |
| root_verifier | For | STOREW | <div style='text-align: right'>43,302</div>  |
| root_verifier | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>5,292</div>  |
| root_verifier | HintBitsF | PHANTOM | <div style='text-align: right'>43</div>  |
| root_verifier | HintInputVec | PHANTOM | <div style='text-align: right'>23,928</div>  |
| root_verifier | IfEq | BNE | <div style='text-align: right'>21,192</div>  |
| root_verifier | IfEqI | BNE | <div style='text-align: right'>162,211</div>  |
| root_verifier | IfEqI | JAL | <div style='text-align: right'>42,055</div>  |
| root_verifier | IfNe | BEQ | <div style='text-align: right'>16,489</div>  |
| root_verifier | IfNe | JAL | <div style='text-align: right'>1</div>  |
| root_verifier | IfNeI | BEQ | <div style='text-align: right'>2,273</div>  |
| root_verifier | ImmE | STOREW | <div style='text-align: right'>3,200</div>  |
| root_verifier | ImmF | STOREW | <div style='text-align: right'>35,481</div>  |
| root_verifier | ImmV | STOREW | <div style='text-align: right'>27,665</div>  |
| root_verifier | LoadE | LOADW | <div style='text-align: right'>21,652</div>  |
| root_verifier | LoadE | LOADW2 | <div style='text-align: right'>70,292</div>  |
| root_verifier | LoadF | LOADW | <div style='text-align: right'>27,613</div>  |
| root_verifier | LoadF | LOADW2 | <div style='text-align: right'>79,670</div>  |
| root_verifier | LoadV | LOADW | <div style='text-align: right'>28,301</div>  |
| root_verifier | LoadV | LOADW2 | <div style='text-align: right'>234,178</div>  |
| root_verifier | MulE | BBE4MUL | <div style='text-align: right'>15,771</div>  |
| root_verifier | MulEF | MUL | <div style='text-align: right'>5,088</div>  |
| root_verifier | MulEFI | MUL | <div style='text-align: right'>396</div>  |
| root_verifier | MulEI | BBE4MUL | <div style='text-align: right'>1,042</div>  |
| root_verifier | MulEI | STOREW | <div style='text-align: right'>4,168</div>  |
| root_verifier | MulF | MUL | <div style='text-align: right'>146,648</div>  |
| root_verifier | MulFI | MUL | <div style='text-align: right'>16</div>  |
| root_verifier | MulV | MUL | <div style='text-align: right'>1,333</div>  |
| root_verifier | MulVI | MUL | <div style='text-align: right'>21,227</div>  |
| root_verifier | NegE | MUL | <div style='text-align: right'>188</div>  |
| root_verifier | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>17,358</div>  |
| root_verifier | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>8,785</div>  |
| root_verifier | Publish | PUBLISH | <div style='text-align: right'>48</div>  |
| root_verifier | StoreE | STOREW | <div style='text-align: right'>25,744</div>  |
| root_verifier | StoreE | STOREW2 | <div style='text-align: right'>38,060</div>  |
| root_verifier | StoreF | STOREW | <div style='text-align: right'>27,918</div>  |
| root_verifier | StoreF | STOREW2 | <div style='text-align: right'>67,540</div>  |
| root_verifier | StoreHintWord | ADD | <div style='text-align: right'>207,291</div>  |
| root_verifier | StoreHintWord | SHINTW | <div style='text-align: right'>232,552</div>  |
| root_verifier | StoreV | STOREW | <div style='text-align: right'>3,294</div>  |
| root_verifier | StoreV | STOREW2 | <div style='text-align: right'>66,319</div>  |
| root_verifier | SubE | FE4SUB | <div style='text-align: right'>3,630</div>  |
| root_verifier | SubEF | LOADW | <div style='text-align: right'>15,984</div>  |
| root_verifier | SubEF | SUB | <div style='text-align: right'>5,328</div>  |
| root_verifier | SubEFI | ADD | <div style='text-align: right'>236</div>  |
| root_verifier | SubEI | ADD | <div style='text-align: right'>1,392</div>  |
| root_verifier | SubF | SUB | <div style='text-align: right'>8</div>  |
| root_verifier | SubV | SUB | <div style='text-align: right'>82,294</div>  |
| root_verifier | SubVI | SUB | <div style='text-align: right'>2,422</div>  |
| root_verifier | SubVIN | SUB | <div style='text-align: right'>924</div>  |

| group | air_name | dsl_ir | opcode | cells_used |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | <div style='text-align: right'>36,000,792</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | ADD | <div style='text-align: right'>68</div>  |
| fibonacci_continuation_program | Boundary |  | ADD | <div style='text-align: right'>160</div>  |
| fibonacci_continuation_program | Merkle |  | ADD | <div style='text-align: right'>3,712</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | <div style='text-align: right'>180</div>  |
| fibonacci_continuation_program | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | <div style='text-align: right'>231</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | AUIPC | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | AUIPC | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | AUIPC | <div style='text-align: right'>3,456</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | <div style='text-align: right'>2,888,964</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | <div style='text-align: right'>96</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLT | <div style='text-align: right'>32</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | <div style='text-align: right'>224</div>  |
| fibonacci_continuation_program | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | <div style='text-align: right'>2,888,964</div>  |
| fibonacci_continuation_program | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | <div style='text-align: right'>78</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | HINT_STOREW | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | HINT_STOREW | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | HINT_STOREW | <div style='text-align: right'>192</div>  |
| fibonacci_continuation_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | <div style='text-align: right'>2,000,052</div>  |
| fibonacci_continuation_program | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | <div style='text-align: right'>448</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADBU | <div style='text-align: right'>240</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | <div style='text-align: right'>720</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | LOADW | <div style='text-align: right'>68</div>  |
| fibonacci_continuation_program | Boundary |  | LOADW | <div style='text-align: right'>160</div>  |
| fibonacci_continuation_program | Merkle |  | LOADW | <div style='text-align: right'>2,304</div>  |
| fibonacci_continuation_program | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | <div style='text-align: right'>180</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | <div style='text-align: right'>144</div>  |
| fibonacci_continuation_program | PhantomAir |  | PHANTOM | <div style='text-align: right'>18</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | <div style='text-align: right'>159</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | <div style='text-align: right'>12,333,617</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | SLTU | <div style='text-align: right'>51</div>  |
| fibonacci_continuation_program | Boundary |  | SLTU | <div style='text-align: right'>120</div>  |
| fibonacci_continuation_program | Merkle |  | SLTU | <div style='text-align: right'>3,648</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SRL | <div style='text-align: right'>53</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREB | <div style='text-align: right'>40</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | STOREB | <div style='text-align: right'>17</div>  |
| fibonacci_continuation_program | Boundary |  | STOREB | <div style='text-align: right'>40</div>  |
| fibonacci_continuation_program | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | <div style='text-align: right'>1,040</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | STOREW | <div style='text-align: right'>255</div>  |
| fibonacci_continuation_program | Boundary |  | STOREW | <div style='text-align: right'>600</div>  |
| fibonacci_continuation_program | Merkle |  | STOREW | <div style='text-align: right'>2,048</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | <div style='text-align: right'>144</div>  |
| fibonacci_continuation_program | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | <div style='text-align: right'>144</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | BEQ | <div style='text-align: right'>34</div>  |
| fibonacci_continuation_program | Boundary |  | BEQ | <div style='text-align: right'>80</div>  |
| fibonacci_continuation_program | Merkle |  | BEQ | <div style='text-align: right'>192</div>  |
| fibonacci_continuation_program | AccessAdapter<8> |  | BNE | <div style='text-align: right'>17</div>  |
| fibonacci_continuation_program | Boundary |  | BNE | <div style='text-align: right'>40</div>  |
| fibonacci_continuation_program | Merkle |  | BNE | <div style='text-align: right'>64</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> |  | JAL | <div style='text-align: right'>10</div>  |
| root_verifier | Boundary |  | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | STOREW | <div style='text-align: right'>82</div>  |
| root_verifier | Boundary |  | STOREW | <div style='text-align: right'>22</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | FE4ADD | <div style='text-align: right'>485,840</div>  |
| root_verifier | AccessAdapter<2> | AddE | FE4ADD | <div style='text-align: right'>231,814</div>  |
| root_verifier | AccessAdapter<4> | AddE | FE4ADD | <div style='text-align: right'>136,981</div>  |
| root_verifier | Boundary | AddE | FE4ADD | <div style='text-align: right'>95,348</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | LOADW | <div style='text-align: right'>14,760</div>  |
| root_verifier | AccessAdapter<2> | AddEFFI | LOADW | <div style='text-align: right'>1,826</div>  |
| root_verifier | AccessAdapter<4> | AddEFFI | LOADW | <div style='text-align: right'>2,158</div>  |
| root_verifier | Boundary | AddEFFI | LOADW | <div style='text-align: right'>1,276</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | STOREW | <div style='text-align: right'>44,280</div>  |
| root_verifier | AccessAdapter<2> | AddEFFI | STOREW | <div style='text-align: right'>1,826</div>  |
| root_verifier | Boundary | AddEFFI | STOREW | <div style='text-align: right'>3,828</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | ADD | <div style='text-align: right'>9,960</div>  |
| root_verifier | AccessAdapter<2> | AddEFI | ADD | <div style='text-align: right'>1,672</div>  |
| root_verifier | AccessAdapter<4> | AddEFI | ADD | <div style='text-align: right'>988</div>  |
| root_verifier | Boundary | AddEFI | ADD | <div style='text-align: right'>1,496</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | ADD | <div style='text-align: right'>980,760</div>  |
| root_verifier | AccessAdapter<2> | AddEI | ADD | <div style='text-align: right'>192,654</div>  |
| root_verifier | AccessAdapter<4> | AddEI | ADD | <div style='text-align: right'>113,841</div>  |
| root_verifier | Boundary | AddEI | ADD | <div style='text-align: right'>160,952</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | ADD | <div style='text-align: right'>2,286,930</div>  |
| root_verifier | Boundary | AddFI | ADD | <div style='text-align: right'>1,309</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | ADD | <div style='text-align: right'>517,170</div>  |
| root_verifier | Boundary | AddV | ADD | <div style='text-align: right'>22</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | ADD | <div style='text-align: right'>10,766,130</div>  |
| root_verifier | Boundary | AddVI | ADD | <div style='text-align: right'>15,708</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | ADD | <div style='text-align: right'>1,749,060</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | LOADW | <div style='text-align: right'>2,390,382</div>  |
| root_verifier | Boundary | Alloc | LOADW | <div style='text-align: right'>1,144</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | MUL | <div style='text-align: right'>1,031,220</div>  |
| root_verifier | AccessAdapter<2> | Alloc | MUL | <div style='text-align: right'>33</div>  |
| root_verifier | AccessAdapter<4> | Alloc | MUL | <div style='text-align: right'>39</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | BNE | <div style='text-align: right'>5,336</div>  |
| root_verifier | AccessAdapter<2> | AssertEqE | BNE | <div style='text-align: right'>1,276</div>  |
| root_verifier | AccessAdapter<4> | AssertEqE | BNE | <div style='text-align: right'>754</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | BNE | <div style='text-align: right'>92</div>  |
| root_verifier | AccessAdapter<2> | AssertEqEI | BNE | <div style='text-align: right'>22</div>  |
| root_verifier | AccessAdapter<4> | AssertEqEI | BNE | <div style='text-align: right'>13</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | BNE | <div style='text-align: right'>217,741</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | BNE | <div style='text-align: right'>115</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | BNE | <div style='text-align: right'>56,787</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | BNE | <div style='text-align: right'>5,083</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | BEQ | <div style='text-align: right'>23</div>  |
| root_verifier | PhantomAir | CT-ExtractPublicValues | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-InitializePcsConst | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-ReadProofsFromInput | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-VerifyProofs | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-compute-reduced-opening | PHANTOM | <div style='text-align: right'>3,024</div>  |
| root_verifier | PhantomAir | CT-exp-reverse-bits-len | PHANTOM | <div style='text-align: right'>45,864</div>  |
| root_verifier | PhantomAir | CT-poseidon2-hash | PHANTOM | <div style='text-align: right'>15,120</div>  |
| root_verifier | PhantomAir | CT-poseidon2-hash-ext | PHANTOM | <div style='text-align: right'>11,088</div>  |
| root_verifier | PhantomAir | CT-poseidon2-hash-setup | PHANTOM | <div style='text-align: right'>713,160</div>  |
| root_verifier | PhantomAir | CT-single-reduced-opening-eval | PHANTOM | <div style='text-align: right'>63,504</div>  |
| root_verifier | PhantomAir | CT-stage-c-build-rounds | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-d-2-fri-fold | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-d-3-verify-challenges | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-d-verify-pcs | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-stage-e-verify-constraints | PHANTOM | <div style='text-align: right'>12</div>  |
| root_verifier | PhantomAir | CT-verify-batch | PHANTOM | <div style='text-align: right'>3,024</div>  |
| root_verifier | PhantomAir | CT-verify-batch-ext | PHANTOM | <div style='text-align: right'>11,088</div>  |
| root_verifier | PhantomAir | CT-verify-batch-reduce-fast | PHANTOM | <div style='text-align: right'>26,208</div>  |
| root_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | PHANTOM | <div style='text-align: right'>26,208</div>  |
| root_verifier | PhantomAir | CT-verify-query | PHANTOM | <div style='text-align: right'>504</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | ADD | <div style='text-align: right'>240</div>  |
| root_verifier | Boundary | CastFV | ADD | <div style='text-align: right'>88</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | BBE4DIV | <div style='text-align: right'>249,920</div>  |
| root_verifier | AccessAdapter<2> | DivE | BBE4DIV | <div style='text-align: right'>117,502</div>  |
| root_verifier | AccessAdapter<4> | DivE | BBE4DIV | <div style='text-align: right'>69,433</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | BBE4DIV | <div style='text-align: right'>6,960</div>  |
| root_verifier | AccessAdapter<2> | DivEIN | BBE4DIV | <div style='text-align: right'>8,866</div>  |
| root_verifier | AccessAdapter<4> | DivEIN | BBE4DIV | <div style='text-align: right'>5,239</div>  |
| root_verifier | Boundary | DivEIN | BBE4DIV | <div style='text-align: right'>2,112</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | STOREW | <div style='text-align: right'>28,536</div>  |
| root_verifier | AccessAdapter<2> | DivEIN | STOREW | <div style='text-align: right'>3,498</div>  |
| root_verifier | AccessAdapter<4> | DivEIN | STOREW | <div style='text-align: right'>1,872</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | DIV | <div style='text-align: right'>10,920</div>  |
| root_verifier | Boundary | DivFIN | DIV | <div style='text-align: right'>297</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | ADD | <div style='text-align: right'>13,035,450</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | BNE | <div style='text-align: right'>11,040,989</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | For | JAL | <div style='text-align: right'>455,280</div>  |
| root_verifier | AccessAdapter<2> | For | JAL | <div style='text-align: right'>330</div>  |
| root_verifier | AccessAdapter<4> | For | JAL | <div style='text-align: right'>390</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | LOADW | <div style='text-align: right'>91,266</div>  |
| root_verifier | Boundary | For | LOADW | <div style='text-align: right'>473</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | STOREW | <div style='text-align: right'>1,775,382</div>  |
| root_verifier | Boundary | For | STOREW | <div style='text-align: right'>803</div>  |
| root_verifier | AccessAdapter<2> | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>141,416</div>  |
| root_verifier | AccessAdapter<4> | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>83,564</div>  |
| root_verifier | FriReducedOpeningAir | FriReducedOpening | FRI_REDUCED_OPENING | <div style='text-align: right'>7,004,928</div>  |
| root_verifier | PhantomAir | HintBitsF | PHANTOM | <div style='text-align: right'>258</div>  |
| root_verifier | PhantomAir | HintInputVec | PHANTOM | <div style='text-align: right'>143,568</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | BNE | <div style='text-align: right'>487,416</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | BNE | <div style='text-align: right'>3,730,853</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | JAL | <div style='text-align: right'>420,550</div>  |
| root_verifier | AccessAdapter<2> | IfEqI | JAL | <div style='text-align: right'>11</div>  |
| root_verifier | AccessAdapter<4> | IfEqI | JAL | <div style='text-align: right'>13</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | BEQ | <div style='text-align: right'>379,247</div>  |
| root_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | JAL | <div style='text-align: right'>10</div>  |
| root_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | BEQ | <div style='text-align: right'>52,279</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | STOREW | <div style='text-align: right'>131,200</div>  |
| root_verifier | AccessAdapter<2> | ImmE | STOREW | <div style='text-align: right'>5,874</div>  |
| root_verifier | AccessAdapter<4> | ImmE | STOREW | <div style='text-align: right'>3,471</div>  |
| root_verifier | Boundary | ImmE | STOREW | <div style='text-align: right'>13,640</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | STOREW | <div style='text-align: right'>1,454,721</div>  |
| root_verifier | Boundary | ImmF | STOREW | <div style='text-align: right'>2,563</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | STOREW | <div style='text-align: right'>1,134,265</div>  |
| root_verifier | Boundary | ImmV | STOREW | <div style='text-align: right'>15,961</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW | <div style='text-align: right'>887,732</div>  |
| root_verifier | AccessAdapter<2> | LoadE | LOADW | <div style='text-align: right'>141,570</div>  |
| root_verifier | AccessAdapter<4> | LoadE | LOADW | <div style='text-align: right'>83,655</div>  |
| root_verifier | Boundary | LoadE | LOADW | <div style='text-align: right'>4,444</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | LOADW2 | <div style='text-align: right'>2,881,972</div>  |
| root_verifier | AccessAdapter<2> | LoadE | LOADW2 | <div style='text-align: right'>61,952</div>  |
| root_verifier | AccessAdapter<4> | LoadE | LOADW2 | <div style='text-align: right'>36,608</div>  |
| root_verifier | Boundary | LoadE | LOADW2 | <div style='text-align: right'>44</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW | <div style='text-align: right'>1,132,133</div>  |
| root_verifier | AccessAdapter<2> | LoadF | LOADW | <div style='text-align: right'>52,272</div>  |
| root_verifier | AccessAdapter<4> | LoadF | LOADW | <div style='text-align: right'>30,888</div>  |
| root_verifier | AccessAdapter<8> | LoadF | LOADW | <div style='text-align: right'>20,196</div>  |
| root_verifier | Boundary | LoadF | LOADW | <div style='text-align: right'>605</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | LOADW2 | <div style='text-align: right'>3,266,470</div>  |
| root_verifier | AccessAdapter<2> | LoadF | LOADW2 | <div style='text-align: right'>836</div>  |
| root_verifier | AccessAdapter<4> | LoadF | LOADW2 | <div style='text-align: right'>494</div>  |
| root_verifier | AccessAdapter<8> | LoadF | LOADW2 | <div style='text-align: right'>527</div>  |
| root_verifier | Boundary | LoadF | LOADW2 | <div style='text-align: right'>550</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW | <div style='text-align: right'>1,160,341</div>  |
| root_verifier | Boundary | LoadV | LOADW | <div style='text-align: right'>15,202</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | LOADW2 | <div style='text-align: right'>9,601,298</div>  |
| root_verifier | Boundary | LoadV | LOADW2 | <div style='text-align: right'>1,111</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | BBE4MUL | <div style='text-align: right'>630,840</div>  |
| root_verifier | AccessAdapter<2> | MulE | BBE4MUL | <div style='text-align: right'>262,570</div>  |
| root_verifier | AccessAdapter<4> | MulE | BBE4MUL | <div style='text-align: right'>155,155</div>  |
| root_verifier | Boundary | MulE | BBE4MUL | <div style='text-align: right'>148,368</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | MUL | <div style='text-align: right'>152,640</div>  |
| root_verifier | AccessAdapter<2> | MulEF | MUL | <div style='text-align: right'>23,980</div>  |
| root_verifier | AccessAdapter<4> | MulEF | MUL | <div style='text-align: right'>14,170</div>  |
| root_verifier | Boundary | MulEF | MUL | <div style='text-align: right'>4,224</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | MUL | <div style='text-align: right'>11,880</div>  |
| root_verifier | AccessAdapter<2> | MulEFI | MUL | <div style='text-align: right'>1,892</div>  |
| root_verifier | AccessAdapter<4> | MulEFI | MUL | <div style='text-align: right'>1,118</div>  |
| root_verifier | Boundary | MulEFI | MUL | <div style='text-align: right'>1,012</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | BBE4MUL | <div style='text-align: right'>41,680</div>  |
| root_verifier | AccessAdapter<2> | MulEI | BBE4MUL | <div style='text-align: right'>46,376</div>  |
| root_verifier | AccessAdapter<4> | MulEI | BBE4MUL | <div style='text-align: right'>27,404</div>  |
| root_verifier | Boundary | MulEI | BBE4MUL | <div style='text-align: right'>5,148</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | STOREW | <div style='text-align: right'>170,888</div>  |
| root_verifier | AccessAdapter<2> | MulEI | STOREW | <div style='text-align: right'>22,869</div>  |
| root_verifier | AccessAdapter<4> | MulEI | STOREW | <div style='text-align: right'>13,494</div>  |
| root_verifier | Boundary | MulEI | STOREW | <div style='text-align: right'>33</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | MUL | <div style='text-align: right'>4,399,440</div>  |
| root_verifier | Boundary | MulF | MUL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | MUL | <div style='text-align: right'>480</div>  |
| root_verifier | Boundary | MulFI | MUL | <div style='text-align: right'>11</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | MUL | <div style='text-align: right'>39,990</div>  |
| root_verifier | Boundary | MulV | MUL | <div style='text-align: right'>14,641</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | MUL | <div style='text-align: right'>636,810</div>  |
| root_verifier | Boundary | MulVI | MUL | <div style='text-align: right'>77</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | MUL | <div style='text-align: right'>5,640</div>  |
| root_verifier | AccessAdapter<2> | NegE | MUL | <div style='text-align: right'>1,012</div>  |
| root_verifier | AccessAdapter<4> | NegE | MUL | <div style='text-align: right'>598</div>  |
| root_verifier | Boundary | NegE | MUL | <div style='text-align: right'>792</div>  |
| root_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>720,324</div>  |
| root_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>425,646</div>  |
| root_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>278,307</div>  |
| root_verifier | Boundary | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>88</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | COMP_POS2 | <div style='text-align: right'>9,703,122</div>  |
| root_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>471,130</div>  |
| root_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>279,214</div>  |
| root_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>186,490</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | PERM_POS2 | <div style='text-align: right'>4,910,815</div>  |
| root_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | PUBLISH | <div style='text-align: right'>1,344</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW | <div style='text-align: right'>1,055,504</div>  |
| root_verifier | AccessAdapter<2> | StoreE | STOREW | <div style='text-align: right'>20,372</div>  |
| root_verifier | AccessAdapter<4> | StoreE | STOREW | <div style='text-align: right'>12,038</div>  |
| root_verifier | Boundary | StoreE | STOREW | <div style='text-align: right'>283,184</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | STOREW2 | <div style='text-align: right'>1,560,460</div>  |
| root_verifier | AccessAdapter<2> | StoreE | STOREW2 | <div style='text-align: right'>168,168</div>  |
| root_verifier | AccessAdapter<4> | StoreE | STOREW2 | <div style='text-align: right'>99,372</div>  |
| root_verifier | Boundary | StoreE | STOREW2 | <div style='text-align: right'>41,668</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW | <div style='text-align: right'>1,144,638</div>  |
| root_verifier | AccessAdapter<2> | StoreF | STOREW | <div style='text-align: right'>484</div>  |
| root_verifier | AccessAdapter<4> | StoreF | STOREW | <div style='text-align: right'>286</div>  |
| root_verifier | AccessAdapter<8> | StoreF | STOREW | <div style='text-align: right'>187</div>  |
| root_verifier | Boundary | StoreF | STOREW | <div style='text-align: right'>306,130</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | STOREW2 | <div style='text-align: right'>2,769,140</div>  |
| root_verifier | AccessAdapter<2> | StoreF | STOREW2 | <div style='text-align: right'>278,036</div>  |
| root_verifier | AccessAdapter<4> | StoreF | STOREW2 | <div style='text-align: right'>165,113</div>  |
| root_verifier | AccessAdapter<8> | StoreF | STOREW2 | <div style='text-align: right'>111,690</div>  |
| root_verifier | Boundary | StoreF | STOREW2 | <div style='text-align: right'>85,371</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | ADD | <div style='text-align: right'>6,218,730</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | SHINTW | <div style='text-align: right'>9,534,632</div>  |
| root_verifier | Boundary | StoreHintWord | SHINTW | <div style='text-align: right'>2,558,072</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW | <div style='text-align: right'>135,054</div>  |
| root_verifier | Boundary | StoreV | STOREW | <div style='text-align: right'>36,234</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | STOREW2 | <div style='text-align: right'>2,719,079</div>  |
| root_verifier | Boundary | StoreV | STOREW2 | <div style='text-align: right'>658,658</div>  |
| root_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | FE4SUB | <div style='text-align: right'>145,200</div>  |
| root_verifier | AccessAdapter<2> | SubE | FE4SUB | <div style='text-align: right'>131,582</div>  |
| root_verifier | AccessAdapter<4> | SubE | FE4SUB | <div style='text-align: right'>77,753</div>  |
| root_verifier | Boundary | SubE | FE4SUB | <div style='text-align: right'>25,212</div>  |
| root_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | LOADW | <div style='text-align: right'>655,344</div>  |
| root_verifier | AccessAdapter<2> | SubEF | LOADW | <div style='text-align: right'>58,608</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | SUB | <div style='text-align: right'>159,840</div>  |
| root_verifier | AccessAdapter<2> | SubEF | SUB | <div style='text-align: right'>58,608</div>  |
| root_verifier | AccessAdapter<4> | SubEF | SUB | <div style='text-align: right'>69,264</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | ADD | <div style='text-align: right'>7,080</div>  |
| root_verifier | AccessAdapter<2> | SubEFI | ADD | <div style='text-align: right'>880</div>  |
| root_verifier | AccessAdapter<4> | SubEFI | ADD | <div style='text-align: right'>520</div>  |
| root_verifier | Boundary | SubEFI | ADD | <div style='text-align: right'>132</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | ADD | <div style='text-align: right'>41,760</div>  |
| root_verifier | AccessAdapter<2> | SubEI | ADD | <div style='text-align: right'>11,528</div>  |
| root_verifier | AccessAdapter<4> | SubEI | ADD | <div style='text-align: right'>6,812</div>  |
| root_verifier | Boundary | SubEI | ADD | <div style='text-align: right'>4,224</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubF | SUB | <div style='text-align: right'>240</div>  |
| root_verifier | Boundary | SubF | SUB | <div style='text-align: right'>88</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | SUB | <div style='text-align: right'>2,468,820</div>  |
| root_verifier | Boundary | SubV | SUB | <div style='text-align: right'>44</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | SUB | <div style='text-align: right'>72,660</div>  |
| root_verifier | Boundary | SubVI | SUB | <div style='text-align: right'>15,147</div>  |
| root_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | SUB | <div style='text-align: right'>27,720</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fibonacci_continuation_program | ProgramAir | 0 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,624</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_continuation_program | PhantomAir | 0 | <div style='text-align: right'>72</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <div style='text-align: right'>420</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <div style='text-align: right'>7,168</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>64</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>1,408</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>321,024</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>512</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | ProgramAir | 1 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 1 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 1 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 1 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 1 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | PhantomAir | 1 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 1 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 1 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 1 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 1 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 1 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 1 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 1 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | ProgramAir | 2 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 2 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 2 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 2 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 2 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | PhantomAir | 2 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 2 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 2 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 2 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 2 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 2 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 2 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 2 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | ProgramAir | 3 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 3 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 3 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 3 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 3 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | PhantomAir | 3 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 3 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 3 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 3 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 3 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 3 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 3 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 3 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | ProgramAir | 4 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 4 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 4 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 4 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 4 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | PhantomAir | 4 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 4 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 4 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 4 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 4 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 4 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 4 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 4 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 4 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | ProgramAir | 5 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 5 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 5 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 5 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 5 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | PhantomAir | 5 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 5 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 5 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 5 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 5 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 5 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 5 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 5 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 5 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | ProgramAir | 6 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 6 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 6 | <div style='text-align: right'>512</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 6 | <div style='text-align: right'>6,656</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>128</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 6 | <div style='text-align: right'>656</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>16</div>  |
| fibonacci_continuation_program | PhantomAir | 6 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 6 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 6 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 6 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 6 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 6 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 6 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 6 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 6 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | ProgramAir | 7 | <div style='text-align: right'>147,456</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>8,192</div>  |
| fibonacci_continuation_program | VmConnectorAir | 7 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| fibonacci_continuation_program | PersistentBoundaryAir<8> | 7 | <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>32</div>  |
| fibonacci_continuation_program | MemoryMerkleAir<8> | 7 | <div style='text-align: right'>13,312</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | AccessAdapterAir<8> | 7 | <div style='text-align: right'>1,312</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>32</div>  |
| fibonacci_continuation_program | PhantomAir | 7 | <div style='text-align: right'>18</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 7 | <div style='text-align: right'>30,408,704</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>262,144</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 7 | <div style='text-align: right'>10,092,544</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>131,072</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 7 | <div style='text-align: right'>896</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <div style='text-align: right'>8</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 7 | <div style='text-align: right'>4,849,664</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 7 | <div style='text-align: right'>2,031,616</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>32,768</div>  |
| fibonacci_continuation_program | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 7 | <div style='text-align: right'>64</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>1</div>  |
| fibonacci_continuation_program | Poseidon2VmAir<BabyBearParameters> | 7 | <div style='text-align: right'>160,512</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <div style='text-align: right'>256</div>  |
| fibonacci_continuation_program | BitwiseOperationLookupAir<8> | 7 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| fibonacci_continuation_program | RangeTupleCheckerAir<2> | 7 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| fibonacci_continuation_program | VariableRangeCheckerAir | 7 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| root_verifier | ProgramAir | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | VmConnectorAir | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | <div style='text-align: right'>2,816</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| root_verifier | VolatileBoundaryAir | 0 | <div style='text-align: right'>9,961,472</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | AccessAdapterAir<2> | 0 | <div style='text-align: right'>14,155,776</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| root_verifier | AccessAdapterAir<4> | 0 | <div style='text-align: right'>7,602,176</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | AccessAdapterAir<8> | 0 | <div style='text-align: right'>2,162,688</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | PhantomAir | 0 | <div style='text-align: right'>3,670,016</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| root_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>53,477,376</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| root_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>104,857,600</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| root_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| root_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>18,350,080</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>131,072</div>  |
| root_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>19,496,960</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>32,768</div>  |
| root_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- |
| fibonacci_continuation_program | 0 | <div style='text-align: right'>248.0</div>  | <div style='text-align: right'>5,142.0</div>  | <div style='text-align: right'>196,581,332</div>  |
| fibonacci_continuation_program | 1 | <div style='text-align: right'>296.0</div>  | <div style='text-align: right'>4,556.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuation_program | 2 | <div style='text-align: right'>256.0</div>  | <div style='text-align: right'>4,543.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuation_program | 3 | <div style='text-align: right'>279.0</div>  | <div style='text-align: right'>4,592.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuation_program | 4 | <div style='text-align: right'>260.0</div>  | <div style='text-align: right'>4,586.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuation_program | 5 | <div style='text-align: right'>282.0</div>  | <div style='text-align: right'>4,624.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuation_program | 6 | <div style='text-align: right'>259.0</div>  | <div style='text-align: right'>4,647.0</div>  | <div style='text-align: right'>196,399,554</div>  |
| fibonacci_continuation_program | 7 | <div style='text-align: right'>82.0</div>  | <div style='text-align: right'>2,011.0</div>  | <div style='text-align: right'>54,260,754</div>  |
| root_verifier | 0 | <div style='text-align: right'>837.0</div>  | <div style='text-align: right'>73,936.0</div>  | <div style='text-align: right'>382,765,848</div>  |

| group | index | execute_time_ms | fri.log_blowup | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | <div style='text-align: right'>7,325.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,756,329</div>  | <div style='text-align: right'>6,434,148</div>  |
| leaf_verifier | 1 | <div style='text-align: right'>6,733.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>240,043,796</div>  | <div style='text-align: right'>5,862,418</div>  |
| leaf_verifier | 2 | <div style='text-align: right'>6,699.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>239,989,726</div>  | <div style='text-align: right'>5,857,894</div>  |
| leaf_verifier | 3 | <div style='text-align: right'>6,848.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>244,255,244</div>  | <div style='text-align: right'>5,965,428</div>  |

| group | chip_name | index | rows_used |
| --- | --- | --- | --- |
| leaf_verifier | ProgramChip | 0 | <div style='text-align: right'>107,256</div>  |
| leaf_verifier | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 0 | <div style='text-align: right'>759,257</div>  |
| leaf_verifier | AccessAdapter<2> | 0 | <div style='text-align: right'>720,644</div>  |
| leaf_verifier | AccessAdapter<4> | 0 | <div style='text-align: right'>360,742</div>  |
| leaf_verifier | AccessAdapter<8> | 0 | <div style='text-align: right'>110,562</div>  |
| leaf_verifier | PhantomAir | 0 | <div style='text-align: right'>368,373</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | <div style='text-align: right'>2,068,117</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <div style='text-align: right'>1,234,630</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 0 | <div style='text-align: right'>138,950</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>2,500,086</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <div style='text-align: right'>61,873</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | <div style='text-align: right'>247,128</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>52,927</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramChip | 1 | <div style='text-align: right'>107,256</div>  |
| leaf_verifier | VmConnectorAir | 1 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 1 | <div style='text-align: right'>723,719</div>  |
| leaf_verifier | AccessAdapter<2> | 1 | <div style='text-align: right'>663,972</div>  |
| leaf_verifier | AccessAdapter<4> | 1 | <div style='text-align: right'>332,322</div>  |
| leaf_verifier | AccessAdapter<8> | 1 | <div style='text-align: right'>104,676</div>  |
| leaf_verifier | PhantomAir | 1 | <div style='text-align: right'>317,103</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 1 | <div style='text-align: right'>1,888,097</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | <div style='text-align: right'>1,117,030</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 1 | <div style='text-align: right'>131,597</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | <div style='text-align: right'>2,296,687</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | <div style='text-align: right'>54,240</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | <div style='text-align: right'>49,984</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramChip | 2 | <div style='text-align: right'>107,256</div>  |
| leaf_verifier | VmConnectorAir | 2 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 2 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 2 | <div style='text-align: right'>723,719</div>  |
| leaf_verifier | AccessAdapter<2> | 2 | <div style='text-align: right'>663,584</div>  |
| leaf_verifier | AccessAdapter<4> | 2 | <div style='text-align: right'>332,128</div>  |
| leaf_verifier | AccessAdapter<8> | 2 | <div style='text-align: right'>104,676</div>  |
| leaf_verifier | PhantomAir | 2 | <div style='text-align: right'>317,103</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 2 | <div style='text-align: right'>1,888,097</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 2 | <div style='text-align: right'>1,117,030</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 2 | <div style='text-align: right'>127,175</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 2 | <div style='text-align: right'>2,296,585</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 2 | <div style='text-align: right'>54,240</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | <div style='text-align: right'>204,792</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | <div style='text-align: right'>49,984</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramChip | 3 | <div style='text-align: right'>107,256</div>  |
| leaf_verifier | VmConnectorAir | 3 | <div style='text-align: right'>2</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 3 | <div style='text-align: right'>36</div>  |
| leaf_verifier | Boundary | 3 | <div style='text-align: right'>724,306</div>  |
| leaf_verifier | AccessAdapter<2> | 3 | <div style='text-align: right'>674,584</div>  |
| leaf_verifier | AccessAdapter<4> | 3 | <div style='text-align: right'>337,670</div>  |
| leaf_verifier | AccessAdapter<8> | 3 | <div style='text-align: right'>104,742</div>  |
| leaf_verifier | PhantomAir | 3 | <div style='text-align: right'>334,247</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 3 | <div style='text-align: right'>1,927,191</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 3 | <div style='text-align: right'>1,138,360</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | 3 | <div style='text-align: right'>127,585</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 3 | <div style='text-align: right'>2,323,654</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 3 | <div style='text-align: right'>56,254</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | <div style='text-align: right'>220,248</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | <div style='text-align: right'>49,953</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | <div style='text-align: right'>131,072</div>  |

| group | dsl_ir | index | opcode | frequency |
| --- | --- | --- | --- | --- |
| leaf_verifier |  | 0 | JAL | <div style='text-align: right'>1</div>  |
| leaf_verifier |  | 0 | STOREW | <div style='text-align: right'>2</div>  |
| leaf_verifier | AddE | 0 | FE4ADD | <div style='text-align: right'>22,817</div>  |
| leaf_verifier | AddEFFI | 0 | LOADW | <div style='text-align: right'>340</div>  |
| leaf_verifier | AddEFFI | 0 | STOREW | <div style='text-align: right'>1,020</div>  |
| leaf_verifier | AddEFI | 0 | ADD | <div style='text-align: right'>484</div>  |
| leaf_verifier | AddEI | 0 | ADD | <div style='text-align: right'>55,612</div>  |
| leaf_verifier | AddFI | 0 | ADD | <div style='text-align: right'>80,971</div>  |
| leaf_verifier | AddV | 0 | ADD | <div style='text-align: right'>28,494</div>  |
| leaf_verifier | AddVI | 0 | ADD | <div style='text-align: right'>647,247</div>  |
| leaf_verifier | Alloc | 0 | ADD | <div style='text-align: right'>110,013</div>  |
| leaf_verifier | Alloc | 0 | LOADW | <div style='text-align: right'>110,013</div>  |
| leaf_verifier | Alloc | 0 | MUL | <div style='text-align: right'>65,578</div>  |
| leaf_verifier | AssertEqE | 0 | BNE | <div style='text-align: right'>472</div>  |
| leaf_verifier | AssertEqEI | 0 | BNE | <div style='text-align: right'>8</div>  |
| leaf_verifier | AssertEqF | 0 | BNE | <div style='text-align: right'>18,919</div>  |
| leaf_verifier | AssertEqFI | 0 | BNE | <div style='text-align: right'>1</div>  |
| leaf_verifier | AssertEqV | 0 | BNE | <div style='text-align: right'>4,746</div>  |
| leaf_verifier | AssertEqVI | 0 | BNE | <div style='text-align: right'>408</div>  |
| leaf_verifier | AssertNeVI | 0 | BEQ | <div style='text-align: right'>1</div>  |
| leaf_verifier | CT-ExtractPublicValuesCommit | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-InitializePcsConst | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-ReadProofsFromInput | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-VerifyProofs | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-compute-reduced-opening | 0 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-exp-reverse-bits-len | 0 | PHANTOM | <div style='text-align: right'>11,760</div>  |
| leaf_verifier | CT-poseidon2-hash | 0 | PHANTOM | <div style='text-align: right'>6,636</div>  |
| leaf_verifier | CT-poseidon2-hash-ext | 0 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-poseidon2-hash-setup | 0 | PHANTOM | <div style='text-align: right'>257,544</div>  |
| leaf_verifier | CT-single-reduced-opening-eval | 0 | PHANTOM | <div style='text-align: right'>18,312</div>  |
| leaf_verifier | CT-stage-c-build-rounds | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-2-fri-fold | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-3-verify-challenges | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-verify-pcs | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-e-verify-constraints | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-verify-batch | 0 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-verify-batch-ext | 0 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast | 0 | PHANTOM | <div style='text-align: right'>9,996</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast-setup | 0 | PHANTOM | <div style='text-align: right'>9,996</div>  |
| leaf_verifier | CT-verify-query | 0 | PHANTOM | <div style='text-align: right'>168</div>  |
| leaf_verifier | CastFV | 0 | ADD | <div style='text-align: right'>1</div>  |
| leaf_verifier | DivE | 0 | BBE4DIV | <div style='text-align: right'>10,904</div>  |
| leaf_verifier | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>90</div>  |
| leaf_verifier | DivEIN | 0 | STOREW | <div style='text-align: right'>360</div>  |
| leaf_verifier | DivFIN | 0 | DIV | <div style='text-align: right'>214</div>  |
| leaf_verifier | For | 0 | ADD | <div style='text-align: right'>785,937</div>  |
| leaf_verifier | For | 0 | BNE | <div style='text-align: right'>869,212</div>  |
| leaf_verifier | For | 0 | JAL | <div style='text-align: right'>83,275</div>  |
| leaf_verifier | For | 0 | LOADW | <div style='text-align: right'>5,082</div>  |
| leaf_verifier | For | 0 | STOREW | <div style='text-align: right'>78,193</div>  |
| leaf_verifier | FriReducedOpening | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>9,156</div>  |
| leaf_verifier | HintBitsF | 0 | PHANTOM | <div style='text-align: right'>86</div>  |
| leaf_verifier | HintInputVec | 0 | PHANTOM | <div style='text-align: right'>44,435</div>  |
| leaf_verifier | IfEq | 0 | BNE | <div style='text-align: right'>44,088</div>  |
| leaf_verifier | IfEqI | 0 | BNE | <div style='text-align: right'>260,071</div>  |
| leaf_verifier | IfEqI | 0 | JAL | <div style='text-align: right'>55,670</div>  |
| leaf_verifier | IfNe | 0 | BEQ | <div style='text-align: right'>31,534</div>  |
| leaf_verifier | IfNe | 0 | JAL | <div style='text-align: right'>4</div>  |
| leaf_verifier | IfNeI | 0 | BEQ | <div style='text-align: right'>5,170</div>  |
| leaf_verifier | ImmE | 0 | STOREW | <div style='text-align: right'>6,280</div>  |
| leaf_verifier | ImmF | 0 | STOREW | <div style='text-align: right'>78,624</div>  |
| leaf_verifier | ImmV | 0 | STOREW | <div style='text-align: right'>57,815</div>  |
| leaf_verifier | LoadE | 0 | LOADW | <div style='text-align: right'>43,992</div>  |
| leaf_verifier | LoadE | 0 | LOADW2 | <div style='text-align: right'>117,408</div>  |
| leaf_verifier | LoadF | 0 | LOADW | <div style='text-align: right'>53,298</div>  |
| leaf_verifier | LoadF | 0 | LOADW2 | <div style='text-align: right'>160,620</div>  |
| leaf_verifier | LoadV | 0 | LOADW | <div style='text-align: right'>53,084</div>  |
| leaf_verifier | LoadV | 0 | LOADW2 | <div style='text-align: right'>358,874</div>  |
| leaf_verifier | MulE | 0 | BBE4MUL | <div style='text-align: right'>18,542</div>  |
| leaf_verifier | MulEF | 0 | MUL | <div style='text-align: right'>7,440</div>  |
| leaf_verifier | MulEFI | 0 | MUL | <div style='text-align: right'>740</div>  |
| leaf_verifier | MulEI | 0 | BBE4MUL | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | MulEI | 0 | STOREW | <div style='text-align: right'>10,664</div>  |
| leaf_verifier | MulF | 0 | MUL | <div style='text-align: right'>152,176</div>  |
| leaf_verifier | MulFI | 0 | MUL | <div style='text-align: right'>34</div>  |
| leaf_verifier | MulV | 0 | MUL | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | MulVI | 0 | MUL | <div style='text-align: right'>39,062</div>  |
| leaf_verifier | NegE | 0 | MUL | <div style='text-align: right'>332</div>  |
| leaf_verifier | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>33,894</div>  |
| leaf_verifier | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>19,033</div>  |
| leaf_verifier | Publish | 0 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier | StoreE | 0 | STOREW | <div style='text-align: right'>48,824</div>  |
| leaf_verifier | StoreE | 0 | STOREW2 | <div style='text-align: right'>60,648</div>  |
| leaf_verifier | StoreF | 0 | STOREW | <div style='text-align: right'>66,696</div>  |
| leaf_verifier | StoreF | 0 | STOREW2 | <div style='text-align: right'>143,400</div>  |
| leaf_verifier | StoreHintWord | 0 | ADD | <div style='text-align: right'>414,903</div>  |
| leaf_verifier | StoreHintWord | 0 | SHINTW | <div style='text-align: right'>462,004</div>  |
| leaf_verifier | StoreV | 0 | STOREW | <div style='text-align: right'>5,890</div>  |
| leaf_verifier | StoreV | 0 | STOREW2 | <div style='text-align: right'>117,194</div>  |
| leaf_verifier | SubE | 0 | FE4SUB | <div style='text-align: right'>6,854</div>  |
| leaf_verifier | SubEF | 0 | LOADW | <div style='text-align: right'>27,792</div>  |
| leaf_verifier | SubEF | 0 | SUB | <div style='text-align: right'>9,264</div>  |
| leaf_verifier | SubEFI | 0 | ADD | <div style='text-align: right'>516</div>  |
| leaf_verifier | SubEI | 0 | ADD | <div style='text-align: right'>720</div>  |
| leaf_verifier | SubV | 0 | SUB | <div style='text-align: right'>91,342</div>  |
| leaf_verifier | SubVI | 0 | SUB | <div style='text-align: right'>4,660</div>  |
| leaf_verifier | SubVIN | 0 | SUB | <div style='text-align: right'>1,680</div>  |
| leaf_verifier |  | 1 | JAL | <div style='text-align: right'>1</div>  |
| leaf_verifier |  | 1 | STOREW | <div style='text-align: right'>2</div>  |
| leaf_verifier | AddE | 1 | FE4ADD | <div style='text-align: right'>19,436</div>  |
| leaf_verifier | AddEFFI | 1 | LOADW | <div style='text-align: right'>328</div>  |
| leaf_verifier | AddEFFI | 1 | STOREW | <div style='text-align: right'>984</div>  |
| leaf_verifier | AddEFI | 1 | ADD | <div style='text-align: right'>344</div>  |
| leaf_verifier | AddEI | 1 | ADD | <div style='text-align: right'>50,904</div>  |
| leaf_verifier | AddFI | 1 | ADD | <div style='text-align: right'>72,934</div>  |
| leaf_verifier | AddV | 1 | ADD | <div style='text-align: right'>27,384</div>  |
| leaf_verifier | AddVI | 1 | ADD | <div style='text-align: right'>587,739</div>  |
| leaf_verifier | Alloc | 1 | ADD | <div style='text-align: right'>107,553</div>  |
| leaf_verifier | Alloc | 1 | LOADW | <div style='text-align: right'>107,553</div>  |
| leaf_verifier | Alloc | 1 | MUL | <div style='text-align: right'>64,240</div>  |
| leaf_verifier | AssertEqE | 1 | BNE | <div style='text-align: right'>448</div>  |
| leaf_verifier | AssertEqEI | 1 | BNE | <div style='text-align: right'>8</div>  |
| leaf_verifier | AssertEqF | 1 | BNE | <div style='text-align: right'>18,919</div>  |
| leaf_verifier | AssertEqFI | 1 | BNE | <div style='text-align: right'>1</div>  |
| leaf_verifier | AssertEqV | 1 | BNE | <div style='text-align: right'>4,680</div>  |
| leaf_verifier | AssertEqVI | 1 | BNE | <div style='text-align: right'>342</div>  |
| leaf_verifier | AssertNeVI | 1 | BEQ | <div style='text-align: right'>1</div>  |
| leaf_verifier | CT-ExtractPublicValuesCommit | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-InitializePcsConst | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-ReadProofsFromInput | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-VerifyProofs | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-compute-reduced-opening | 1 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-exp-reverse-bits-len | 1 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-poseidon2-hash | 1 | PHANTOM | <div style='text-align: right'>6,384</div>  |
| leaf_verifier | CT-poseidon2-hash-ext | 1 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-poseidon2-hash-setup | 1 | PHANTOM | <div style='text-align: right'>213,192</div>  |
| leaf_verifier | CT-single-reduced-opening-eval | 1 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| leaf_verifier | CT-stage-c-build-rounds | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-2-fri-fold | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-3-verify-challenges | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-verify-pcs | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-e-verify-constraints | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-verify-batch | 1 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-verify-batch-ext | 1 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast | 1 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast-setup | 1 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-verify-query | 1 | PHANTOM | <div style='text-align: right'>168</div>  |
| leaf_verifier | CastFV | 1 | ADD | <div style='text-align: right'>1</div>  |
| leaf_verifier | DivE | 1 | BBE4DIV | <div style='text-align: right'>9,380</div>  |
| leaf_verifier | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>72</div>  |
| leaf_verifier | DivEIN | 1 | STOREW | <div style='text-align: right'>288</div>  |
| leaf_verifier | DivFIN | 1 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier | For | 1 | ADD | <div style='text-align: right'>713,889</div>  |
| leaf_verifier | For | 1 | BNE | <div style='text-align: right'>792,664</div>  |
| leaf_verifier | For | 1 | JAL | <div style='text-align: right'>78,775</div>  |
| leaf_verifier | For | 1 | LOADW | <div style='text-align: right'>4,956</div>  |
| leaf_verifier | For | 1 | STOREW | <div style='text-align: right'>73,819</div>  |
| leaf_verifier | FriReducedOpening | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier | HintBitsF | 1 | PHANTOM | <div style='text-align: right'>86</div>  |
| leaf_verifier | HintInputVec | 1 | PHANTOM | <div style='text-align: right'>43,313</div>  |
| leaf_verifier | IfEq | 1 | BNE | <div style='text-align: right'>33,630</div>  |
| leaf_verifier | IfEqI | 1 | BNE | <div style='text-align: right'>229,789</div>  |
| leaf_verifier | IfEqI | 1 | JAL | <div style='text-align: right'>52,817</div>  |
| leaf_verifier | IfNe | 1 | BEQ | <div style='text-align: right'>31,534</div>  |
| leaf_verifier | IfNe | 1 | JAL | <div style='text-align: right'>4</div>  |
| leaf_verifier | IfNeI | 1 | BEQ | <div style='text-align: right'>5,014</div>  |
| leaf_verifier | ImmE | 1 | STOREW | <div style='text-align: right'>5,056</div>  |
| leaf_verifier | ImmF | 1 | STOREW | <div style='text-align: right'>74,580</div>  |
| leaf_verifier | ImmV | 1 | STOREW | <div style='text-align: right'>54,371</div>  |
| leaf_verifier | LoadE | 1 | LOADW | <div style='text-align: right'>39,912</div>  |
| leaf_verifier | LoadE | 1 | LOADW2 | <div style='text-align: right'>103,272</div>  |
| leaf_verifier | LoadF | 1 | LOADW | <div style='text-align: right'>53,298</div>  |
| leaf_verifier | LoadF | 1 | LOADW2 | <div style='text-align: right'>133,284</div>  |
| leaf_verifier | LoadV | 1 | LOADW | <div style='text-align: right'>51,836</div>  |
| leaf_verifier | LoadV | 1 | LOADW2 | <div style='text-align: right'>316,004</div>  |
| leaf_verifier | MulE | 1 | BBE4MUL | <div style='text-align: right'>16,718</div>  |
| leaf_verifier | MulEF | 1 | MUL | <div style='text-align: right'>7,296</div>  |
| leaf_verifier | MulEFI | 1 | MUL | <div style='text-align: right'>440</div>  |
| leaf_verifier | MulEI | 1 | BBE4MUL | <div style='text-align: right'>2,040</div>  |
| leaf_verifier | MulEI | 1 | STOREW | <div style='text-align: right'>8,160</div>  |
| leaf_verifier | MulF | 1 | MUL | <div style='text-align: right'>136,996</div>  |
| leaf_verifier | MulFI | 1 | MUL | <div style='text-align: right'>28</div>  |
| leaf_verifier | MulV | 1 | MUL | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | MulVI | 1 | MUL | <div style='text-align: right'>38,012</div>  |
| leaf_verifier | NegE | 1 | MUL | <div style='text-align: right'>256</div>  |
| leaf_verifier | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>33,768</div>  |
| leaf_verifier | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>16,216</div>  |
| leaf_verifier | Publish | 1 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier | StoreE | 1 | STOREW | <div style='text-align: right'>48,752</div>  |
| leaf_verifier | StoreE | 1 | STOREW2 | <div style='text-align: right'>52,584</div>  |
| leaf_verifier | StoreF | 1 | STOREW | <div style='text-align: right'>64,680</div>  |
| leaf_verifier | StoreF | 1 | STOREW2 | <div style='text-align: right'>121,104</div>  |
| leaf_verifier | StoreHintWord | 1 | ADD | <div style='text-align: right'>388,599</div>  |
| leaf_verifier | StoreHintWord | 1 | SHINTW | <div style='text-align: right'>434,578</div>  |
| leaf_verifier | StoreV | 1 | STOREW | <div style='text-align: right'>5,800</div>  |
| leaf_verifier | StoreV | 1 | STOREW2 | <div style='text-align: right'>109,676</div>  |
| leaf_verifier | SubE | 1 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier | SubEF | 1 | LOADW | <div style='text-align: right'>23,220</div>  |
| leaf_verifier | SubEF | 1 | SUB | <div style='text-align: right'>7,740</div>  |
| leaf_verifier | SubEFI | 1 | ADD | <div style='text-align: right'>320</div>  |
| leaf_verifier | SubEI | 1 | ADD | <div style='text-align: right'>576</div>  |
| leaf_verifier | SubV | 1 | SUB | <div style='text-align: right'>82,264</div>  |
| leaf_verifier | SubVI | 1 | SUB | <div style='text-align: right'>4,654</div>  |
| leaf_verifier | SubVIN | 1 | SUB | <div style='text-align: right'>1,680</div>  |
| leaf_verifier |  | 2 | JAL | <div style='text-align: right'>1</div>  |
| leaf_verifier |  | 2 | STOREW | <div style='text-align: right'>2</div>  |
| leaf_verifier | AddE | 2 | FE4ADD | <div style='text-align: right'>19,436</div>  |
| leaf_verifier | AddEFFI | 2 | LOADW | <div style='text-align: right'>328</div>  |
| leaf_verifier | AddEFFI | 2 | STOREW | <div style='text-align: right'>984</div>  |
| leaf_verifier | AddEFI | 2 | ADD | <div style='text-align: right'>344</div>  |
| leaf_verifier | AddEI | 2 | ADD | <div style='text-align: right'>50,904</div>  |
| leaf_verifier | AddFI | 2 | ADD | <div style='text-align: right'>72,832</div>  |
| leaf_verifier | AddV | 2 | ADD | <div style='text-align: right'>27,384</div>  |
| leaf_verifier | AddVI | 2 | ADD | <div style='text-align: right'>587,739</div>  |
| leaf_verifier | Alloc | 2 | ADD | <div style='text-align: right'>107,553</div>  |
| leaf_verifier | Alloc | 2 | LOADW | <div style='text-align: right'>107,553</div>  |
| leaf_verifier | Alloc | 2 | MUL | <div style='text-align: right'>64,240</div>  |
| leaf_verifier | AssertEqE | 2 | BNE | <div style='text-align: right'>448</div>  |
| leaf_verifier | AssertEqEI | 2 | BNE | <div style='text-align: right'>8</div>  |
| leaf_verifier | AssertEqF | 2 | BNE | <div style='text-align: right'>18,919</div>  |
| leaf_verifier | AssertEqFI | 2 | BNE | <div style='text-align: right'>1</div>  |
| leaf_verifier | AssertEqV | 2 | BNE | <div style='text-align: right'>4,680</div>  |
| leaf_verifier | AssertEqVI | 2 | BNE | <div style='text-align: right'>342</div>  |
| leaf_verifier | AssertNeVI | 2 | BEQ | <div style='text-align: right'>1</div>  |
| leaf_verifier | CT-ExtractPublicValuesCommit | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-InitializePcsConst | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-ReadProofsFromInput | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-VerifyProofs | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-compute-reduced-opening | 2 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-exp-reverse-bits-len | 2 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-poseidon2-hash | 2 | PHANTOM | <div style='text-align: right'>6,384</div>  |
| leaf_verifier | CT-poseidon2-hash-ext | 2 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-poseidon2-hash-setup | 2 | PHANTOM | <div style='text-align: right'>213,192</div>  |
| leaf_verifier | CT-single-reduced-opening-eval | 2 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| leaf_verifier | CT-stage-c-build-rounds | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-2-fri-fold | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-3-verify-challenges | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-verify-pcs | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-e-verify-constraints | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-verify-batch | 2 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-verify-batch-ext | 2 | PHANTOM | <div style='text-align: right'>3,360</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast | 2 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast-setup | 2 | PHANTOM | <div style='text-align: right'>9,744</div>  |
| leaf_verifier | CT-verify-query | 2 | PHANTOM | <div style='text-align: right'>168</div>  |
| leaf_verifier | CastFV | 2 | ADD | <div style='text-align: right'>1</div>  |
| leaf_verifier | DivE | 2 | BBE4DIV | <div style='text-align: right'>9,380</div>  |
| leaf_verifier | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>72</div>  |
| leaf_verifier | DivEIN | 2 | STOREW | <div style='text-align: right'>288</div>  |
| leaf_verifier | DivFIN | 2 | DIV | <div style='text-align: right'>172</div>  |
| leaf_verifier | For | 2 | ADD | <div style='text-align: right'>713,889</div>  |
| leaf_verifier | For | 2 | BNE | <div style='text-align: right'>792,664</div>  |
| leaf_verifier | For | 2 | JAL | <div style='text-align: right'>78,775</div>  |
| leaf_verifier | For | 2 | LOADW | <div style='text-align: right'>4,956</div>  |
| leaf_verifier | For | 2 | STOREW | <div style='text-align: right'>73,819</div>  |
| leaf_verifier | FriReducedOpening | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>7,644</div>  |
| leaf_verifier | HintBitsF | 2 | PHANTOM | <div style='text-align: right'>86</div>  |
| leaf_verifier | HintInputVec | 2 | PHANTOM | <div style='text-align: right'>43,313</div>  |
| leaf_verifier | IfEq | 2 | BNE | <div style='text-align: right'>33,630</div>  |
| leaf_verifier | IfEqI | 2 | BNE | <div style='text-align: right'>229,789</div>  |
| leaf_verifier | IfEqI | 2 | JAL | <div style='text-align: right'>48,395</div>  |
| leaf_verifier | IfNe | 2 | BEQ | <div style='text-align: right'>31,534</div>  |
| leaf_verifier | IfNe | 2 | JAL | <div style='text-align: right'>4</div>  |
| leaf_verifier | IfNeI | 2 | BEQ | <div style='text-align: right'>5,014</div>  |
| leaf_verifier | ImmE | 2 | STOREW | <div style='text-align: right'>5,056</div>  |
| leaf_verifier | ImmF | 2 | STOREW | <div style='text-align: right'>74,580</div>  |
| leaf_verifier | ImmV | 2 | STOREW | <div style='text-align: right'>54,371</div>  |
| leaf_verifier | LoadE | 2 | LOADW | <div style='text-align: right'>39,912</div>  |
| leaf_verifier | LoadE | 2 | LOADW2 | <div style='text-align: right'>103,272</div>  |
| leaf_verifier | LoadF | 2 | LOADW | <div style='text-align: right'>53,298</div>  |
| leaf_verifier | LoadF | 2 | LOADW2 | <div style='text-align: right'>133,284</div>  |
| leaf_verifier | LoadV | 2 | LOADW | <div style='text-align: right'>51,836</div>  |
| leaf_verifier | LoadV | 2 | LOADW2 | <div style='text-align: right'>316,004</div>  |
| leaf_verifier | MulE | 2 | BBE4MUL | <div style='text-align: right'>16,718</div>  |
| leaf_verifier | MulEF | 2 | MUL | <div style='text-align: right'>7,296</div>  |
| leaf_verifier | MulEFI | 2 | MUL | <div style='text-align: right'>440</div>  |
| leaf_verifier | MulEI | 2 | BBE4MUL | <div style='text-align: right'>2,040</div>  |
| leaf_verifier | MulEI | 2 | STOREW | <div style='text-align: right'>8,160</div>  |
| leaf_verifier | MulF | 2 | MUL | <div style='text-align: right'>136,996</div>  |
| leaf_verifier | MulFI | 2 | MUL | <div style='text-align: right'>28</div>  |
| leaf_verifier | MulV | 2 | MUL | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | MulVI | 2 | MUL | <div style='text-align: right'>38,012</div>  |
| leaf_verifier | NegE | 2 | MUL | <div style='text-align: right'>256</div>  |
| leaf_verifier | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>33,768</div>  |
| leaf_verifier | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>16,216</div>  |
| leaf_verifier | Publish | 2 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier | StoreE | 2 | STOREW | <div style='text-align: right'>48,752</div>  |
| leaf_verifier | StoreE | 2 | STOREW2 | <div style='text-align: right'>52,584</div>  |
| leaf_verifier | StoreF | 2 | STOREW | <div style='text-align: right'>64,680</div>  |
| leaf_verifier | StoreF | 2 | STOREW2 | <div style='text-align: right'>121,104</div>  |
| leaf_verifier | StoreHintWord | 2 | ADD | <div style='text-align: right'>388,599</div>  |
| leaf_verifier | StoreHintWord | 2 | SHINTW | <div style='text-align: right'>434,578</div>  |
| leaf_verifier | StoreV | 2 | STOREW | <div style='text-align: right'>5,800</div>  |
| leaf_verifier | StoreV | 2 | STOREW2 | <div style='text-align: right'>109,676</div>  |
| leaf_verifier | SubE | 2 | FE4SUB | <div style='text-align: right'>6,594</div>  |
| leaf_verifier | SubEF | 2 | LOADW | <div style='text-align: right'>23,220</div>  |
| leaf_verifier | SubEF | 2 | SUB | <div style='text-align: right'>7,740</div>  |
| leaf_verifier | SubEFI | 2 | ADD | <div style='text-align: right'>320</div>  |
| leaf_verifier | SubEI | 2 | ADD | <div style='text-align: right'>576</div>  |
| leaf_verifier | SubV | 2 | SUB | <div style='text-align: right'>82,264</div>  |
| leaf_verifier | SubVI | 2 | SUB | <div style='text-align: right'>4,654</div>  |
| leaf_verifier | SubVIN | 2 | SUB | <div style='text-align: right'>1,680</div>  |
| leaf_verifier |  | 3 | JAL | <div style='text-align: right'>1</div>  |
| leaf_verifier |  | 3 | STOREW | <div style='text-align: right'>2</div>  |
| leaf_verifier | AddE | 3 | FE4ADD | <div style='text-align: right'>20,471</div>  |
| leaf_verifier | AddEFFI | 3 | LOADW | <div style='text-align: right'>328</div>  |
| leaf_verifier | AddEFFI | 3 | STOREW | <div style='text-align: right'>984</div>  |
| leaf_verifier | AddEFI | 3 | ADD | <div style='text-align: right'>400</div>  |
| leaf_verifier | AddEI | 3 | ADD | <div style='text-align: right'>51,988</div>  |
| leaf_verifier | AddFI | 3 | ADD | <div style='text-align: right'>73,547</div>  |
| leaf_verifier | AddV | 3 | ADD | <div style='text-align: right'>26,806</div>  |
| leaf_verifier | AddVI | 3 | ADD | <div style='text-align: right'>599,670</div>  |
| leaf_verifier | Alloc | 3 | ADD | <div style='text-align: right'>106,595</div>  |
| leaf_verifier | Alloc | 3 | LOADW | <div style='text-align: right'>106,595</div>  |
| leaf_verifier | Alloc | 3 | MUL | <div style='text-align: right'>63,610</div>  |
| leaf_verifier | AssertEqE | 3 | BNE | <div style='text-align: right'>456</div>  |
| leaf_verifier | AssertEqEI | 3 | BNE | <div style='text-align: right'>8</div>  |
| leaf_verifier | AssertEqF | 3 | BNE | <div style='text-align: right'>18,591</div>  |
| leaf_verifier | AssertEqFI | 3 | BNE | <div style='text-align: right'>1</div>  |
| leaf_verifier | AssertEqV | 3 | BNE | <div style='text-align: right'>4,660</div>  |
| leaf_verifier | AssertEqVI | 3 | BNE | <div style='text-align: right'>365</div>  |
| leaf_verifier | AssertNeVI | 3 | BEQ | <div style='text-align: right'>1</div>  |
| leaf_verifier | CT-ExtractPublicValuesCommit | 3 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-InitializePcsConst | 3 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-ReadProofsFromInput | 3 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-VerifyProofs | 3 | PHANTOM | <div style='text-align: right'>2</div>  |
| leaf_verifier | CT-compute-reduced-opening | 3 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-exp-reverse-bits-len | 3 | PHANTOM | <div style='text-align: right'>10,416</div>  |
| leaf_verifier | CT-poseidon2-hash | 3 | PHANTOM | <div style='text-align: right'>6,384</div>  |
| leaf_verifier | CT-poseidon2-hash-ext | 3 | PHANTOM | <div style='text-align: right'>3,276</div>  |
| leaf_verifier | CT-poseidon2-hash-setup | 3 | PHANTOM | <div style='text-align: right'>229,320</div>  |
| leaf_verifier | CT-single-reduced-opening-eval | 3 | PHANTOM | <div style='text-align: right'>16,296</div>  |
| leaf_verifier | CT-stage-c-build-rounds | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-1-verify-shape-and-sample-challenges | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-2-fri-fold | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-3-verify-challenges | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-d-verify-pcs | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-stage-e-verify-constraints | 3 | PHANTOM | <div style='text-align: right'>4</div>  |
| leaf_verifier | CT-verify-batch | 3 | PHANTOM | <div style='text-align: right'>1,344</div>  |
| leaf_verifier | CT-verify-batch-ext | 3 | PHANTOM | <div style='text-align: right'>3,276</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast | 3 | PHANTOM | <div style='text-align: right'>9,660</div>  |
| leaf_verifier | CT-verify-batch-reduce-fast-setup | 3 | PHANTOM | <div style='text-align: right'>9,660</div>  |
| leaf_verifier | CT-verify-query | 3 | PHANTOM | <div style='text-align: right'>168</div>  |
| leaf_verifier | CastFV | 3 | ADD | <div style='text-align: right'>1</div>  |
| leaf_verifier | DivE | 3 | BBE4DIV | <div style='text-align: right'>9,846</div>  |
| leaf_verifier | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>78</div>  |
| leaf_verifier | DivEIN | 3 | STOREW | <div style='text-align: right'>312</div>  |
| leaf_verifier | DivFIN | 3 | DIV | <div style='text-align: right'>186</div>  |
| leaf_verifier | For | 3 | ADD | <div style='text-align: right'>725,867</div>  |
| leaf_verifier | For | 3 | BNE | <div style='text-align: right'>805,105</div>  |
| leaf_verifier | For | 3 | JAL | <div style='text-align: right'>79,238</div>  |
| leaf_verifier | For | 3 | LOADW | <div style='text-align: right'>4,914</div>  |
| leaf_verifier | For | 3 | STOREW | <div style='text-align: right'>74,324</div>  |
| leaf_verifier | FriReducedOpening | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>8,148</div>  |
| leaf_verifier | HintBitsF | 3 | PHANTOM | <div style='text-align: right'>86</div>  |
| leaf_verifier | HintInputVec | 3 | PHANTOM | <div style='text-align: right'>42,985</div>  |
| leaf_verifier | IfEq | 3 | BNE | <div style='text-align: right'>36,566</div>  |
| leaf_verifier | IfEqI | 3 | BNE | <div style='text-align: right'>237,103</div>  |
| leaf_verifier | IfEqI | 3 | JAL | <div style='text-align: right'>48,342</div>  |
| leaf_verifier | IfNe | 3 | BEQ | <div style='text-align: right'>30,522</div>  |
| leaf_verifier | IfNe | 3 | JAL | <div style='text-align: right'>4</div>  |
| leaf_verifier | IfNeI | 3 | BEQ | <div style='text-align: right'>4,982</div>  |
| leaf_verifier | ImmE | 3 | STOREW | <div style='text-align: right'>5,440</div>  |
| leaf_verifier | ImmF | 3 | STOREW | <div style='text-align: right'>74,920</div>  |
| leaf_verifier | ImmV | 3 | STOREW | <div style='text-align: right'>55,004</div>  |
| leaf_verifier | LoadE | 3 | LOADW | <div style='text-align: right'>40,896</div>  |
| leaf_verifier | LoadE | 3 | LOADW2 | <div style='text-align: right'>107,144</div>  |
| leaf_verifier | LoadF | 3 | LOADW | <div style='text-align: right'>53,520</div>  |
| leaf_verifier | LoadF | 3 | LOADW2 | <div style='text-align: right'>143,064</div>  |
| leaf_verifier | LoadV | 3 | LOADW | <div style='text-align: right'>51,467</div>  |
| leaf_verifier | LoadV | 3 | LOADW2 | <div style='text-align: right'>325,252</div>  |
| leaf_verifier | MulE | 3 | BBE4MUL | <div style='text-align: right'>17,044</div>  |
| leaf_verifier | MulEF | 3 | MUL | <div style='text-align: right'>7,176</div>  |
| leaf_verifier | MulEFI | 3 | MUL | <div style='text-align: right'>568</div>  |
| leaf_verifier | MulEI | 3 | BBE4MUL | <div style='text-align: right'>2,274</div>  |
| leaf_verifier | MulEI | 3 | STOREW | <div style='text-align: right'>9,096</div>  |
| leaf_verifier | MulF | 3 | MUL | <div style='text-align: right'>138,024</div>  |
| leaf_verifier | MulFI | 3 | MUL | <div style='text-align: right'>30</div>  |
| leaf_verifier | MulV | 3 | MUL | <div style='text-align: right'>2,666</div>  |
| leaf_verifier | MulVI | 3 | MUL | <div style='text-align: right'>37,312</div>  |
| leaf_verifier | NegE | 3 | MUL | <div style='text-align: right'>284</div>  |
| leaf_verifier | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>32,813</div>  |
| leaf_verifier | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>17,140</div>  |
| leaf_verifier | Publish | 3 | PUBLISH | <div style='text-align: right'>36</div>  |
| leaf_verifier | StoreE | 3 | STOREW | <div style='text-align: right'>48,104</div>  |
| leaf_verifier | StoreE | 3 | STOREW2 | <div style='text-align: right'>54,932</div>  |
| leaf_verifier | StoreF | 3 | STOREW | <div style='text-align: right'>65,192</div>  |
| leaf_verifier | StoreF | 3 | STOREW2 | <div style='text-align: right'>129,288</div>  |
| leaf_verifier | StoreHintWord | 3 | ADD | <div style='text-align: right'>390,276</div>  |
| leaf_verifier | StoreHintWord | 3 | SHINTW | <div style='text-align: right'>435,927</div>  |
| leaf_verifier | StoreV | 3 | STOREW | <div style='text-align: right'>5,704</div>  |
| leaf_verifier | StoreV | 3 | STOREW2 | <div style='text-align: right'>110,038</div>  |
| leaf_verifier | SubE | 3 | FE4SUB | <div style='text-align: right'>6,541</div>  |
| leaf_verifier | SubEF | 3 | LOADW | <div style='text-align: right'>24,744</div>  |
| leaf_verifier | SubEF | 3 | SUB | <div style='text-align: right'>8,248</div>  |
| leaf_verifier | SubEFI | 3 | ADD | <div style='text-align: right'>380</div>  |
| leaf_verifier | SubEI | 3 | ADD | <div style='text-align: right'>624</div>  |
| leaf_verifier | SubV | 3 | SUB | <div style='text-align: right'>83,148</div>  |
| leaf_verifier | SubVI | 3 | SUB | <div style='text-align: right'>4,610</div>  |
| leaf_verifier | SubVIN | 3 | SUB | <div style='text-align: right'>1,638</div>  |

| group | air_name | dsl_ir | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 0 | JAL | <div style='text-align: right'>10</div>  |
| leaf_verifier | Boundary |  | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | STOREW | <div style='text-align: right'>82</div>  |
| leaf_verifier | Boundary |  | 0 | STOREW | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | FE4ADD | <div style='text-align: right'>912,680</div>  |
| leaf_verifier | AccessAdapter<2> | AddE | 0 | FE4ADD | <div style='text-align: right'>383,460</div>  |
| leaf_verifier | AccessAdapter<4> | AddE | 0 | FE4ADD | <div style='text-align: right'>226,590</div>  |
| leaf_verifier | Boundary | AddE | 0 | FE4ADD | <div style='text-align: right'>114,576</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | LOADW | <div style='text-align: right'>13,940</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 0 | LOADW | <div style='text-align: right'>2,354</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFFI | 0 | LOADW | <div style='text-align: right'>2,782</div>  |
| leaf_verifier | Boundary | AddEFFI | 0 | LOADW | <div style='text-align: right'>308</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | STOREW | <div style='text-align: right'>41,820</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 0 | STOREW | <div style='text-align: right'>2,354</div>  |
| leaf_verifier | Boundary | AddEFFI | 0 | STOREW | <div style='text-align: right'>924</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 0 | ADD | <div style='text-align: right'>14,520</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFI | 0 | ADD | <div style='text-align: right'>2,420</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFI | 0 | ADD | <div style='text-align: right'>1,430</div>  |
| leaf_verifier | Boundary | AddEFI | 0 | ADD | <div style='text-align: right'>1,364</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | ADD | <div style='text-align: right'>1,668,360</div>  |
| leaf_verifier | AccessAdapter<2> | AddEI | 0 | ADD | <div style='text-align: right'>346,060</div>  |
| leaf_verifier | AccessAdapter<4> | AddEI | 0 | ADD | <div style='text-align: right'>204,490</div>  |
| leaf_verifier | Boundary | AddEI | 0 | ADD | <div style='text-align: right'>135,564</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | ADD | <div style='text-align: right'>2,429,130</div>  |
| leaf_verifier | Boundary | AddFI | 0 | ADD | <div style='text-align: right'>539</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | ADD | <div style='text-align: right'>854,820</div>  |
| leaf_verifier | Boundary | AddV | 0 | ADD | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | ADD | <div style='text-align: right'>19,417,410</div>  |
| leaf_verifier | Boundary | AddVI | 0 | ADD | <div style='text-align: right'>15,323</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | ADD | <div style='text-align: right'>3,300,390</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 0 | LOADW | <div style='text-align: right'>4,510,533</div>  |
| leaf_verifier | Boundary | Alloc | 0 | LOADW | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | MUL | <div style='text-align: right'>1,967,340</div>  |
| leaf_verifier | AccessAdapter<2> | Alloc | 0 | MUL | <div style='text-align: right'>66</div>  |
| leaf_verifier | AccessAdapter<4> | Alloc | 0 | MUL | <div style='text-align: right'>78</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 0 | BNE | <div style='text-align: right'>10,856</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqE | 0 | BNE | <div style='text-align: right'>2,596</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqE | 0 | BNE | <div style='text-align: right'>1,534</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 0 | BNE | <div style='text-align: right'>184</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqEI | 0 | BNE | <div style='text-align: right'>44</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqEI | 0 | BNE | <div style='text-align: right'>26</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 0 | BNE | <div style='text-align: right'>435,137</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 0 | BNE | <div style='text-align: right'>23</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 0 | BNE | <div style='text-align: right'>109,158</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 0 | BNE | <div style='text-align: right'>9,384</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 0 | BEQ | <div style='text-align: right'>23</div>  |
| leaf_verifier | PhantomAir | CT-ExtractPublicValuesCommit | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-InitializePcsConst | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-ReadProofsFromInput | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-VerifyProofs | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-compute-reduced-opening | 0 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-exp-reverse-bits-len | 0 | PHANTOM | <div style='text-align: right'>70,560</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash | 0 | PHANTOM | <div style='text-align: right'>39,816</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-ext | 0 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-setup | 0 | PHANTOM | <div style='text-align: right'>1,545,264</div>  |
| leaf_verifier | PhantomAir | CT-single-reduced-opening-eval | 0 | PHANTOM | <div style='text-align: right'>109,872</div>  |
| leaf_verifier | PhantomAir | CT-stage-c-build-rounds | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-2-fri-fold | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-3-verify-challenges | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-verify-pcs | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-e-verify-constraints | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch | 0 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-ext | 0 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast | 0 | PHANTOM | <div style='text-align: right'>59,976</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | 0 | PHANTOM | <div style='text-align: right'>59,976</div>  |
| leaf_verifier | PhantomAir | CT-verify-query | 0 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 0 | ADD | <div style='text-align: right'>30</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 0 | BBE4DIV | <div style='text-align: right'>436,160</div>  |
| leaf_verifier | AccessAdapter<2> | DivE | 0 | BBE4DIV | <div style='text-align: right'>203,720</div>  |
| leaf_verifier | AccessAdapter<4> | DivE | 0 | BBE4DIV | <div style='text-align: right'>120,380</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>3,600</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>3,718</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>2,197</div>  |
| leaf_verifier | Boundary | DivEIN | 0 | BBE4DIV | <div style='text-align: right'>528</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 0 | STOREW | <div style='text-align: right'>14,760</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 0 | STOREW | <div style='text-align: right'>1,298</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 0 | STOREW | <div style='text-align: right'>364</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 0 | DIV | <div style='text-align: right'>6,420</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | ADD | <div style='text-align: right'>23,578,110</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 0 | BNE | <div style='text-align: right'>19,991,876</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | For | 0 | JAL | <div style='text-align: right'>832,750</div>  |
| leaf_verifier | AccessAdapter<2> | For | 0 | JAL | <div style='text-align: right'>682</div>  |
| leaf_verifier | AccessAdapter<4> | For | 0 | JAL | <div style='text-align: right'>806</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | LOADW | <div style='text-align: right'>208,362</div>  |
| leaf_verifier | Boundary | For | 0 | LOADW | <div style='text-align: right'>473</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | STOREW | <div style='text-align: right'>3,205,913</div>  |
| leaf_verifier | Boundary | For | 0 | STOREW | <div style='text-align: right'>726</div>  |
| leaf_verifier | AccessAdapter<2> | FriReducedOpening | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>258,808</div>  |
| leaf_verifier | AccessAdapter<4> | FriReducedOpening | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>152,932</div>  |
| leaf_verifier | FriReducedOpeningAir | FriReducedOpening | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>15,816,192</div>  |
| leaf_verifier | PhantomAir | HintBitsF | 0 | PHANTOM | <div style='text-align: right'>516</div>  |
| leaf_verifier | PhantomAir | HintInputVec | 0 | PHANTOM | <div style='text-align: right'>266,610</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | BNE | <div style='text-align: right'>1,014,024</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | BNE | <div style='text-align: right'>5,981,633</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 0 | JAL | <div style='text-align: right'>556,700</div>  |
| leaf_verifier | AccessAdapter<2> | IfEqI | 0 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | AccessAdapter<4> | IfEqI | 0 | JAL | <div style='text-align: right'>13</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | BEQ | <div style='text-align: right'>725,282</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | 0 | JAL | <div style='text-align: right'>40</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | BEQ | <div style='text-align: right'>118,910</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 0 | STOREW | <div style='text-align: right'>257,480</div>  |
| leaf_verifier | AccessAdapter<2> | ImmE | 0 | STOREW | <div style='text-align: right'>17,050</div>  |
| leaf_verifier | AccessAdapter<4> | ImmE | 0 | STOREW | <div style='text-align: right'>10,075</div>  |
| leaf_verifier | Boundary | ImmE | 0 | STOREW | <div style='text-align: right'>13,332</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 0 | STOREW | <div style='text-align: right'>3,223,584</div>  |
| leaf_verifier | Boundary | ImmF | 0 | STOREW | <div style='text-align: right'>1,573</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 0 | STOREW | <div style='text-align: right'>2,370,415</div>  |
| leaf_verifier | Boundary | ImmV | 0 | STOREW | <div style='text-align: right'>16,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | LOADW | <div style='text-align: right'>1,803,672</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 0 | LOADW | <div style='text-align: right'>303,402</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 0 | LOADW | <div style='text-align: right'>179,283</div>  |
| leaf_verifier | Boundary | LoadE | 0 | LOADW | <div style='text-align: right'>3,740</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | LOADW2 | <div style='text-align: right'>4,813,728</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 0 | LOADW2 | <div style='text-align: right'>112,838</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 0 | LOADW2 | <div style='text-align: right'>66,677</div>  |
| leaf_verifier | Boundary | LoadE | 0 | LOADW2 | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | LOADW | <div style='text-align: right'>2,185,218</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 0 | LOADW | <div style='text-align: right'>103,488</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 0 | LOADW | <div style='text-align: right'>61,152</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary | LoadF | 0 | LOADW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | LOADW2 | <div style='text-align: right'>6,585,420</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 0 | LOADW2 | <div style='text-align: right'>1,584</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 0 | LOADW2 | <div style='text-align: right'>936</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 0 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary | LoadF | 0 | LOADW2 | <div style='text-align: right'>550</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | LOADW | <div style='text-align: right'>2,176,444</div>  |
| leaf_verifier | Boundary | LoadV | 0 | LOADW | <div style='text-align: right'>15,180</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | LOADW2 | <div style='text-align: right'>14,713,834</div>  |
| leaf_verifier | Boundary | LoadV | 0 | LOADW2 | <div style='text-align: right'>550</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 0 | BBE4MUL | <div style='text-align: right'>741,680</div>  |
| leaf_verifier | AccessAdapter<2> | MulE | 0 | BBE4MUL | <div style='text-align: right'>355,080</div>  |
| leaf_verifier | AccessAdapter<4> | MulE | 0 | BBE4MUL | <div style='text-align: right'>209,820</div>  |
| leaf_verifier | Boundary | MulE | 0 | BBE4MUL | <div style='text-align: right'>135,916</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | MUL | <div style='text-align: right'>223,200</div>  |
| leaf_verifier | AccessAdapter<2> | MulEF | 0 | MUL | <div style='text-align: right'>38,126</div>  |
| leaf_verifier | AccessAdapter<4> | MulEF | 0 | MUL | <div style='text-align: right'>22,529</div>  |
| leaf_verifier | Boundary | MulEF | 0 | MUL | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 0 | MUL | <div style='text-align: right'>22,200</div>  |
| leaf_verifier | AccessAdapter<2> | MulEFI | 0 | MUL | <div style='text-align: right'>4,092</div>  |
| leaf_verifier | AccessAdapter<4> | MulEFI | 0 | MUL | <div style='text-align: right'>2,418</div>  |
| leaf_verifier | Boundary | MulEFI | 0 | MUL | <div style='text-align: right'>1,364</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | BBE4MUL | <div style='text-align: right'>106,640</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 0 | BBE4MUL | <div style='text-align: right'>120,010</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 0 | BBE4MUL | <div style='text-align: right'>70,915</div>  |
| leaf_verifier | Boundary | MulEI | 0 | BBE4MUL | <div style='text-align: right'>4,312</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 0 | STOREW | <div style='text-align: right'>437,224</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 0 | STOREW | <div style='text-align: right'>58,553</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 0 | STOREW | <div style='text-align: right'>34,554</div>  |
| leaf_verifier | Boundary | MulEI | 0 | STOREW | <div style='text-align: right'>33</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | MUL | <div style='text-align: right'>4,565,280</div>  |
| leaf_verifier | Boundary | MulF | 0 | MUL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | MUL | <div style='text-align: right'>1,020</div>  |
| leaf_verifier | Boundary | MulFI | 0 | MUL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | 0 | MUL | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | Boundary | MulV | 0 | MUL | <div style='text-align: right'>14,641</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | MUL | <div style='text-align: right'>1,171,860</div>  |
| leaf_verifier | Boundary | MulVI | 0 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 0 | MUL | <div style='text-align: right'>9,960</div>  |
| leaf_verifier | AccessAdapter<2> | NegE | 0 | MUL | <div style='text-align: right'>2,728</div>  |
| leaf_verifier | AccessAdapter<4> | NegE | 0 | MUL | <div style='text-align: right'>1,612</div>  |
| leaf_verifier | Boundary | NegE | 0 | MUL | <div style='text-align: right'>792</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>1,374,912</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>812,448</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>531,216</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 0 | COMP_POS2 | <div style='text-align: right'>18,946,746</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>1,029,688</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>611,182</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>408,561</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 0 | PERM_POS2 | <div style='text-align: right'>10,639,447</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 0 | PUBLISH | <div style='text-align: right'>972</div>  |
| leaf_verifier | Boundary | Publish | 0 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | STOREW | <div style='text-align: right'>2,001,784</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 0 | STOREW | <div style='text-align: right'>37,048</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 0 | STOREW | <div style='text-align: right'>21,892</div>  |
| leaf_verifier | Boundary | StoreE | 0 | STOREW | <div style='text-align: right'>537,064</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | STOREW2 | <div style='text-align: right'>2,486,568</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 0 | STOREW2 | <div style='text-align: right'>258,720</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 0 | STOREW2 | <div style='text-align: right'>152,880</div>  |
| leaf_verifier | Boundary | StoreE | 0 | STOREW2 | <div style='text-align: right'>75,768</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | STOREW | <div style='text-align: right'>2,734,536</div>  |
| leaf_verifier | Boundary | StoreF | 0 | STOREW | <div style='text-align: right'>733,656</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | STOREW2 | <div style='text-align: right'>5,879,400</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 0 | STOREW2 | <div style='text-align: right'>588,148</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 0 | STOREW2 | <div style='text-align: right'>350,272</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 0 | STOREW2 | <div style='text-align: right'>237,609</div>  |
| leaf_verifier | Boundary | StoreF | 0 | STOREW2 | <div style='text-align: right'>153,934</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | ADD | <div style='text-align: right'>12,447,090</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 0 | SHINTW | <div style='text-align: right'>18,942,164</div>  |
| leaf_verifier | Boundary | StoreHintWord | 0 | SHINTW | <div style='text-align: right'>5,082,044</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | STOREW | <div style='text-align: right'>241,490</div>  |
| leaf_verifier | Boundary | StoreV | 0 | STOREW | <div style='text-align: right'>64,790</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | STOREW2 | <div style='text-align: right'>4,804,954</div>  |
| leaf_verifier | Boundary | StoreV | 0 | STOREW2 | <div style='text-align: right'>1,178,848</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 0 | FE4SUB | <div style='text-align: right'>274,160</div>  |
| leaf_verifier | AccessAdapter<2> | SubE | 0 | FE4SUB | <div style='text-align: right'>234,146</div>  |
| leaf_verifier | AccessAdapter<4> | SubE | 0 | FE4SUB | <div style='text-align: right'>138,359</div>  |
| leaf_verifier | Boundary | SubE | 0 | FE4SUB | <div style='text-align: right'>27,896</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 0 | LOADW | <div style='text-align: right'>1,139,472</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 0 | LOADW | <div style='text-align: right'>101,618</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 0 | SUB | <div style='text-align: right'>277,920</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 0 | SUB | <div style='text-align: right'>101,618</div>  |
| leaf_verifier | AccessAdapter<4> | SubEF | 0 | SUB | <div style='text-align: right'>120,094</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 0 | ADD | <div style='text-align: right'>15,480</div>  |
| leaf_verifier | AccessAdapter<2> | SubEFI | 0 | ADD | <div style='text-align: right'>2,486</div>  |
| leaf_verifier | AccessAdapter<4> | SubEFI | 0 | ADD | <div style='text-align: right'>1,469</div>  |
| leaf_verifier | Boundary | SubEFI | 0 | ADD | <div style='text-align: right'>220</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 0 | ADD | <div style='text-align: right'>21,600</div>  |
| leaf_verifier | AccessAdapter<2> | SubEI | 0 | ADD | <div style='text-align: right'>5,918</div>  |
| leaf_verifier | AccessAdapter<4> | SubEI | 0 | ADD | <div style='text-align: right'>3,497</div>  |
| leaf_verifier | Boundary | SubEI | 0 | ADD | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | SUB | <div style='text-align: right'>2,740,260</div>  |
| leaf_verifier | Boundary | SubV | 0 | SUB | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | SUB | <div style='text-align: right'>139,800</div>  |
| leaf_verifier | Boundary | SubVI | 0 | SUB | <div style='text-align: right'>15,147</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 0 | SUB | <div style='text-align: right'>50,400</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 1 | JAL | <div style='text-align: right'>10</div>  |
| leaf_verifier | Boundary |  | 1 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | STOREW | <div style='text-align: right'>82</div>  |
| leaf_verifier | Boundary |  | 1 | STOREW | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 1 | FE4ADD | <div style='text-align: right'>777,440</div>  |
| leaf_verifier | AccessAdapter<2> | AddE | 1 | FE4ADD | <div style='text-align: right'>339,240</div>  |
| leaf_verifier | AccessAdapter<4> | AddE | 1 | FE4ADD | <div style='text-align: right'>200,460</div>  |
| leaf_verifier | Boundary | AddE | 1 | FE4ADD | <div style='text-align: right'>119,724</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 1 | LOADW | <div style='text-align: right'>13,448</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 1 | LOADW | <div style='text-align: right'>2,277</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFFI | 1 | LOADW | <div style='text-align: right'>2,691</div>  |
| leaf_verifier | Boundary | AddEFFI | 1 | LOADW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 1 | STOREW | <div style='text-align: right'>40,344</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 1 | STOREW | <div style='text-align: right'>2,277</div>  |
| leaf_verifier | Boundary | AddEFFI | 1 | STOREW | <div style='text-align: right'>858</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 1 | ADD | <div style='text-align: right'>10,320</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFI | 1 | ADD | <div style='text-align: right'>1,540</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFI | 1 | ADD | <div style='text-align: right'>910</div>  |
| leaf_verifier | Boundary | AddEFI | 1 | ADD | <div style='text-align: right'>1,144</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 1 | ADD | <div style='text-align: right'>1,527,120</div>  |
| leaf_verifier | AccessAdapter<2> | AddEI | 1 | ADD | <div style='text-align: right'>304,260</div>  |
| leaf_verifier | AccessAdapter<4> | AddEI | 1 | ADD | <div style='text-align: right'>179,790</div>  |
| leaf_verifier | Boundary | AddEI | 1 | ADD | <div style='text-align: right'>128,788</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 1 | ADD | <div style='text-align: right'>2,188,020</div>  |
| leaf_verifier | Boundary | AddFI | 1 | ADD | <div style='text-align: right'>539</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 1 | ADD | <div style='text-align: right'>821,520</div>  |
| leaf_verifier | Boundary | AddV | 1 | ADD | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 1 | ADD | <div style='text-align: right'>17,632,170</div>  |
| leaf_verifier | Boundary | AddVI | 1 | ADD | <div style='text-align: right'>15,323</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 1 | ADD | <div style='text-align: right'>3,226,590</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 1 | LOADW | <div style='text-align: right'>4,409,673</div>  |
| leaf_verifier | Boundary | Alloc | 1 | LOADW | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 1 | MUL | <div style='text-align: right'>1,927,200</div>  |
| leaf_verifier | AccessAdapter<2> | Alloc | 1 | MUL | <div style='text-align: right'>66</div>  |
| leaf_verifier | AccessAdapter<4> | Alloc | 1 | MUL | <div style='text-align: right'>78</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 1 | BNE | <div style='text-align: right'>10,304</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqE | 1 | BNE | <div style='text-align: right'>2,464</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqE | 1 | BNE | <div style='text-align: right'>1,456</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 1 | BNE | <div style='text-align: right'>184</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqEI | 1 | BNE | <div style='text-align: right'>44</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqEI | 1 | BNE | <div style='text-align: right'>26</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 1 | BNE | <div style='text-align: right'>435,137</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 1 | BNE | <div style='text-align: right'>23</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 1 | BNE | <div style='text-align: right'>107,640</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 1 | BNE | <div style='text-align: right'>7,866</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 1 | BEQ | <div style='text-align: right'>23</div>  |
| leaf_verifier | PhantomAir | CT-ExtractPublicValuesCommit | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-InitializePcsConst | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-ReadProofsFromInput | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-VerifyProofs | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-compute-reduced-opening | 1 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-exp-reverse-bits-len | 1 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash | 1 | PHANTOM | <div style='text-align: right'>38,304</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-ext | 1 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-setup | 1 | PHANTOM | <div style='text-align: right'>1,279,152</div>  |
| leaf_verifier | PhantomAir | CT-single-reduced-opening-eval | 1 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| leaf_verifier | PhantomAir | CT-stage-c-build-rounds | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-2-fri-fold | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-3-verify-challenges | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-verify-pcs | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-e-verify-constraints | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch | 1 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-ext | 1 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast | 1 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | 1 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-verify-query | 1 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 1 | ADD | <div style='text-align: right'>30</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 1 | BBE4DIV | <div style='text-align: right'>375,200</div>  |
| leaf_verifier | AccessAdapter<2> | DivE | 1 | BBE4DIV | <div style='text-align: right'>170,060</div>  |
| leaf_verifier | AccessAdapter<4> | DivE | 1 | BBE4DIV | <div style='text-align: right'>100,490</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>2,880</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>3,014</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>1,781</div>  |
| leaf_verifier | Boundary | DivEIN | 1 | BBE4DIV | <div style='text-align: right'>440</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 1 | STOREW | <div style='text-align: right'>11,808</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 1 | STOREW | <div style='text-align: right'>1,034</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 1 | STOREW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 1 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 1 | ADD | <div style='text-align: right'>21,416,670</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 1 | BNE | <div style='text-align: right'>18,231,272</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | For | 1 | JAL | <div style='text-align: right'>787,750</div>  |
| leaf_verifier | AccessAdapter<2> | For | 1 | JAL | <div style='text-align: right'>550</div>  |
| leaf_verifier | AccessAdapter<4> | For | 1 | JAL | <div style='text-align: right'>650</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 1 | LOADW | <div style='text-align: right'>203,196</div>  |
| leaf_verifier | Boundary | For | 1 | LOADW | <div style='text-align: right'>473</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 1 | STOREW | <div style='text-align: right'>3,026,579</div>  |
| leaf_verifier | Boundary | For | 1 | STOREW | <div style='text-align: right'>627</div>  |
| leaf_verifier | AccessAdapter<2> | FriReducedOpening | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> | FriReducedOpening | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir | FriReducedOpening | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | PhantomAir | HintBitsF | 1 | PHANTOM | <div style='text-align: right'>516</div>  |
| leaf_verifier | PhantomAir | HintInputVec | 1 | PHANTOM | <div style='text-align: right'>259,878</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 1 | BNE | <div style='text-align: right'>773,490</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 1 | BNE | <div style='text-align: right'>5,285,147</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 1 | JAL | <div style='text-align: right'>528,170</div>  |
| leaf_verifier | AccessAdapter<2> | IfEqI | 1 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | AccessAdapter<4> | IfEqI | 1 | JAL | <div style='text-align: right'>13</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 1 | BEQ | <div style='text-align: right'>725,282</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | 1 | JAL | <div style='text-align: right'>40</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 1 | BEQ | <div style='text-align: right'>115,322</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 1 | STOREW | <div style='text-align: right'>207,296</div>  |
| leaf_verifier | AccessAdapter<2> | ImmE | 1 | STOREW | <div style='text-align: right'>11,880</div>  |
| leaf_verifier | AccessAdapter<4> | ImmE | 1 | STOREW | <div style='text-align: right'>7,020</div>  |
| leaf_verifier | Boundary | ImmE | 1 | STOREW | <div style='text-align: right'>12,232</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 1 | STOREW | <div style='text-align: right'>3,057,780</div>  |
| leaf_verifier | Boundary | ImmF | 1 | STOREW | <div style='text-align: right'>1,573</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 1 | STOREW | <div style='text-align: right'>2,229,211</div>  |
| leaf_verifier | Boundary | ImmV | 1 | STOREW | <div style='text-align: right'>16,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 1 | LOADW | <div style='text-align: right'>1,636,392</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 1 | LOADW | <div style='text-align: right'>264,528</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 1 | LOADW | <div style='text-align: right'>156,312</div>  |
| leaf_verifier | Boundary | LoadE | 1 | LOADW | <div style='text-align: right'>3,388</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 1 | LOADW2 | <div style='text-align: right'>4,234,152</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 1 | LOADW2 | <div style='text-align: right'>112,838</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 1 | LOADW2 | <div style='text-align: right'>66,677</div>  |
| leaf_verifier | Boundary | LoadE | 1 | LOADW2 | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 1 | LOADW | <div style='text-align: right'>2,185,218</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 1 | LOADW | <div style='text-align: right'>103,488</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 1 | LOADW | <div style='text-align: right'>61,152</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary | LoadF | 1 | LOADW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 1 | LOADW2 | <div style='text-align: right'>5,464,644</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 1 | LOADW2 | <div style='text-align: right'>1,584</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 1 | LOADW2 | <div style='text-align: right'>936</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 1 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary | LoadF | 1 | LOADW2 | <div style='text-align: right'>550</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 1 | LOADW | <div style='text-align: right'>2,125,276</div>  |
| leaf_verifier | Boundary | LoadV | 1 | LOADW | <div style='text-align: right'>15,246</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 1 | LOADW2 | <div style='text-align: right'>12,956,164</div>  |
| leaf_verifier | Boundary | LoadV | 1 | LOADW2 | <div style='text-align: right'>583</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 1 | BBE4MUL | <div style='text-align: right'>668,720</div>  |
| leaf_verifier | AccessAdapter<2> | MulE | 1 | BBE4MUL | <div style='text-align: right'>324,940</div>  |
| leaf_verifier | AccessAdapter<4> | MulE | 1 | BBE4MUL | <div style='text-align: right'>192,010</div>  |
| leaf_verifier | Boundary | MulE | 1 | BBE4MUL | <div style='text-align: right'>138,732</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 1 | MUL | <div style='text-align: right'>218,880</div>  |
| leaf_verifier | AccessAdapter<2> | MulEF | 1 | MUL | <div style='text-align: right'>37,818</div>  |
| leaf_verifier | AccessAdapter<4> | MulEF | 1 | MUL | <div style='text-align: right'>22,347</div>  |
| leaf_verifier | Boundary | MulEF | 1 | MUL | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 1 | MUL | <div style='text-align: right'>13,200</div>  |
| leaf_verifier | AccessAdapter<2> | MulEFI | 1 | MUL | <div style='text-align: right'>2,332</div>  |
| leaf_verifier | AccessAdapter<4> | MulEFI | 1 | MUL | <div style='text-align: right'>1,378</div>  |
| leaf_verifier | Boundary | MulEFI | 1 | MUL | <div style='text-align: right'>1,188</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 1 | BBE4MUL | <div style='text-align: right'>81,600</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 1 | BBE4MUL | <div style='text-align: right'>93,566</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 1 | BBE4MUL | <div style='text-align: right'>55,289</div>  |
| leaf_verifier | Boundary | MulEI | 1 | BBE4MUL | <div style='text-align: right'>6,204</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 1 | STOREW | <div style='text-align: right'>334,560</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 1 | STOREW | <div style='text-align: right'>44,781</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 1 | STOREW | <div style='text-align: right'>26,416</div>  |
| leaf_verifier | Boundary | MulEI | 1 | STOREW | <div style='text-align: right'>33</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 1 | MUL | <div style='text-align: right'>4,109,880</div>  |
| leaf_verifier | Boundary | MulF | 1 | MUL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 1 | MUL | <div style='text-align: right'>840</div>  |
| leaf_verifier | Boundary | MulFI | 1 | MUL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | 1 | MUL | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | Boundary | MulV | 1 | MUL | <div style='text-align: right'>14,641</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 1 | MUL | <div style='text-align: right'>1,140,360</div>  |
| leaf_verifier | Boundary | MulVI | 1 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 1 | MUL | <div style='text-align: right'>7,680</div>  |
| leaf_verifier | AccessAdapter<2> | NegE | 1 | MUL | <div style='text-align: right'>2,068</div>  |
| leaf_verifier | AccessAdapter<4> | NegE | 1 | MUL | <div style='text-align: right'>1,222</div>  |
| leaf_verifier | Boundary | NegE | 1 | MUL | <div style='text-align: right'>616</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>1,374,912</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>812,448</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>531,216</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 1 | COMP_POS2 | <div style='text-align: right'>18,876,312</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>902,044</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>535,210</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>358,530</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 1 | PERM_POS2 | <div style='text-align: right'>9,064,744</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 1 | PUBLISH | <div style='text-align: right'>972</div>  |
| leaf_verifier | Boundary | Publish | 1 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 1 | STOREW | <div style='text-align: right'>1,998,832</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 1 | STOREW | <div style='text-align: right'>37,048</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 1 | STOREW | <div style='text-align: right'>21,892</div>  |
| leaf_verifier | Boundary | StoreE | 1 | STOREW | <div style='text-align: right'>536,272</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 1 | STOREW2 | <div style='text-align: right'>2,155,944</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 1 | STOREW2 | <div style='text-align: right'>214,368</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 1 | STOREW2 | <div style='text-align: right'>126,672</div>  |
| leaf_verifier | Boundary | StoreE | 1 | STOREW2 | <div style='text-align: right'>75,768</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 1 | STOREW | <div style='text-align: right'>2,651,880</div>  |
| leaf_verifier | Boundary | StoreF | 1 | STOREW | <div style='text-align: right'>711,480</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 1 | STOREW2 | <div style='text-align: right'>4,965,264</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 1 | STOREW2 | <div style='text-align: right'>471,592</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 1 | STOREW2 | <div style='text-align: right'>280,852</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 1 | STOREW2 | <div style='text-align: right'>191,862</div>  |
| leaf_verifier | Boundary | StoreF | 1 | STOREW2 | <div style='text-align: right'>152,878</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 1 | ADD | <div style='text-align: right'>11,657,970</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 1 | SHINTW | <div style='text-align: right'>17,817,698</div>  |
| leaf_verifier | Boundary | StoreHintWord | 1 | SHINTW | <div style='text-align: right'>4,780,358</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 1 | STOREW | <div style='text-align: right'>237,800</div>  |
| leaf_verifier | Boundary | StoreV | 1 | STOREW | <div style='text-align: right'>63,800</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 1 | STOREW2 | <div style='text-align: right'>4,496,716</div>  |
| leaf_verifier | Boundary | StoreV | 1 | STOREW2 | <div style='text-align: right'>1,114,630</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 1 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> | SubE | 1 | FE4SUB | <div style='text-align: right'>229,614</div>  |
| leaf_verifier | AccessAdapter<4> | SubE | 1 | FE4SUB | <div style='text-align: right'>135,681</div>  |
| leaf_verifier | Boundary | SubE | 1 | FE4SUB | <div style='text-align: right'>27,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 1 | LOADW | <div style='text-align: right'>952,020</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 1 | LOADW | <div style='text-align: right'>84,854</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 1 | SUB | <div style='text-align: right'>232,200</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 1 | SUB | <div style='text-align: right'>84,854</div>  |
| leaf_verifier | AccessAdapter<4> | SubEF | 1 | SUB | <div style='text-align: right'>100,282</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 1 | ADD | <div style='text-align: right'>9,600</div>  |
| leaf_verifier | AccessAdapter<2> | SubEFI | 1 | ADD | <div style='text-align: right'>1,628</div>  |
| leaf_verifier | AccessAdapter<4> | SubEFI | 1 | ADD | <div style='text-align: right'>962</div>  |
| leaf_verifier | Boundary | SubEFI | 1 | ADD | <div style='text-align: right'>220</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 1 | ADD | <div style='text-align: right'>17,280</div>  |
| leaf_verifier | AccessAdapter<2> | SubEI | 1 | ADD | <div style='text-align: right'>4,510</div>  |
| leaf_verifier | AccessAdapter<4> | SubEI | 1 | ADD | <div style='text-align: right'>2,665</div>  |
| leaf_verifier | Boundary | SubEI | 1 | ADD | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 1 | SUB | <div style='text-align: right'>2,467,920</div>  |
| leaf_verifier | Boundary | SubV | 1 | SUB | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 1 | SUB | <div style='text-align: right'>139,620</div>  |
| leaf_verifier | Boundary | SubVI | 1 | SUB | <div style='text-align: right'>15,147</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 1 | SUB | <div style='text-align: right'>50,400</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 2 | JAL | <div style='text-align: right'>10</div>  |
| leaf_verifier | Boundary |  | 2 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 2 | STOREW | <div style='text-align: right'>82</div>  |
| leaf_verifier | Boundary |  | 2 | STOREW | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 2 | FE4ADD | <div style='text-align: right'>777,440</div>  |
| leaf_verifier | AccessAdapter<2> | AddE | 2 | FE4ADD | <div style='text-align: right'>339,240</div>  |
| leaf_verifier | AccessAdapter<4> | AddE | 2 | FE4ADD | <div style='text-align: right'>200,460</div>  |
| leaf_verifier | Boundary | AddE | 2 | FE4ADD | <div style='text-align: right'>119,724</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 2 | LOADW | <div style='text-align: right'>13,448</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 2 | LOADW | <div style='text-align: right'>2,277</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFFI | 2 | LOADW | <div style='text-align: right'>2,691</div>  |
| leaf_verifier | Boundary | AddEFFI | 2 | LOADW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 2 | STOREW | <div style='text-align: right'>40,344</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 2 | STOREW | <div style='text-align: right'>2,277</div>  |
| leaf_verifier | Boundary | AddEFFI | 2 | STOREW | <div style='text-align: right'>858</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 2 | ADD | <div style='text-align: right'>10,320</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFI | 2 | ADD | <div style='text-align: right'>1,540</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFI | 2 | ADD | <div style='text-align: right'>910</div>  |
| leaf_verifier | Boundary | AddEFI | 2 | ADD | <div style='text-align: right'>1,144</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 2 | ADD | <div style='text-align: right'>1,527,120</div>  |
| leaf_verifier | AccessAdapter<2> | AddEI | 2 | ADD | <div style='text-align: right'>302,126</div>  |
| leaf_verifier | AccessAdapter<4> | AddEI | 2 | ADD | <div style='text-align: right'>178,529</div>  |
| leaf_verifier | Boundary | AddEI | 2 | ADD | <div style='text-align: right'>128,788</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 2 | ADD | <div style='text-align: right'>2,184,960</div>  |
| leaf_verifier | Boundary | AddFI | 2 | ADD | <div style='text-align: right'>539</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 2 | ADD | <div style='text-align: right'>821,520</div>  |
| leaf_verifier | Boundary | AddV | 2 | ADD | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 2 | ADD | <div style='text-align: right'>17,632,170</div>  |
| leaf_verifier | Boundary | AddVI | 2 | ADD | <div style='text-align: right'>15,323</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 2 | ADD | <div style='text-align: right'>3,226,590</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 2 | LOADW | <div style='text-align: right'>4,409,673</div>  |
| leaf_verifier | Boundary | Alloc | 2 | LOADW | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 2 | MUL | <div style='text-align: right'>1,927,200</div>  |
| leaf_verifier | AccessAdapter<2> | Alloc | 2 | MUL | <div style='text-align: right'>66</div>  |
| leaf_verifier | AccessAdapter<4> | Alloc | 2 | MUL | <div style='text-align: right'>78</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 2 | BNE | <div style='text-align: right'>10,304</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqE | 2 | BNE | <div style='text-align: right'>2,464</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqE | 2 | BNE | <div style='text-align: right'>1,456</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 2 | BNE | <div style='text-align: right'>184</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqEI | 2 | BNE | <div style='text-align: right'>44</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqEI | 2 | BNE | <div style='text-align: right'>26</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 2 | BNE | <div style='text-align: right'>435,137</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 2 | BNE | <div style='text-align: right'>23</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 2 | BNE | <div style='text-align: right'>107,640</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 2 | BNE | <div style='text-align: right'>7,866</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 2 | BEQ | <div style='text-align: right'>23</div>  |
| leaf_verifier | PhantomAir | CT-ExtractPublicValuesCommit | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-InitializePcsConst | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-ReadProofsFromInput | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-VerifyProofs | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-compute-reduced-opening | 2 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-exp-reverse-bits-len | 2 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash | 2 | PHANTOM | <div style='text-align: right'>38,304</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-ext | 2 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-setup | 2 | PHANTOM | <div style='text-align: right'>1,279,152</div>  |
| leaf_verifier | PhantomAir | CT-single-reduced-opening-eval | 2 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| leaf_verifier | PhantomAir | CT-stage-c-build-rounds | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-2-fri-fold | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-3-verify-challenges | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-verify-pcs | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-e-verify-constraints | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch | 2 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-ext | 2 | PHANTOM | <div style='text-align: right'>20,160</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast | 2 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | 2 | PHANTOM | <div style='text-align: right'>58,464</div>  |
| leaf_verifier | PhantomAir | CT-verify-query | 2 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 2 | ADD | <div style='text-align: right'>30</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 2 | BBE4DIV | <div style='text-align: right'>375,200</div>  |
| leaf_verifier | AccessAdapter<2> | DivE | 2 | BBE4DIV | <div style='text-align: right'>170,060</div>  |
| leaf_verifier | AccessAdapter<4> | DivE | 2 | BBE4DIV | <div style='text-align: right'>100,490</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>2,880</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>3,014</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>1,781</div>  |
| leaf_verifier | Boundary | DivEIN | 2 | BBE4DIV | <div style='text-align: right'>440</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 2 | STOREW | <div style='text-align: right'>11,808</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 2 | STOREW | <div style='text-align: right'>1,034</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 2 | STOREW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 2 | DIV | <div style='text-align: right'>5,160</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 2 | ADD | <div style='text-align: right'>21,416,670</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 2 | BNE | <div style='text-align: right'>18,231,272</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | For | 2 | JAL | <div style='text-align: right'>787,750</div>  |
| leaf_verifier | AccessAdapter<2> | For | 2 | JAL | <div style='text-align: right'>550</div>  |
| leaf_verifier | AccessAdapter<4> | For | 2 | JAL | <div style='text-align: right'>650</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 2 | LOADW | <div style='text-align: right'>203,196</div>  |
| leaf_verifier | Boundary | For | 2 | LOADW | <div style='text-align: right'>473</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 2 | STOREW | <div style='text-align: right'>3,026,579</div>  |
| leaf_verifier | Boundary | For | 2 | STOREW | <div style='text-align: right'>627</div>  |
| leaf_verifier | AccessAdapter<2> | FriReducedOpening | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>214,456</div>  |
| leaf_verifier | AccessAdapter<4> | FriReducedOpening | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>126,724</div>  |
| leaf_verifier | FriReducedOpeningAir | FriReducedOpening | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,106,688</div>  |
| leaf_verifier | PhantomAir | HintBitsF | 2 | PHANTOM | <div style='text-align: right'>516</div>  |
| leaf_verifier | PhantomAir | HintInputVec | 2 | PHANTOM | <div style='text-align: right'>259,878</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 2 | BNE | <div style='text-align: right'>773,490</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 2 | BNE | <div style='text-align: right'>5,285,147</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 2 | JAL | <div style='text-align: right'>483,950</div>  |
| leaf_verifier | AccessAdapter<2> | IfEqI | 2 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | AccessAdapter<4> | IfEqI | 2 | JAL | <div style='text-align: right'>13</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 2 | BEQ | <div style='text-align: right'>725,282</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | 2 | JAL | <div style='text-align: right'>40</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 2 | BEQ | <div style='text-align: right'>115,322</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 2 | STOREW | <div style='text-align: right'>207,296</div>  |
| leaf_verifier | AccessAdapter<2> | ImmE | 2 | STOREW | <div style='text-align: right'>11,880</div>  |
| leaf_verifier | AccessAdapter<4> | ImmE | 2 | STOREW | <div style='text-align: right'>7,020</div>  |
| leaf_verifier | Boundary | ImmE | 2 | STOREW | <div style='text-align: right'>12,232</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 2 | STOREW | <div style='text-align: right'>3,057,780</div>  |
| leaf_verifier | Boundary | ImmF | 2 | STOREW | <div style='text-align: right'>1,573</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 2 | STOREW | <div style='text-align: right'>2,229,211</div>  |
| leaf_verifier | Boundary | ImmV | 2 | STOREW | <div style='text-align: right'>16,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 2 | LOADW | <div style='text-align: right'>1,636,392</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 2 | LOADW | <div style='text-align: right'>264,528</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 2 | LOADW | <div style='text-align: right'>156,312</div>  |
| leaf_verifier | Boundary | LoadE | 2 | LOADW | <div style='text-align: right'>3,388</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 2 | LOADW2 | <div style='text-align: right'>4,234,152</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 2 | LOADW2 | <div style='text-align: right'>112,838</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 2 | LOADW2 | <div style='text-align: right'>66,677</div>  |
| leaf_verifier | Boundary | LoadE | 2 | LOADW2 | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 2 | LOADW | <div style='text-align: right'>2,185,218</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 2 | LOADW | <div style='text-align: right'>103,488</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 2 | LOADW | <div style='text-align: right'>61,152</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| leaf_verifier | Boundary | LoadF | 2 | LOADW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 2 | LOADW2 | <div style='text-align: right'>5,464,644</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 2 | LOADW2 | <div style='text-align: right'>1,584</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 2 | LOADW2 | <div style='text-align: right'>936</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 2 | LOADW2 | <div style='text-align: right'>986</div>  |
| leaf_verifier | Boundary | LoadF | 2 | LOADW2 | <div style='text-align: right'>550</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 2 | LOADW | <div style='text-align: right'>2,125,276</div>  |
| leaf_verifier | Boundary | LoadV | 2 | LOADW | <div style='text-align: right'>15,246</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 2 | LOADW2 | <div style='text-align: right'>12,956,164</div>  |
| leaf_verifier | Boundary | LoadV | 2 | LOADW2 | <div style='text-align: right'>583</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 2 | BBE4MUL | <div style='text-align: right'>668,720</div>  |
| leaf_verifier | AccessAdapter<2> | MulE | 2 | BBE4MUL | <div style='text-align: right'>322,806</div>  |
| leaf_verifier | AccessAdapter<4> | MulE | 2 | BBE4MUL | <div style='text-align: right'>190,749</div>  |
| leaf_verifier | Boundary | MulE | 2 | BBE4MUL | <div style='text-align: right'>138,732</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 2 | MUL | <div style='text-align: right'>218,880</div>  |
| leaf_verifier | AccessAdapter<2> | MulEF | 2 | MUL | <div style='text-align: right'>37,818</div>  |
| leaf_verifier | AccessAdapter<4> | MulEF | 2 | MUL | <div style='text-align: right'>22,347</div>  |
| leaf_verifier | Boundary | MulEF | 2 | MUL | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 2 | MUL | <div style='text-align: right'>13,200</div>  |
| leaf_verifier | AccessAdapter<2> | MulEFI | 2 | MUL | <div style='text-align: right'>2,332</div>  |
| leaf_verifier | AccessAdapter<4> | MulEFI | 2 | MUL | <div style='text-align: right'>1,378</div>  |
| leaf_verifier | Boundary | MulEFI | 2 | MUL | <div style='text-align: right'>1,188</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 2 | BBE4MUL | <div style='text-align: right'>81,600</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 2 | BBE4MUL | <div style='text-align: right'>93,566</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 2 | BBE4MUL | <div style='text-align: right'>55,289</div>  |
| leaf_verifier | Boundary | MulEI | 2 | BBE4MUL | <div style='text-align: right'>6,204</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 2 | STOREW | <div style='text-align: right'>334,560</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 2 | STOREW | <div style='text-align: right'>44,781</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 2 | STOREW | <div style='text-align: right'>26,416</div>  |
| leaf_verifier | Boundary | MulEI | 2 | STOREW | <div style='text-align: right'>33</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 2 | MUL | <div style='text-align: right'>4,109,880</div>  |
| leaf_verifier | Boundary | MulF | 2 | MUL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 2 | MUL | <div style='text-align: right'>840</div>  |
| leaf_verifier | Boundary | MulFI | 2 | MUL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | 2 | MUL | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | Boundary | MulV | 2 | MUL | <div style='text-align: right'>14,641</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 2 | MUL | <div style='text-align: right'>1,140,360</div>  |
| leaf_verifier | Boundary | MulVI | 2 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 2 | MUL | <div style='text-align: right'>7,680</div>  |
| leaf_verifier | AccessAdapter<2> | NegE | 2 | MUL | <div style='text-align: right'>2,068</div>  |
| leaf_verifier | AccessAdapter<4> | NegE | 2 | MUL | <div style='text-align: right'>1,222</div>  |
| leaf_verifier | Boundary | NegE | 2 | MUL | <div style='text-align: right'>616</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>1,374,912</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>812,448</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>531,216</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 2 | COMP_POS2 | <div style='text-align: right'>18,876,312</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>902,044</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>535,210</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>358,530</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 2 | PERM_POS2 | <div style='text-align: right'>9,064,744</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 2 | PUBLISH | <div style='text-align: right'>972</div>  |
| leaf_verifier | Boundary | Publish | 2 | PUBLISH | <div style='text-align: right'>88</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 2 | STOREW | <div style='text-align: right'>1,998,832</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 2 | STOREW | <div style='text-align: right'>37,048</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 2 | STOREW | <div style='text-align: right'>21,892</div>  |
| leaf_verifier | Boundary | StoreE | 2 | STOREW | <div style='text-align: right'>536,272</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 2 | STOREW2 | <div style='text-align: right'>2,155,944</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 2 | STOREW2 | <div style='text-align: right'>214,368</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 2 | STOREW2 | <div style='text-align: right'>126,672</div>  |
| leaf_verifier | Boundary | StoreE | 2 | STOREW2 | <div style='text-align: right'>75,768</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 2 | STOREW | <div style='text-align: right'>2,651,880</div>  |
| leaf_verifier | Boundary | StoreF | 2 | STOREW | <div style='text-align: right'>711,480</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 2 | STOREW2 | <div style='text-align: right'>4,965,264</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 2 | STOREW2 | <div style='text-align: right'>471,592</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 2 | STOREW2 | <div style='text-align: right'>280,852</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 2 | STOREW2 | <div style='text-align: right'>191,862</div>  |
| leaf_verifier | Boundary | StoreF | 2 | STOREW2 | <div style='text-align: right'>152,878</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 2 | ADD | <div style='text-align: right'>11,657,970</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 2 | SHINTW | <div style='text-align: right'>17,817,698</div>  |
| leaf_verifier | Boundary | StoreHintWord | 2 | SHINTW | <div style='text-align: right'>4,780,358</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 2 | STOREW | <div style='text-align: right'>237,800</div>  |
| leaf_verifier | Boundary | StoreV | 2 | STOREW | <div style='text-align: right'>63,800</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 2 | STOREW2 | <div style='text-align: right'>4,496,716</div>  |
| leaf_verifier | Boundary | StoreV | 2 | STOREW2 | <div style='text-align: right'>1,114,630</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 2 | FE4SUB | <div style='text-align: right'>263,760</div>  |
| leaf_verifier | AccessAdapter<2> | SubE | 2 | FE4SUB | <div style='text-align: right'>229,614</div>  |
| leaf_verifier | AccessAdapter<4> | SubE | 2 | FE4SUB | <div style='text-align: right'>135,681</div>  |
| leaf_verifier | Boundary | SubE | 2 | FE4SUB | <div style='text-align: right'>27,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 2 | LOADW | <div style='text-align: right'>952,020</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 2 | LOADW | <div style='text-align: right'>84,854</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 2 | SUB | <div style='text-align: right'>232,200</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 2 | SUB | <div style='text-align: right'>84,854</div>  |
| leaf_verifier | AccessAdapter<4> | SubEF | 2 | SUB | <div style='text-align: right'>100,282</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 2 | ADD | <div style='text-align: right'>9,600</div>  |
| leaf_verifier | AccessAdapter<2> | SubEFI | 2 | ADD | <div style='text-align: right'>1,628</div>  |
| leaf_verifier | AccessAdapter<4> | SubEFI | 2 | ADD | <div style='text-align: right'>962</div>  |
| leaf_verifier | Boundary | SubEFI | 2 | ADD | <div style='text-align: right'>220</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 2 | ADD | <div style='text-align: right'>17,280</div>  |
| leaf_verifier | AccessAdapter<2> | SubEI | 2 | ADD | <div style='text-align: right'>4,510</div>  |
| leaf_verifier | AccessAdapter<4> | SubEI | 2 | ADD | <div style='text-align: right'>2,665</div>  |
| leaf_verifier | Boundary | SubEI | 2 | ADD | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 2 | SUB | <div style='text-align: right'>2,467,920</div>  |
| leaf_verifier | Boundary | SubV | 2 | SUB | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 2 | SUB | <div style='text-align: right'>139,620</div>  |
| leaf_verifier | Boundary | SubVI | 2 | SUB | <div style='text-align: right'>15,147</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 2 | SUB | <div style='text-align: right'>50,400</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> |  | 3 | JAL | <div style='text-align: right'>10</div>  |
| leaf_verifier | Boundary |  | 3 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 3 | STOREW | <div style='text-align: right'>82</div>  |
| leaf_verifier | Boundary |  | 3 | STOREW | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 3 | FE4ADD | <div style='text-align: right'>818,840</div>  |
| leaf_verifier | AccessAdapter<2> | AddE | 3 | FE4ADD | <div style='text-align: right'>352,968</div>  |
| leaf_verifier | AccessAdapter<4> | AddE | 3 | FE4ADD | <div style='text-align: right'>208,572</div>  |
| leaf_verifier | Boundary | AddE | 3 | FE4ADD | <div style='text-align: right'>119,724</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 3 | LOADW | <div style='text-align: right'>13,448</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 3 | LOADW | <div style='text-align: right'>2,255</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFFI | 3 | LOADW | <div style='text-align: right'>2,665</div>  |
| leaf_verifier | Boundary | AddEFFI | 3 | LOADW | <div style='text-align: right'>286</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 3 | STOREW | <div style='text-align: right'>40,344</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFFI | 3 | STOREW | <div style='text-align: right'>2,255</div>  |
| leaf_verifier | Boundary | AddEFFI | 3 | STOREW | <div style='text-align: right'>858</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 3 | ADD | <div style='text-align: right'>12,000</div>  |
| leaf_verifier | AccessAdapter<2> | AddEFI | 3 | ADD | <div style='text-align: right'>1,870</div>  |
| leaf_verifier | AccessAdapter<4> | AddEFI | 3 | ADD | <div style='text-align: right'>1,105</div>  |
| leaf_verifier | Boundary | AddEFI | 3 | ADD | <div style='text-align: right'>1,144</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 3 | ADD | <div style='text-align: right'>1,559,640</div>  |
| leaf_verifier | AccessAdapter<2> | AddEI | 3 | ADD | <div style='text-align: right'>315,964</div>  |
| leaf_verifier | AccessAdapter<4> | AddEI | 3 | ADD | <div style='text-align: right'>186,706</div>  |
| leaf_verifier | Boundary | AddEI | 3 | ADD | <div style='text-align: right'>128,788</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 3 | ADD | <div style='text-align: right'>2,206,410</div>  |
| leaf_verifier | Boundary | AddFI | 3 | ADD | <div style='text-align: right'>627</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 3 | ADD | <div style='text-align: right'>804,180</div>  |
| leaf_verifier | Boundary | AddV | 3 | ADD | <div style='text-align: right'>22</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 3 | ADD | <div style='text-align: right'>17,990,100</div>  |
| leaf_verifier | Boundary | AddVI | 3 | ADD | <div style='text-align: right'>15,323</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 3 | ADD | <div style='text-align: right'>3,197,850</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 3 | LOADW | <div style='text-align: right'>4,370,395</div>  |
| leaf_verifier | Boundary | Alloc | 3 | LOADW | <div style='text-align: right'>1,056</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 3 | MUL | <div style='text-align: right'>1,908,300</div>  |
| leaf_verifier | AccessAdapter<2> | Alloc | 3 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | AccessAdapter<4> | Alloc | 3 | MUL | <div style='text-align: right'>91</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 3 | BNE | <div style='text-align: right'>10,488</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqE | 3 | BNE | <div style='text-align: right'>2,508</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqE | 3 | BNE | <div style='text-align: right'>1,482</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 3 | BNE | <div style='text-align: right'>184</div>  |
| leaf_verifier | AccessAdapter<2> | AssertEqEI | 3 | BNE | <div style='text-align: right'>44</div>  |
| leaf_verifier | AccessAdapter<4> | AssertEqEI | 3 | BNE | <div style='text-align: right'>26</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 3 | BNE | <div style='text-align: right'>427,593</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 3 | BNE | <div style='text-align: right'>23</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 3 | BNE | <div style='text-align: right'>107,180</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 3 | BNE | <div style='text-align: right'>8,395</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 3 | BEQ | <div style='text-align: right'>23</div>  |
| leaf_verifier | PhantomAir | CT-ExtractPublicValuesCommit | 3 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-InitializePcsConst | 3 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-ReadProofsFromInput | 3 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-VerifyProofs | 3 | PHANTOM | <div style='text-align: right'>12</div>  |
| leaf_verifier | PhantomAir | CT-compute-reduced-opening | 3 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-exp-reverse-bits-len | 3 | PHANTOM | <div style='text-align: right'>62,496</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash | 3 | PHANTOM | <div style='text-align: right'>38,304</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-ext | 3 | PHANTOM | <div style='text-align: right'>19,656</div>  |
| leaf_verifier | PhantomAir | CT-poseidon2-hash-setup | 3 | PHANTOM | <div style='text-align: right'>1,375,920</div>  |
| leaf_verifier | PhantomAir | CT-single-reduced-opening-eval | 3 | PHANTOM | <div style='text-align: right'>97,776</div>  |
| leaf_verifier | PhantomAir | CT-stage-c-build-rounds | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-2-fri-fold | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-3-verify-challenges | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-d-verify-pcs | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-stage-e-verify-constraints | 3 | PHANTOM | <div style='text-align: right'>24</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch | 3 | PHANTOM | <div style='text-align: right'>8,064</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-ext | 3 | PHANTOM | <div style='text-align: right'>19,656</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast | 3 | PHANTOM | <div style='text-align: right'>57,960</div>  |
| leaf_verifier | PhantomAir | CT-verify-batch-reduce-fast-setup | 3 | PHANTOM | <div style='text-align: right'>57,960</div>  |
| leaf_verifier | PhantomAir | CT-verify-query | 3 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 3 | ADD | <div style='text-align: right'>30</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 3 | BBE4DIV | <div style='text-align: right'>393,840</div>  |
| leaf_verifier | AccessAdapter<2> | DivE | 3 | BBE4DIV | <div style='text-align: right'>181,280</div>  |
| leaf_verifier | AccessAdapter<4> | DivE | 3 | BBE4DIV | <div style='text-align: right'>107,120</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>3,120</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>3,190</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>1,885</div>  |
| leaf_verifier | Boundary | DivEIN | 3 | BBE4DIV | <div style='text-align: right'>440</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 3 | STOREW | <div style='text-align: right'>12,792</div>  |
| leaf_verifier | AccessAdapter<2> | DivEIN | 3 | STOREW | <div style='text-align: right'>1,122</div>  |
| leaf_verifier | AccessAdapter<4> | DivEIN | 3 | STOREW | <div style='text-align: right'>312</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 3 | DIV | <div style='text-align: right'>5,580</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 3 | ADD | <div style='text-align: right'>21,776,010</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 3 | BNE | <div style='text-align: right'>18,517,415</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | For | 3 | JAL | <div style='text-align: right'>792,380</div>  |
| leaf_verifier | AccessAdapter<2> | For | 3 | JAL | <div style='text-align: right'>594</div>  |
| leaf_verifier | AccessAdapter<4> | For | 3 | JAL | <div style='text-align: right'>702</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 3 | LOADW | <div style='text-align: right'>201,474</div>  |
| leaf_verifier | Boundary | For | 3 | LOADW | <div style='text-align: right'>473</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 3 | STOREW | <div style='text-align: right'>3,047,284</div>  |
| leaf_verifier | Boundary | For | 3 | STOREW | <div style='text-align: right'>627</div>  |
| leaf_verifier | AccessAdapter<2> | FriReducedOpening | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>229,944</div>  |
| leaf_verifier | AccessAdapter<4> | FriReducedOpening | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>135,876</div>  |
| leaf_verifier | FriReducedOpeningAir | FriReducedOpening | 3 | FRI_REDUCED_OPENING | <div style='text-align: right'>14,095,872</div>  |
| leaf_verifier | PhantomAir | HintBitsF | 3 | PHANTOM | <div style='text-align: right'>516</div>  |
| leaf_verifier | PhantomAir | HintInputVec | 3 | PHANTOM | <div style='text-align: right'>257,910</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 3 | BNE | <div style='text-align: right'>841,018</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 3 | BNE | <div style='text-align: right'>5,453,369</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 3 | JAL | <div style='text-align: right'>483,420</div>  |
| leaf_verifier | AccessAdapter<2> | IfEqI | 3 | JAL | <div style='text-align: right'>11</div>  |
| leaf_verifier | AccessAdapter<4> | IfEqI | 3 | JAL | <div style='text-align: right'>13</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 3 | BEQ | <div style='text-align: right'>702,006</div>  |
| leaf_verifier | <JalNativeAdapterAir,JalCoreAir> | IfNe | 3 | JAL | <div style='text-align: right'>40</div>  |
| leaf_verifier | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 3 | BEQ | <div style='text-align: right'>114,586</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 3 | STOREW | <div style='text-align: right'>223,040</div>  |
| leaf_verifier | AccessAdapter<2> | ImmE | 3 | STOREW | <div style='text-align: right'>13,574</div>  |
| leaf_verifier | AccessAdapter<4> | ImmE | 3 | STOREW | <div style='text-align: right'>8,021</div>  |
| leaf_verifier | Boundary | ImmE | 3 | STOREW | <div style='text-align: right'>12,232</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 3 | STOREW | <div style='text-align: right'>3,071,720</div>  |
| leaf_verifier | Boundary | ImmF | 3 | STOREW | <div style='text-align: right'>1,573</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 3 | STOREW | <div style='text-align: right'>2,255,164</div>  |
| leaf_verifier | Boundary | ImmV | 3 | STOREW | <div style='text-align: right'>16,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 3 | LOADW | <div style='text-align: right'>1,676,736</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 3 | LOADW | <div style='text-align: right'>277,002</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 3 | LOADW | <div style='text-align: right'>163,683</div>  |
| leaf_verifier | Boundary | LoadE | 3 | LOADW | <div style='text-align: right'>3,388</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 3 | LOADW2 | <div style='text-align: right'>4,392,904</div>  |
| leaf_verifier | AccessAdapter<2> | LoadE | 3 | LOADW2 | <div style='text-align: right'>110,066</div>  |
| leaf_verifier | AccessAdapter<4> | LoadE | 3 | LOADW2 | <div style='text-align: right'>65,039</div>  |
| leaf_verifier | Boundary | LoadE | 3 | LOADW2 | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 3 | LOADW | <div style='text-align: right'>2,194,320</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 3 | LOADW | <div style='text-align: right'>103,972</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 3 | LOADW | <div style='text-align: right'>61,438</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 3 | LOADW | <div style='text-align: right'>40,171</div>  |
| leaf_verifier | Boundary | LoadF | 3 | LOADW | <div style='text-align: right'>7,964</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 3 | LOADW2 | <div style='text-align: right'>5,865,624</div>  |
| leaf_verifier | AccessAdapter<2> | LoadF | 3 | LOADW2 | <div style='text-align: right'>1,562</div>  |
| leaf_verifier | AccessAdapter<4> | LoadF | 3 | LOADW2 | <div style='text-align: right'>923</div>  |
| leaf_verifier | AccessAdapter<8> | LoadF | 3 | LOADW2 | <div style='text-align: right'>969</div>  |
| leaf_verifier | Boundary | LoadF | 3 | LOADW2 | <div style='text-align: right'>550</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 3 | LOADW | <div style='text-align: right'>2,110,147</div>  |
| leaf_verifier | Boundary | LoadV | 3 | LOADW | <div style='text-align: right'>15,246</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 3 | LOADW2 | <div style='text-align: right'>13,335,332</div>  |
| leaf_verifier | Boundary | LoadV | 3 | LOADW2 | <div style='text-align: right'>583</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 3 | BBE4MUL | <div style='text-align: right'>681,760</div>  |
| leaf_verifier | AccessAdapter<2> | MulE | 3 | BBE4MUL | <div style='text-align: right'>332,024</div>  |
| leaf_verifier | AccessAdapter<4> | MulE | 3 | BBE4MUL | <div style='text-align: right'>196,196</div>  |
| leaf_verifier | Boundary | MulE | 3 | BBE4MUL | <div style='text-align: right'>138,732</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 3 | MUL | <div style='text-align: right'>215,280</div>  |
| leaf_verifier | AccessAdapter<2> | MulEF | 3 | MUL | <div style='text-align: right'>37,048</div>  |
| leaf_verifier | AccessAdapter<4> | MulEF | 3 | MUL | <div style='text-align: right'>21,892</div>  |
| leaf_verifier | Boundary | MulEF | 3 | MUL | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 3 | MUL | <div style='text-align: right'>17,040</div>  |
| leaf_verifier | AccessAdapter<2> | MulEFI | 3 | MUL | <div style='text-align: right'>3,080</div>  |
| leaf_verifier | AccessAdapter<4> | MulEFI | 3 | MUL | <div style='text-align: right'>1,820</div>  |
| leaf_verifier | Boundary | MulEFI | 3 | MUL | <div style='text-align: right'>1,188</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 3 | BBE4MUL | <div style='text-align: right'>90,960</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 3 | BBE4MUL | <div style='text-align: right'>103,818</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 3 | BBE4MUL | <div style='text-align: right'>61,347</div>  |
| leaf_verifier | Boundary | MulEI | 3 | BBE4MUL | <div style='text-align: right'>6,204</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 3 | STOREW | <div style='text-align: right'>372,936</div>  |
| leaf_verifier | AccessAdapter<2> | MulEI | 3 | STOREW | <div style='text-align: right'>49,929</div>  |
| leaf_verifier | AccessAdapter<4> | MulEI | 3 | STOREW | <div style='text-align: right'>29,458</div>  |
| leaf_verifier | Boundary | MulEI | 3 | STOREW | <div style='text-align: right'>33</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 3 | MUL | <div style='text-align: right'>4,140,720</div>  |
| leaf_verifier | Boundary | MulF | 3 | MUL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 3 | MUL | <div style='text-align: right'>900</div>  |
| leaf_verifier | Boundary | MulFI | 3 | MUL | <div style='text-align: right'>11</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | 3 | MUL | <div style='text-align: right'>79,980</div>  |
| leaf_verifier | Boundary | MulV | 3 | MUL | <div style='text-align: right'>14,641</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 3 | MUL | <div style='text-align: right'>1,119,360</div>  |
| leaf_verifier | Boundary | MulVI | 3 | MUL | <div style='text-align: right'>77</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 3 | MUL | <div style='text-align: right'>8,520</div>  |
| leaf_verifier | AccessAdapter<2> | NegE | 3 | MUL | <div style='text-align: right'>2,310</div>  |
| leaf_verifier | AccessAdapter<4> | NegE | 3 | MUL | <div style='text-align: right'>1,365</div>  |
| leaf_verifier | Boundary | NegE | 3 | MUL | <div style='text-align: right'>616</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>1,337,556</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>790,374</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>516,783</div>  |
| leaf_verifier | Boundary | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>88</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 3 | COMP_POS2 | <div style='text-align: right'>18,342,467</div>  |
| leaf_verifier | AccessAdapter<2> | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>943,162</div>  |
| leaf_verifier | AccessAdapter<4> | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>559,780</div>  |
| leaf_verifier | AccessAdapter<8> | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>373,524</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 3 | PERM_POS2 | <div style='text-align: right'>9,581,260</div>  |
| leaf_verifier | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 3 | PUBLISH | <div style='text-align: right'>972</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 3 | STOREW | <div style='text-align: right'>1,972,264</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 3 | STOREW | <div style='text-align: right'>36,124</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 3 | STOREW | <div style='text-align: right'>21,346</div>  |
| leaf_verifier | Boundary | StoreE | 3 | STOREW | <div style='text-align: right'>529,144</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 3 | STOREW2 | <div style='text-align: right'>2,252,212</div>  |
| leaf_verifier | AccessAdapter<2> | StoreE | 3 | STOREW2 | <div style='text-align: right'>229,152</div>  |
| leaf_verifier | AccessAdapter<4> | StoreE | 3 | STOREW2 | <div style='text-align: right'>135,408</div>  |
| leaf_verifier | Boundary | StoreE | 3 | STOREW2 | <div style='text-align: right'>73,876</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 3 | STOREW | <div style='text-align: right'>2,672,872</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 3 | STOREW | <div style='text-align: right'>4,576</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 3 | STOREW | <div style='text-align: right'>2,704</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 3 | STOREW | <div style='text-align: right'>1,768</div>  |
| leaf_verifier | Boundary | StoreF | 3 | STOREW | <div style='text-align: right'>707,960</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 3 | STOREW2 | <div style='text-align: right'>5,300,808</div>  |
| leaf_verifier | AccessAdapter<2> | StoreF | 3 | STOREW2 | <div style='text-align: right'>516,428</div>  |
| leaf_verifier | AccessAdapter<4> | StoreF | 3 | STOREW2 | <div style='text-align: right'>307,619</div>  |
| leaf_verifier | AccessAdapter<8> | StoreF | 3 | STOREW2 | <div style='text-align: right'>208,301</div>  |
| leaf_verifier | Boundary | StoreF | 3 | STOREW2 | <div style='text-align: right'>154,198</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 3 | ADD | <div style='text-align: right'>11,708,280</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 3 | SHINTW | <div style='text-align: right'>17,873,007</div>  |
| leaf_verifier | Boundary | StoreHintWord | 3 | SHINTW | <div style='text-align: right'>4,795,197</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 3 | STOREW | <div style='text-align: right'>233,864</div>  |
| leaf_verifier | Boundary | StoreV | 3 | STOREW | <div style='text-align: right'>62,744</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 3 | STOREW2 | <div style='text-align: right'>4,511,558</div>  |
| leaf_verifier | Boundary | StoreV | 3 | STOREW2 | <div style='text-align: right'>1,110,758</div>  |
| leaf_verifier | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 3 | FE4SUB | <div style='text-align: right'>261,640</div>  |
| leaf_verifier | AccessAdapter<2> | SubE | 3 | FE4SUB | <div style='text-align: right'>226,270</div>  |
| leaf_verifier | AccessAdapter<4> | SubE | 3 | FE4SUB | <div style='text-align: right'>133,705</div>  |
| leaf_verifier | Boundary | SubE | 3 | FE4SUB | <div style='text-align: right'>27,368</div>  |
| leaf_verifier | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 3 | LOADW | <div style='text-align: right'>1,014,504</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 3 | LOADW | <div style='text-align: right'>90,442</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 3 | SUB | <div style='text-align: right'>247,440</div>  |
| leaf_verifier | AccessAdapter<2> | SubEF | 3 | SUB | <div style='text-align: right'>90,442</div>  |
| leaf_verifier | AccessAdapter<4> | SubEF | 3 | SUB | <div style='text-align: right'>106,886</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 3 | ADD | <div style='text-align: right'>11,400</div>  |
| leaf_verifier | AccessAdapter<2> | SubEFI | 3 | ADD | <div style='text-align: right'>1,936</div>  |
| leaf_verifier | AccessAdapter<4> | SubEFI | 3 | ADD | <div style='text-align: right'>1,144</div>  |
| leaf_verifier | Boundary | SubEFI | 3 | ADD | <div style='text-align: right'>220</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 3 | ADD | <div style='text-align: right'>18,720</div>  |
| leaf_verifier | AccessAdapter<2> | SubEI | 3 | ADD | <div style='text-align: right'>4,840</div>  |
| leaf_verifier | AccessAdapter<4> | SubEI | 3 | ADD | <div style='text-align: right'>2,860</div>  |
| leaf_verifier | Boundary | SubEI | 3 | ADD | <div style='text-align: right'>880</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 3 | SUB | <div style='text-align: right'>2,494,440</div>  |
| leaf_verifier | Boundary | SubV | 3 | SUB | <div style='text-align: right'>44</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 3 | SUB | <div style='text-align: right'>138,300</div>  |
| leaf_verifier | Boundary | SubVI | 3 | SUB | <div style='text-align: right'>15,147</div>  |
| leaf_verifier | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 3 | SUB | <div style='text-align: right'>49,140</div>  |

| group | air_name | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| leaf_verifier | ProgramAir | 0 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>2,752</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | PhantomAir | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramAir | 1 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 0 | <div style='text-align: right'>2,752</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | PhantomAir | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 1 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VariableRangeCheckerAir | 1 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramAir | 2 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 2 | 0 | <div style='text-align: right'>2,752</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | PhantomAir | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 2 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 2 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 2 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VariableRangeCheckerAir | 2 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | ProgramAir | 3 | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmConnectorAir | 3 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 3 | 0 | <div style='text-align: right'>2,752</div>  | <div style='text-align: right'>27</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| leaf_verifier | VolatileBoundaryAir | 3 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<2> | 3 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| leaf_verifier | AccessAdapterAir<4> | 3 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | AccessAdapterAir<8> | 3 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | PhantomAir | 3 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| leaf_verifier | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 3 | 0 | <div style='text-align: right'>136,314,880</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 3 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| leaf_verifier | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 3 | 0 | <div style='text-align: right'>2,883,584</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>131,072</div>  |
| leaf_verifier | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 3 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| leaf_verifier | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 3 | 0 | <div style='text-align: right'>3,932,160</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | FriReducedOpeningAir | 3 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_verifier | Poseidon2VmAir<BabyBearParameters> | 3 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| leaf_verifier | VariableRangeCheckerAir | 3 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- |
| leaf_verifier | 0 | 0 | <div style='text-align: right'>1,480.0</div>  | <div style='text-align: right'>22,257.0</div>  | <div style='text-align: right'>617,024,216</div>  |
| leaf_verifier | 1 | 0 | <div style='text-align: right'>1,428.0</div>  | <div style='text-align: right'>22,219.0</div>  | <div style='text-align: right'>617,024,216</div>  |
| leaf_verifier | 2 | 0 | <div style='text-align: right'>1,417.0</div>  | <div style='text-align: right'>22,247.0</div>  | <div style='text-align: right'>614,140,632</div>  |
| leaf_verifier | 3 | 0 | <div style='text-align: right'>1,395.0</div>  | <div style='text-align: right'>22,195.0</div>  | <div style='text-align: right'>614,140,632</div>  |

| group | height | index | execute_time_ms | fri.log_blowup | total_cells_used | total_cycles |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | <div style='text-align: right'>8,212.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>286,073,329</div>  | <div style='text-align: right'>7,254,693</div>  |
| internal_verifier_height_0 | 0 | 1 | <div style='text-align: right'>8,491.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>285,996,275</div>  | <div style='text-align: right'>7,251,876</div>  |
| internal_verifier_height_1 | 1 | 2 | <div style='text-align: right'>8,451.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>286,727,063</div>  | <div style='text-align: right'>7,270,241</div>  |

| group | chip_name | height | index | rows_used |
| --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramChip | 0 | 0 | <div style='text-align: right'>155,025</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 0 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 0 | <div style='text-align: right'>770,716</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 0 | <div style='text-align: right'>751,304</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 0 | <div style='text-align: right'>375,988</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 0 | <div style='text-align: right'>108,720</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | <div style='text-align: right'>353,989</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | 0 | <div style='text-align: right'>2,227,925</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 0 | <div style='text-align: right'>1,387,409</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 0 | <div style='text-align: right'>180,079</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 0 | <div style='text-align: right'>2,964,901</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 0 | <div style='text-align: right'>77,748</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | <div style='text-align: right'>218,736</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | <div style='text-align: right'>52,006</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | ProgramChip | 0 | 1 | <div style='text-align: right'>155,025</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 0 | 1 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | Boundary | 0 | 1 | <div style='text-align: right'>771,388</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | 0 | 1 | <div style='text-align: right'>751,324</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | 0 | 1 | <div style='text-align: right'>375,998</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | 0 | 1 | <div style='text-align: right'>108,720</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | <div style='text-align: right'>353,989</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 0 | 1 | <div style='text-align: right'>2,227,841</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | 1 | <div style='text-align: right'>1,387,355</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | 0 | 1 | <div style='text-align: right'>180,041</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | 1 | <div style='text-align: right'>2,962,310</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | 1 | <div style='text-align: right'>77,698</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | <div style='text-align: right'>218,736</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | <div style='text-align: right'>52,006</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | ProgramChip | 1 | 2 | <div style='text-align: right'>155,025</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | 1 | 2 | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 | Boundary | 1 | 2 | <div style='text-align: right'>771,964</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | 1 | 2 | <div style='text-align: right'>752,848</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | 1 | 2 | <div style='text-align: right'>376,676</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | 1 | 2 | <div style='text-align: right'>109,232</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | <div style='text-align: right'>354,241</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | 1 | 2 | <div style='text-align: right'>2,229,909</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 1 | 2 | <div style='text-align: right'>1,389,163</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | 1 | 2 | <div style='text-align: right'>180,753</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 1 | 2 | <div style='text-align: right'>2,975,255</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 1 | 2 | <div style='text-align: right'>78,022</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | <div style='text-align: right'>218,904</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | <div style='text-align: right'>52,262</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | <div style='text-align: right'>131,072</div>  |

| group | dsl_ir | height | index | opcode | frequency |
| --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 |  | 0 | 0 | JAL | <div style='text-align: right'>1</div>  |
| internal_verifier_height_0 |  | 0 | 0 | STOREW | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>24,230</div>  |
| internal_verifier_height_0 | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>688</div>  |
| internal_verifier_height_0 | AddEFFI | 0 | 0 | STOREW | <div style='text-align: right'>2,064</div>  |
| internal_verifier_height_0 | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>664</div>  |
| internal_verifier_height_0 | AddEI | 0 | 0 | ADD | <div style='text-align: right'>65,016</div>  |
| internal_verifier_height_0 | AddFI | 0 | 0 | ADD | <div style='text-align: right'>151,064</div>  |
| internal_verifier_height_0 | AddV | 0 | 0 | ADD | <div style='text-align: right'>34,466</div>  |
| internal_verifier_height_0 | AddVI | 0 | 0 | ADD | <div style='text-align: right'>715,622</div>  |
| internal_verifier_height_0 | Alloc | 0 | 0 | ADD | <div style='text-align: right'>116,417</div>  |
| internal_verifier_height_0 | Alloc | 0 | 0 | LOADW | <div style='text-align: right'>116,417</div>  |
| internal_verifier_height_0 | Alloc | 0 | 0 | MUL | <div style='text-align: right'>68,640</div>  |
| internal_verifier_height_0 | AssertEqE | 0 | 0 | BNE | <div style='text-align: right'>464</div>  |
| internal_verifier_height_0 | AssertEqEI | 0 | 0 | BNE | <div style='text-align: right'>8</div>  |
| internal_verifier_height_0 | AssertEqF | 0 | 0 | BNE | <div style='text-align: right'>18,927</div>  |
| internal_verifier_height_0 | AssertEqFI | 0 | 0 | BNE | <div style='text-align: right'>7</div>  |
| internal_verifier_height_0 | AssertEqV | 0 | 0 | BNE | <div style='text-align: right'>4,938</div>  |
| internal_verifier_height_0 | AssertEqVI | 0 | 0 | BNE | <div style='text-align: right'>440</div>  |
| internal_verifier_height_0 | AssertNeVI | 0 | 0 | BEQ | <div style='text-align: right'>1</div>  |
| internal_verifier_height_0 | CT-InitializePcsConst | 0 | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-ReadProofsFromInput | 0 | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-VerifyProofs | 0 | 0 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-compute-reduced-opening | 0 | 0 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | CT-exp-reverse-bits-len | 0 | 0 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash | 0 | 0 | PHANTOM | <div style='text-align: right'>5,040</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash-ext | 0 | 0 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash-setup | 0 | 0 | PHANTOM | <div style='text-align: right'>237,552</div>  |
| internal_verifier_height_0 | CT-single-reduced-opening-eval | 0 | 0 | PHANTOM | <div style='text-align: right'>21,168</div>  |
| internal_verifier_height_0 | CT-stage-c-build-rounds | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-2-fri-fold | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-3-verify-challenges | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-verify-pcs | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-e-verify-constraints | 0 | 0 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-verify-batch | 0 | 0 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | CT-verify-batch-ext | 0 | 0 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_0 | CT-verify-batch-reduce-fast | 0 | 0 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_0 | CT-verify-batch-reduce-fast-setup | 0 | 0 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_0 | CT-verify-query | 0 | 0 | PHANTOM | <div style='text-align: right'>168</div>  |
| internal_verifier_height_0 | CastFV | 0 | 0 | ADD | <div style='text-align: right'>16</div>  |
| internal_verifier_height_0 | DivE | 0 | 0 | BBE4DIV | <div style='text-align: right'>12,496</div>  |
| internal_verifier_height_0 | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>348</div>  |
| internal_verifier_height_0 | DivEIN | 0 | 0 | STOREW | <div style='text-align: right'>1,392</div>  |
| internal_verifier_height_0 | DivFIN | 0 | 0 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 | For | 0 | 0 | ADD | <div style='text-align: right'>867,687</div>  |
| internal_verifier_height_0 | For | 0 | 0 | BNE | <div style='text-align: right'>958,662</div>  |
| internal_verifier_height_0 | For | 0 | 0 | JAL | <div style='text-align: right'>90,975</div>  |
| internal_verifier_height_0 | For | 0 | 0 | LOADW | <div style='text-align: right'>4,452</div>  |
| internal_verifier_height_0 | For | 0 | 0 | STOREW | <div style='text-align: right'>86,523</div>  |
| internal_verifier_height_0 | FriReducedOpening | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 | HintBitsF | 0 | 0 | PHANTOM | <div style='text-align: right'>86</div>  |
| internal_verifier_height_0 | HintInputVec | 0 | 0 | PHANTOM | <div style='text-align: right'>47,777</div>  |
| internal_verifier_height_0 | IfEq | 0 | 0 | BNE | <div style='text-align: right'>43,644</div>  |
| internal_verifier_height_0 | IfEqI | 0 | 0 | BNE | <div style='text-align: right'>322,878</div>  |
| internal_verifier_height_0 | IfEqI | 0 | 0 | JAL | <div style='text-align: right'>89,101</div>  |
| internal_verifier_height_0 | IfNe | 0 | 0 | BEQ | <div style='text-align: right'>32,894</div>  |
| internal_verifier_height_0 | IfNe | 0 | 0 | JAL | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | IfNeI | 0 | 0 | BEQ | <div style='text-align: right'>4,546</div>  |
| internal_verifier_height_0 | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>6,272</div>  |
| internal_verifier_height_0 | ImmF | 0 | 0 | STOREW | <div style='text-align: right'>70,776</div>  |
| internal_verifier_height_0 | ImmV | 0 | 0 | STOREW | <div style='text-align: right'>55,417</div>  |
| internal_verifier_height_0 | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>43,288</div>  |
| internal_verifier_height_0 | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>140,584</div>  |
| internal_verifier_height_0 | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>54,898</div>  |
| internal_verifier_height_0 | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>159,224</div>  |
| internal_verifier_height_0 | LoadV | 0 | 0 | LOADW | <div style='text-align: right'>56,522</div>  |
| internal_verifier_height_0 | LoadV | 0 | 0 | LOADW2 | <div style='text-align: right'>468,104</div>  |
| internal_verifier_height_0 | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>31,330</div>  |
| internal_verifier_height_0 | MulEF | 0 | 0 | MUL | <div style='text-align: right'>10,176</div>  |
| internal_verifier_height_0 | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>792</div>  |
| internal_verifier_height_0 | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>2,084</div>  |
| internal_verifier_height_0 | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>8,336</div>  |
| internal_verifier_height_0 | MulF | 0 | 0 | MUL | <div style='text-align: right'>290,608</div>  |
| internal_verifier_height_0 | MulFI | 0 | 0 | MUL | <div style='text-align: right'>32</div>  |
| internal_verifier_height_0 | MulV | 0 | 0 | MUL | <div style='text-align: right'>2,666</div>  |
| internal_verifier_height_0 | MulVI | 0 | 0 | MUL | <div style='text-align: right'>42,370</div>  |
| internal_verifier_height_0 | NegE | 0 | 0 | MUL | <div style='text-align: right'>376</div>  |
| internal_verifier_height_0 | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>34,608</div>  |
| internal_verifier_height_0 | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>17,398</div>  |
| internal_verifier_height_0 | Publish | 0 | 0 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>51,488</div>  |
| internal_verifier_height_0 | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>76,120</div>  |
| internal_verifier_height_0 | StoreF | 0 | 0 | STOREW | <div style='text-align: right'>55,316</div>  |
| internal_verifier_height_0 | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>134,964</div>  |
| internal_verifier_height_0 | StoreHintWord | 0 | 0 | ADD | <div style='text-align: right'>413,721</div>  |
| internal_verifier_height_0 | StoreHintWord | 0 | 0 | SHINTW | <div style='text-align: right'>464,164</div>  |
| internal_verifier_height_0 | StoreV | 0 | 0 | STOREW | <div style='text-align: right'>6,476</div>  |
| internal_verifier_height_0 | StoreV | 0 | 0 | STOREW2 | <div style='text-align: right'>132,470</div>  |
| internal_verifier_height_0 | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>7,260</div>  |
| internal_verifier_height_0 | SubEF | 0 | 0 | LOADW | <div style='text-align: right'>31,968</div>  |
| internal_verifier_height_0 | SubEF | 0 | 0 | SUB | <div style='text-align: right'>10,656</div>  |
| internal_verifier_height_0 | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>448</div>  |
| internal_verifier_height_0 | SubEI | 0 | 0 | ADD | <div style='text-align: right'>2,784</div>  |
| internal_verifier_height_0 | SubF | 0 | 0 | SUB | <div style='text-align: right'>16</div>  |
| internal_verifier_height_0 | SubV | 0 | 0 | SUB | <div style='text-align: right'>163,244</div>  |
| internal_verifier_height_0 | SubVI | 0 | 0 | SUB | <div style='text-align: right'>4,844</div>  |
| internal_verifier_height_0 | SubVIN | 0 | 0 | SUB | <div style='text-align: right'>1,848</div>  |
| internal_verifier_height_0 |  | 0 | 1 | JAL | <div style='text-align: right'>1</div>  |
| internal_verifier_height_0 |  | 0 | 1 | STOREW | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>24,230</div>  |
| internal_verifier_height_0 | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>688</div>  |
| internal_verifier_height_0 | AddEFFI | 0 | 1 | STOREW | <div style='text-align: right'>2,064</div>  |
| internal_verifier_height_0 | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>664</div>  |
| internal_verifier_height_0 | AddEI | 0 | 1 | ADD | <div style='text-align: right'>65,016</div>  |
| internal_verifier_height_0 | AddFI | 0 | 1 | ADD | <div style='text-align: right'>150,547</div>  |
| internal_verifier_height_0 | AddV | 0 | 1 | ADD | <div style='text-align: right'>34,462</div>  |
| internal_verifier_height_0 | AddVI | 0 | 1 | ADD | <div style='text-align: right'>715,118</div>  |
| internal_verifier_height_0 | Alloc | 0 | 1 | ADD | <div style='text-align: right'>116,417</div>  |
| internal_verifier_height_0 | Alloc | 0 | 1 | LOADW | <div style='text-align: right'>116,417</div>  |
| internal_verifier_height_0 | Alloc | 0 | 1 | MUL | <div style='text-align: right'>68,640</div>  |
| internal_verifier_height_0 | AssertEqE | 0 | 1 | BNE | <div style='text-align: right'>464</div>  |
| internal_verifier_height_0 | AssertEqEI | 0 | 1 | BNE | <div style='text-align: right'>8</div>  |
| internal_verifier_height_0 | AssertEqF | 0 | 1 | BNE | <div style='text-align: right'>18,927</div>  |
| internal_verifier_height_0 | AssertEqFI | 0 | 1 | BNE | <div style='text-align: right'>7</div>  |
| internal_verifier_height_0 | AssertEqV | 0 | 1 | BNE | <div style='text-align: right'>4,938</div>  |
| internal_verifier_height_0 | AssertEqVI | 0 | 1 | BNE | <div style='text-align: right'>440</div>  |
| internal_verifier_height_0 | AssertNeVI | 0 | 1 | BEQ | <div style='text-align: right'>1</div>  |
| internal_verifier_height_0 | CT-InitializePcsConst | 0 | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-ReadProofsFromInput | 0 | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-VerifyProofs | 0 | 1 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | CT-compute-reduced-opening | 0 | 1 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | CT-exp-reverse-bits-len | 0 | 1 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash | 0 | 1 | PHANTOM | <div style='text-align: right'>5,040</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash-ext | 0 | 1 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_0 | CT-poseidon2-hash-setup | 0 | 1 | PHANTOM | <div style='text-align: right'>237,552</div>  |
| internal_verifier_height_0 | CT-single-reduced-opening-eval | 0 | 1 | PHANTOM | <div style='text-align: right'>21,168</div>  |
| internal_verifier_height_0 | CT-stage-c-build-rounds | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-2-fri-fold | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-3-verify-challenges | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-d-verify-pcs | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-stage-e-verify-constraints | 0 | 1 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_0 | CT-verify-batch | 0 | 1 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | CT-verify-batch-ext | 0 | 1 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_0 | CT-verify-batch-reduce-fast | 0 | 1 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_0 | CT-verify-batch-reduce-fast-setup | 0 | 1 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_0 | CT-verify-query | 0 | 1 | PHANTOM | <div style='text-align: right'>168</div>  |
| internal_verifier_height_0 | CastFV | 0 | 1 | ADD | <div style='text-align: right'>16</div>  |
| internal_verifier_height_0 | DivE | 0 | 1 | BBE4DIV | <div style='text-align: right'>12,496</div>  |
| internal_verifier_height_0 | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>348</div>  |
| internal_verifier_height_0 | DivEIN | 0 | 1 | STOREW | <div style='text-align: right'>1,392</div>  |
| internal_verifier_height_0 | DivFIN | 0 | 1 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_0 | For | 0 | 1 | ADD | <div style='text-align: right'>867,633</div>  |
| internal_verifier_height_0 | For | 0 | 1 | BNE | <div style='text-align: right'>958,608</div>  |
| internal_verifier_height_0 | For | 0 | 1 | JAL | <div style='text-align: right'>90,975</div>  |
| internal_verifier_height_0 | For | 0 | 1 | LOADW | <div style='text-align: right'>4,452</div>  |
| internal_verifier_height_0 | For | 0 | 1 | STOREW | <div style='text-align: right'>86,523</div>  |
| internal_verifier_height_0 | FriReducedOpening | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_0 | HintBitsF | 0 | 1 | PHANTOM | <div style='text-align: right'>86</div>  |
| internal_verifier_height_0 | HintInputVec | 0 | 1 | PHANTOM | <div style='text-align: right'>47,777</div>  |
| internal_verifier_height_0 | IfEq | 0 | 1 | BNE | <div style='text-align: right'>44,148</div>  |
| internal_verifier_height_0 | IfEqI | 0 | 1 | BNE | <div style='text-align: right'>322,374</div>  |
| internal_verifier_height_0 | IfEqI | 0 | 1 | JAL | <div style='text-align: right'>89,063</div>  |
| internal_verifier_height_0 | IfNe | 0 | 1 | BEQ | <div style='text-align: right'>32,894</div>  |
| internal_verifier_height_0 | IfNe | 0 | 1 | JAL | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | IfNeI | 0 | 1 | BEQ | <div style='text-align: right'>4,546</div>  |
| internal_verifier_height_0 | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>6,272</div>  |
| internal_verifier_height_0 | ImmF | 0 | 1 | STOREW | <div style='text-align: right'>70,776</div>  |
| internal_verifier_height_0 | ImmV | 0 | 1 | STOREW | <div style='text-align: right'>55,333</div>  |
| internal_verifier_height_0 | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>43,288</div>  |
| internal_verifier_height_0 | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>140,584</div>  |
| internal_verifier_height_0 | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>54,898</div>  |
| internal_verifier_height_0 | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>159,224</div>  |
| internal_verifier_height_0 | LoadV | 0 | 1 | LOADW | <div style='text-align: right'>56,522</div>  |
| internal_verifier_height_0 | LoadV | 0 | 1 | LOADW2 | <div style='text-align: right'>468,104</div>  |
| internal_verifier_height_0 | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>31,280</div>  |
| internal_verifier_height_0 | MulEF | 0 | 1 | MUL | <div style='text-align: right'>10,176</div>  |
| internal_verifier_height_0 | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>792</div>  |
| internal_verifier_height_0 | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>2,084</div>  |
| internal_verifier_height_0 | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>8,336</div>  |
| internal_verifier_height_0 | MulF | 0 | 1 | MUL | <div style='text-align: right'>289,600</div>  |
| internal_verifier_height_0 | MulFI | 0 | 1 | MUL | <div style='text-align: right'>32</div>  |
| internal_verifier_height_0 | MulV | 0 | 1 | MUL | <div style='text-align: right'>2,666</div>  |
| internal_verifier_height_0 | MulVI | 0 | 1 | MUL | <div style='text-align: right'>42,370</div>  |
| internal_verifier_height_0 | NegE | 0 | 1 | MUL | <div style='text-align: right'>376</div>  |
| internal_verifier_height_0 | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>34,608</div>  |
| internal_verifier_height_0 | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>17,398</div>  |
| internal_verifier_height_0 | Publish | 0 | 1 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_0 | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>51,488</div>  |
| internal_verifier_height_0 | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>76,120</div>  |
| internal_verifier_height_0 | StoreF | 0 | 1 | STOREW | <div style='text-align: right'>55,316</div>  |
| internal_verifier_height_0 | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>134,964</div>  |
| internal_verifier_height_0 | StoreHintWord | 0 | 1 | ADD | <div style='text-align: right'>413,721</div>  |
| internal_verifier_height_0 | StoreHintWord | 0 | 1 | SHINTW | <div style='text-align: right'>464,164</div>  |
| internal_verifier_height_0 | StoreV | 0 | 1 | STOREW | <div style='text-align: right'>6,476</div>  |
| internal_verifier_height_0 | StoreV | 0 | 1 | STOREW2 | <div style='text-align: right'>132,470</div>  |
| internal_verifier_height_0 | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>7,260</div>  |
| internal_verifier_height_0 | SubEF | 0 | 1 | LOADW | <div style='text-align: right'>31,968</div>  |
| internal_verifier_height_0 | SubEF | 0 | 1 | SUB | <div style='text-align: right'>10,656</div>  |
| internal_verifier_height_0 | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>448</div>  |
| internal_verifier_height_0 | SubEI | 0 | 1 | ADD | <div style='text-align: right'>2,784</div>  |
| internal_verifier_height_0 | SubF | 0 | 1 | SUB | <div style='text-align: right'>16</div>  |
| internal_verifier_height_0 | SubV | 0 | 1 | SUB | <div style='text-align: right'>162,740</div>  |
| internal_verifier_height_0 | SubVI | 0 | 1 | SUB | <div style='text-align: right'>4,844</div>  |
| internal_verifier_height_0 | SubVIN | 0 | 1 | SUB | <div style='text-align: right'>1,848</div>  |
| internal_verifier_height_1 |  | 1 | 2 | JAL | <div style='text-align: right'>1</div>  |
| internal_verifier_height_1 |  | 1 | 2 | STOREW | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>24,292</div>  |
| internal_verifier_height_1 | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>720</div>  |
| internal_verifier_height_1 | AddEFFI | 1 | 2 | STOREW | <div style='text-align: right'>2,160</div>  |
| internal_verifier_height_1 | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>664</div>  |
| internal_verifier_height_1 | AddEI | 1 | 2 | ADD | <div style='text-align: right'>65,384</div>  |
| internal_verifier_height_1 | AddFI | 1 | 2 | ADD | <div style='text-align: right'>152,476</div>  |
| internal_verifier_height_1 | AddV | 1 | 2 | ADD | <div style='text-align: right'>34,478</div>  |
| internal_verifier_height_1 | AddVI | 1 | 2 | ADD | <div style='text-align: right'>717,702</div>  |
| internal_verifier_height_1 | Alloc | 1 | 2 | ADD | <div style='text-align: right'>116,585</div>  |
| internal_verifier_height_1 | Alloc | 1 | 2 | LOADW | <div style='text-align: right'>116,585</div>  |
| internal_verifier_height_1 | Alloc | 1 | 2 | MUL | <div style='text-align: right'>68,724</div>  |
| internal_verifier_height_1 | AssertEqE | 1 | 2 | BNE | <div style='text-align: right'>464</div>  |
| internal_verifier_height_1 | AssertEqEI | 1 | 2 | BNE | <div style='text-align: right'>8</div>  |
| internal_verifier_height_1 | AssertEqF | 1 | 2 | BNE | <div style='text-align: right'>18,943</div>  |
| internal_verifier_height_1 | AssertEqFI | 1 | 2 | BNE | <div style='text-align: right'>7</div>  |
| internal_verifier_height_1 | AssertEqV | 1 | 2 | BNE | <div style='text-align: right'>4,938</div>  |
| internal_verifier_height_1 | AssertEqVI | 1 | 2 | BNE | <div style='text-align: right'>440</div>  |
| internal_verifier_height_1 | AssertNeVI | 1 | 2 | BEQ | <div style='text-align: right'>1</div>  |
| internal_verifier_height_1 | CT-InitializePcsConst | 1 | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | CT-ReadProofsFromInput | 1 | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | CT-VerifyProofs | 1 | 2 | PHANTOM | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | CT-compute-reduced-opening | 1 | 2 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_1 | CT-exp-reverse-bits-len | 1 | 2 | PHANTOM | <div style='text-align: right'>15,288</div>  |
| internal_verifier_height_1 | CT-poseidon2-hash | 1 | 2 | PHANTOM | <div style='text-align: right'>5,040</div>  |
| internal_verifier_height_1 | CT-poseidon2-hash-ext | 1 | 2 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_1 | CT-poseidon2-hash-setup | 1 | 2 | PHANTOM | <div style='text-align: right'>237,720</div>  |
| internal_verifier_height_1 | CT-single-reduced-opening-eval | 1 | 2 | PHANTOM | <div style='text-align: right'>21,168</div>  |
| internal_verifier_height_1 | CT-stage-c-build-rounds | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-d-1-verify-shape-and-sample-challenges | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-d-2-fri-fold | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-d-3-verify-challenges | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-d-verify-pcs | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-stage-e-verify-constraints | 1 | 2 | PHANTOM | <div style='text-align: right'>4</div>  |
| internal_verifier_height_1 | CT-verify-batch | 1 | 2 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_1 | CT-verify-batch-ext | 1 | 2 | PHANTOM | <div style='text-align: right'>3,696</div>  |
| internal_verifier_height_1 | CT-verify-batch-reduce-fast | 1 | 2 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_1 | CT-verify-batch-reduce-fast-setup | 1 | 2 | PHANTOM | <div style='text-align: right'>8,736</div>  |
| internal_verifier_height_1 | CT-verify-query | 1 | 2 | PHANTOM | <div style='text-align: right'>168</div>  |
| internal_verifier_height_1 | CastFV | 1 | 2 | ADD | <div style='text-align: right'>16</div>  |
| internal_verifier_height_1 | DivE | 1 | 2 | BBE4DIV | <div style='text-align: right'>12,496</div>  |
| internal_verifier_height_1 | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>348</div>  |
| internal_verifier_height_1 | DivEIN | 1 | 2 | STOREW | <div style='text-align: right'>1,392</div>  |
| internal_verifier_height_1 | DivFIN | 1 | 2 | DIV | <div style='text-align: right'>728</div>  |
| internal_verifier_height_1 | For | 1 | 2 | ADD | <div style='text-align: right'>868,973</div>  |
| internal_verifier_height_1 | For | 1 | 2 | BNE | <div style='text-align: right'>960,032</div>  |
| internal_verifier_height_1 | For | 1 | 2 | JAL | <div style='text-align: right'>91,059</div>  |
| internal_verifier_height_1 | For | 1 | 2 | LOADW | <div style='text-align: right'>4,452</div>  |
| internal_verifier_height_1 | For | 1 | 2 | STOREW | <div style='text-align: right'>86,607</div>  |
| internal_verifier_height_1 | FriReducedOpening | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>10,584</div>  |
| internal_verifier_height_1 | HintBitsF | 1 | 2 | PHANTOM | <div style='text-align: right'>86</div>  |
| internal_verifier_height_1 | HintInputVec | 1 | 2 | PHANTOM | <div style='text-align: right'>47,861</div>  |
| internal_verifier_height_1 | IfEq | 1 | 2 | BNE | <div style='text-align: right'>42,384</div>  |
| internal_verifier_height_1 | IfEqI | 1 | 2 | BNE | <div style='text-align: right'>324,422</div>  |
| internal_verifier_height_1 | IfEqI | 1 | 2 | JAL | <div style='text-align: right'>89,691</div>  |
| internal_verifier_height_1 | IfNe | 1 | 2 | BEQ | <div style='text-align: right'>32,978</div>  |
| internal_verifier_height_1 | IfNe | 1 | 2 | JAL | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | IfNeI | 1 | 2 | BEQ | <div style='text-align: right'>4,546</div>  |
| internal_verifier_height_1 | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>6,400</div>  |
| internal_verifier_height_1 | ImmF | 1 | 2 | STOREW | <div style='text-align: right'>70,776</div>  |
| internal_verifier_height_1 | ImmV | 1 | 2 | STOREW | <div style='text-align: right'>55,189</div>  |
| internal_verifier_height_1 | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>43,304</div>  |
| internal_verifier_height_1 | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>140,584</div>  |
| internal_verifier_height_1 | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>54,962</div>  |
| internal_verifier_height_1 | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>159,340</div>  |
| internal_verifier_height_1 | LoadV | 1 | 2 | LOADW | <div style='text-align: right'>56,606</div>  |
| internal_verifier_height_1 | LoadV | 1 | 2 | LOADW2 | <div style='text-align: right'>468,356</div>  |
| internal_verifier_height_1 | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>31,542</div>  |
| internal_verifier_height_1 | MulEF | 1 | 2 | MUL | <div style='text-align: right'>10,176</div>  |
| internal_verifier_height_1 | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>792</div>  |
| internal_verifier_height_1 | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>2,084</div>  |
| internal_verifier_height_1 | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>8,336</div>  |
| internal_verifier_height_1 | MulF | 1 | 2 | MUL | <div style='text-align: right'>293,296</div>  |
| internal_verifier_height_1 | MulFI | 1 | 2 | MUL | <div style='text-align: right'>32</div>  |
| internal_verifier_height_1 | MulV | 1 | 2 | MUL | <div style='text-align: right'>2,666</div>  |
| internal_verifier_height_1 | MulVI | 1 | 2 | MUL | <div style='text-align: right'>42,454</div>  |
| internal_verifier_height_1 | NegE | 1 | 2 | MUL | <div style='text-align: right'>376</div>  |
| internal_verifier_height_1 | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>34,692</div>  |
| internal_verifier_height_1 | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>17,570</div>  |
| internal_verifier_height_1 | Publish | 1 | 2 | PUBLISH | <div style='text-align: right'>52</div>  |
| internal_verifier_height_1 | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>51,488</div>  |
| internal_verifier_height_1 | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>76,120</div>  |
| internal_verifier_height_1 | StoreF | 1 | 2 | STOREW | <div style='text-align: right'>55,316</div>  |
| internal_verifier_height_1 | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>135,080</div>  |
| internal_verifier_height_1 | StoreHintWord | 1 | 2 | ADD | <div style='text-align: right'>414,525</div>  |
| internal_verifier_height_1 | StoreHintWord | 1 | 2 | SHINTW | <div style='text-align: right'>465,052</div>  |
| internal_verifier_height_1 | StoreV | 1 | 2 | STOREW | <div style='text-align: right'>6,476</div>  |
| internal_verifier_height_1 | StoreV | 1 | 2 | STOREW2 | <div style='text-align: right'>132,638</div>  |
| internal_verifier_height_1 | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>7,260</div>  |
| internal_verifier_height_1 | SubEF | 1 | 2 | LOADW | <div style='text-align: right'>31,968</div>  |
| internal_verifier_height_1 | SubEF | 1 | 2 | SUB | <div style='text-align: right'>10,656</div>  |
| internal_verifier_height_1 | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>472</div>  |
| internal_verifier_height_1 | SubEI | 1 | 2 | ADD | <div style='text-align: right'>2,784</div>  |
| internal_verifier_height_1 | SubF | 1 | 2 | SUB | <div style='text-align: right'>16</div>  |
| internal_verifier_height_1 | SubV | 1 | 2 | SUB | <div style='text-align: right'>164,588</div>  |
| internal_verifier_height_1 | SubVI | 1 | 2 | SUB | <div style='text-align: right'>4,844</div>  |
| internal_verifier_height_1 | SubVIN | 1 | 2 | SUB | <div style='text-align: right'>1,848</div>  |

| group | air_name | dsl_ir | height | index | opcode | cells_used |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 0 | JAL | <div style='text-align: right'>10</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 0 | STOREW | <div style='text-align: right'>82</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 0 | STOREW | <div style='text-align: right'>22</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>969,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>423,346</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>250,159</div>  |
| internal_verifier_height_0 | Boundary | AddE | 0 | 0 | FE4ADD | <div style='text-align: right'>96,448</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>28,208</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>4,312</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>5,096</div>  |
| internal_verifier_height_0 | Boundary | AddEFFI | 0 | 0 | LOADW | <div style='text-align: right'>1,100</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | 0 | STOREW | <div style='text-align: right'>84,624</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFFI | 0 | 0 | STOREW | <div style='text-align: right'>4,312</div>  |
| internal_verifier_height_0 | Boundary | AddEFFI | 0 | 0 | STOREW | <div style='text-align: right'>3,300</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>19,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>3,894</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>2,301</div>  |
| internal_verifier_height_0 | Boundary | AddEFI | 0 | 0 | ADD | <div style='text-align: right'>1,540</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | 0 | ADD | <div style='text-align: right'>1,950,480</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEI | 0 | 0 | ADD | <div style='text-align: right'>409,068</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEI | 0 | 0 | ADD | <div style='text-align: right'>241,722</div>  |
| internal_verifier_height_0 | Boundary | AddEI | 0 | 0 | ADD | <div style='text-align: right'>160,424</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | 0 | ADD | <div style='text-align: right'>4,531,920</div>  |
| internal_verifier_height_0 | Boundary | AddFI | 0 | 0 | ADD | <div style='text-align: right'>1,221</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | 0 | ADD | <div style='text-align: right'>1,033,980</div>  |
| internal_verifier_height_0 | Boundary | AddV | 0 | 0 | ADD | <div style='text-align: right'>22</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | 0 | ADD | <div style='text-align: right'>21,468,660</div>  |
| internal_verifier_height_0 | Boundary | AddVI | 0 | 0 | ADD | <div style='text-align: right'>15,796</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | 0 | ADD | <div style='text-align: right'>3,492,510</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 0 | 0 | LOADW | <div style='text-align: right'>4,773,097</div>  |
| internal_verifier_height_0 | Boundary | Alloc | 0 | 0 | LOADW | <div style='text-align: right'>1,320</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | 0 | MUL | <div style='text-align: right'>2,059,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Alloc | 0 | 0 | MUL | <div style='text-align: right'>66</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Alloc | 0 | 0 | MUL | <div style='text-align: right'>78</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 0 | 0 | BNE | <div style='text-align: right'>10,672</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AssertEqE | 0 | 0 | BNE | <div style='text-align: right'>2,552</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AssertEqE | 0 | 0 | BNE | <div style='text-align: right'>1,508</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 0 | 0 | BNE | <div style='text-align: right'>184</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AssertEqEI | 0 | 0 | BNE | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AssertEqEI | 0 | 0 | BNE | <div style='text-align: right'>26</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 0 | 0 | BNE | <div style='text-align: right'>435,321</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 0 | 0 | BNE | <div style='text-align: right'>161</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 0 | 0 | BNE | <div style='text-align: right'>113,574</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 0 | 0 | BNE | <div style='text-align: right'>10,120</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 0 | 0 | BEQ | <div style='text-align: right'>23</div>  |
| internal_verifier_height_0 | PhantomAir | CT-InitializePcsConst | 0 | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-ReadProofsFromInput | 0 | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-VerifyProofs | 0 | 0 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-compute-reduced-opening | 0 | 0 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_0 | PhantomAir | CT-exp-reverse-bits-len | 0 | 0 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash | 0 | 0 | PHANTOM | <div style='text-align: right'>30,240</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash-ext | 0 | 0 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash-setup | 0 | 0 | PHANTOM | <div style='text-align: right'>1,425,312</div>  |
| internal_verifier_height_0 | PhantomAir | CT-single-reduced-opening-eval | 0 | 0 | PHANTOM | <div style='text-align: right'>127,008</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-c-build-rounds | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-2-fri-fold | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-3-verify-challenges | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-verify-pcs | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-e-verify-constraints | 0 | 0 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch | 0 | 0 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-ext | 0 | 0 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-reduce-fast | 0 | 0 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-reduce-fast-setup | 0 | 0 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-query | 0 | 0 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 0 | 0 | ADD | <div style='text-align: right'>480</div>  |
| internal_verifier_height_0 | Boundary | CastFV | 0 | 0 | ADD | <div style='text-align: right'>88</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 0 | 0 | BBE4DIV | <div style='text-align: right'>499,840</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivE | 0 | 0 | BBE4DIV | <div style='text-align: right'>235,004</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivE | 0 | 0 | BBE4DIV | <div style='text-align: right'>138,866</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>13,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>17,270</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>10,205</div>  |
| internal_verifier_height_0 | Boundary | DivEIN | 0 | 0 | BBE4DIV | <div style='text-align: right'>2,112</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 0 | 0 | STOREW | <div style='text-align: right'>57,072</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivEIN | 0 | 0 | STOREW | <div style='text-align: right'>6,996</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivEIN | 0 | 0 | STOREW | <div style='text-align: right'>3,744</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 0 | 0 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary | DivFIN | 0 | 0 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | 0 | ADD | <div style='text-align: right'>26,030,610</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 0 | 0 | BNE | <div style='text-align: right'>22,049,226</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | For | 0 | 0 | JAL | <div style='text-align: right'>909,750</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | For | 0 | 0 | JAL | <div style='text-align: right'>660</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | For | 0 | 0 | JAL | <div style='text-align: right'>780</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | 0 | LOADW | <div style='text-align: right'>182,532</div>  |
| internal_verifier_height_0 | Boundary | For | 0 | 0 | LOADW | <div style='text-align: right'>473</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | 0 | STOREW | <div style='text-align: right'>3,547,443</div>  |
| internal_verifier_height_0 | Boundary | For | 0 | 0 | STOREW | <div style='text-align: right'>891</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | FriReducedOpening | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | FriReducedOpening | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>167,076</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | FriReducedOpening | 0 | 0 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,999,104</div>  |
| internal_verifier_height_0 | PhantomAir | HintBitsF | 0 | 0 | PHANTOM | <div style='text-align: right'>516</div>  |
| internal_verifier_height_0 | PhantomAir | HintInputVec | 0 | 0 | PHANTOM | <div style='text-align: right'>286,662</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | 0 | BNE | <div style='text-align: right'>1,003,812</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | 0 | BNE | <div style='text-align: right'>7,426,194</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 0 | 0 | JAL | <div style='text-align: right'>891,010</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | IfEqI | 0 | 0 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | IfEqI | 0 | 0 | JAL | <div style='text-align: right'>13</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | 0 | BEQ | <div style='text-align: right'>756,562</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | IfNe | 0 | 0 | JAL | <div style='text-align: right'>20</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | 0 | BEQ | <div style='text-align: right'>104,558</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>257,152</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>17,050</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>10,075</div>  |
| internal_verifier_height_0 | Boundary | ImmE | 0 | 0 | STOREW | <div style='text-align: right'>13,288</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 0 | 0 | STOREW | <div style='text-align: right'>2,901,816</div>  |
| internal_verifier_height_0 | Boundary | ImmF | 0 | 0 | STOREW | <div style='text-align: right'>2,475</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 0 | 0 | STOREW | <div style='text-align: right'>2,272,097</div>  |
| internal_verifier_height_0 | Boundary | ImmV | 0 | 0 | STOREW | <div style='text-align: right'>15,961</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>1,774,808</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>284,614</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>168,181</div>  |
| internal_verifier_height_0 | Boundary | LoadE | 0 | 0 | LOADW | <div style='text-align: right'>4,444</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>5,763,944</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>123,926</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>73,229</div>  |
| internal_verifier_height_0 | Boundary | LoadE | 0 | 0 | LOADW2 | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>2,250,818</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>103,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>61,152</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary | LoadF | 0 | 0 | LOADW | <div style='text-align: right'>517</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>6,528,184</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>1,672</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>988</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary | LoadF | 0 | 0 | LOADW2 | <div style='text-align: right'>550</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | 0 | LOADW | <div style='text-align: right'>2,317,402</div>  |
| internal_verifier_height_0 | Boundary | LoadV | 0 | 0 | LOADW | <div style='text-align: right'>15,290</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | 0 | LOADW2 | <div style='text-align: right'>19,192,264</div>  |
| internal_verifier_height_0 | Boundary | LoadV | 0 | 0 | LOADW2 | <div style='text-align: right'>1,056</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>1,253,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>457,094</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>270,101</div>  |
| internal_verifier_height_0 | Boundary | MulE | 0 | 0 | BBE4MUL | <div style='text-align: right'>148,588</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | 0 | MUL | <div style='text-align: right'>305,280</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEF | 0 | 0 | MUL | <div style='text-align: right'>49,236</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEF | 0 | 0 | MUL | <div style='text-align: right'>29,094</div>  |
| internal_verifier_height_0 | Boundary | MulEF | 0 | 0 | MUL | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>23,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>4,246</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>2,509</div>  |
| internal_verifier_height_0 | Boundary | MulEFI | 0 | 0 | MUL | <div style='text-align: right'>1,012</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>83,360</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>90,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>53,534</div>  |
| internal_verifier_height_0 | Boundary | MulEI | 0 | 0 | BBE4MUL | <div style='text-align: right'>5,148</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>341,776</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>45,749</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>26,988</div>  |
| internal_verifier_height_0 | Boundary | MulEI | 0 | 0 | STOREW | <div style='text-align: right'>33</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | 0 | MUL | <div style='text-align: right'>8,718,240</div>  |
| internal_verifier_height_0 | Boundary | MulF | 0 | 0 | MUL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | 0 | MUL | <div style='text-align: right'>960</div>  |
| internal_verifier_height_0 | Boundary | MulFI | 0 | 0 | MUL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | 0 | 0 | MUL | <div style='text-align: right'>79,980</div>  |
| internal_verifier_height_0 | Boundary | MulV | 0 | 0 | MUL | <div style='text-align: right'>14,641</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | 0 | MUL | <div style='text-align: right'>1,271,100</div>  |
| internal_verifier_height_0 | Boundary | MulVI | 0 | 0 | MUL | <div style='text-align: right'>77</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 0 | 0 | MUL | <div style='text-align: right'>11,280</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | NegE | 0 | 0 | MUL | <div style='text-align: right'>2,398</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | NegE | 0 | 0 | MUL | <div style='text-align: right'>1,417</div>  |
| internal_verifier_height_0 | Boundary | NegE | 0 | 0 | MUL | <div style='text-align: right'>880</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>1,434,048</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>847,392</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>554,064</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 0 | 0 | COMP_POS2 | <div style='text-align: right'>19,345,872</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>941,160</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>558,324</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>370,056</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 0 | 0 | PERM_POS2 | <div style='text-align: right'>9,725,482</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 0 | 0 | PUBLISH | <div style='text-align: right'>1,456</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>2,111,008</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>40,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>24,076</div>  |
| internal_verifier_height_0 | Boundary | StoreE | 0 | 0 | STOREW | <div style='text-align: right'>566,368</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>3,120,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>336,336</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>198,744</div>  |
| internal_verifier_height_0 | Boundary | StoreE | 0 | 0 | STOREW2 | <div style='text-align: right'>83,336</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | 0 | STOREW | <div style='text-align: right'>2,267,956</div>  |
| internal_verifier_height_0 | Boundary | StoreF | 0 | 0 | STOREW | <div style='text-align: right'>608,476</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>5,533,524</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>554,972</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>330,122</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>220,456</div>  |
| internal_verifier_height_0 | Boundary | StoreF | 0 | 0 | STOREW2 | <div style='text-align: right'>170,742</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | 0 | ADD | <div style='text-align: right'>12,411,630</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 0 | 0 | SHINTW | <div style='text-align: right'>19,030,724</div>  |
| internal_verifier_height_0 | Boundary | StoreHintWord | 0 | 0 | SHINTW | <div style='text-align: right'>5,105,804</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | 0 | STOREW | <div style='text-align: right'>265,516</div>  |
| internal_verifier_height_0 | Boundary | StoreV | 0 | 0 | STOREW | <div style='text-align: right'>71,236</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | 0 | STOREW2 | <div style='text-align: right'>5,431,270</div>  |
| internal_verifier_height_0 | Boundary | StoreV | 0 | 0 | STOREW2 | <div style='text-align: right'>1,313,620</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>290,400</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>250,910</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>148,265</div>  |
| internal_verifier_height_0 | Boundary | SubE | 0 | 0 | FE4SUB | <div style='text-align: right'>25,344</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 0 | 0 | LOADW | <div style='text-align: right'>1,310,688</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEF | 0 | 0 | LOADW | <div style='text-align: right'>117,216</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 0 | 0 | SUB | <div style='text-align: right'>319,680</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEF | 0 | 0 | SUB | <div style='text-align: right'>117,216</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEF | 0 | 0 | SUB | <div style='text-align: right'>138,528</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>13,440</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>1,892</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>1,118</div>  |
| internal_verifier_height_0 | Boundary | SubEFI | 0 | 0 | ADD | <div style='text-align: right'>132</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 0 | 0 | ADD | <div style='text-align: right'>83,520</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEI | 0 | 0 | ADD | <div style='text-align: right'>24,750</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEI | 0 | 0 | ADD | <div style='text-align: right'>14,625</div>  |
| internal_verifier_height_0 | Boundary | SubEI | 0 | 0 | ADD | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubF | 0 | 0 | SUB | <div style='text-align: right'>480</div>  |
| internal_verifier_height_0 | Boundary | SubF | 0 | 0 | SUB | <div style='text-align: right'>88</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | 0 | SUB | <div style='text-align: right'>4,897,320</div>  |
| internal_verifier_height_0 | Boundary | SubV | 0 | 0 | SUB | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | 0 | SUB | <div style='text-align: right'>145,320</div>  |
| internal_verifier_height_0 | Boundary | SubVI | 0 | 0 | SUB | <div style='text-align: right'>15,147</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 0 | 0 | SUB | <div style='text-align: right'>55,440</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> |  | 0 | 1 | JAL | <div style='text-align: right'>10</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 0 | 1 | STOREW | <div style='text-align: right'>82</div>  |
| internal_verifier_height_0 | Boundary |  | 0 | 1 | STOREW | <div style='text-align: right'>22</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>969,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>423,346</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>250,159</div>  |
| internal_verifier_height_0 | Boundary | AddE | 0 | 1 | FE4ADD | <div style='text-align: right'>96,448</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>28,208</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>4,312</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>5,096</div>  |
| internal_verifier_height_0 | Boundary | AddEFFI | 0 | 1 | LOADW | <div style='text-align: right'>1,100</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 0 | 1 | STOREW | <div style='text-align: right'>84,624</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFFI | 0 | 1 | STOREW | <div style='text-align: right'>4,312</div>  |
| internal_verifier_height_0 | Boundary | AddEFFI | 0 | 1 | STOREW | <div style='text-align: right'>3,300</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>19,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>3,894</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>2,301</div>  |
| internal_verifier_height_0 | Boundary | AddEFI | 0 | 1 | ADD | <div style='text-align: right'>1,540</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 0 | 1 | ADD | <div style='text-align: right'>1,950,480</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AddEI | 0 | 1 | ADD | <div style='text-align: right'>409,178</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AddEI | 0 | 1 | ADD | <div style='text-align: right'>241,787</div>  |
| internal_verifier_height_0 | Boundary | AddEI | 0 | 1 | ADD | <div style='text-align: right'>160,424</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 0 | 1 | ADD | <div style='text-align: right'>4,516,410</div>  |
| internal_verifier_height_0 | Boundary | AddFI | 0 | 1 | ADD | <div style='text-align: right'>1,221</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 0 | 1 | ADD | <div style='text-align: right'>1,033,860</div>  |
| internal_verifier_height_0 | Boundary | AddV | 0 | 1 | ADD | <div style='text-align: right'>22</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 0 | 1 | ADD | <div style='text-align: right'>21,453,540</div>  |
| internal_verifier_height_0 | Boundary | AddVI | 0 | 1 | ADD | <div style='text-align: right'>15,796</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | 1 | ADD | <div style='text-align: right'>3,492,510</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 0 | 1 | LOADW | <div style='text-align: right'>4,773,097</div>  |
| internal_verifier_height_0 | Boundary | Alloc | 0 | 1 | LOADW | <div style='text-align: right'>1,320</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 0 | 1 | MUL | <div style='text-align: right'>2,059,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Alloc | 0 | 1 | MUL | <div style='text-align: right'>66</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Alloc | 0 | 1 | MUL | <div style='text-align: right'>78</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 0 | 1 | BNE | <div style='text-align: right'>10,672</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AssertEqE | 0 | 1 | BNE | <div style='text-align: right'>2,552</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AssertEqE | 0 | 1 | BNE | <div style='text-align: right'>1,508</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 0 | 1 | BNE | <div style='text-align: right'>184</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | AssertEqEI | 0 | 1 | BNE | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | AssertEqEI | 0 | 1 | BNE | <div style='text-align: right'>26</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 0 | 1 | BNE | <div style='text-align: right'>435,321</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 0 | 1 | BNE | <div style='text-align: right'>161</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 0 | 1 | BNE | <div style='text-align: right'>113,574</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 0 | 1 | BNE | <div style='text-align: right'>10,120</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 0 | 1 | BEQ | <div style='text-align: right'>23</div>  |
| internal_verifier_height_0 | PhantomAir | CT-InitializePcsConst | 0 | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-ReadProofsFromInput | 0 | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-VerifyProofs | 0 | 1 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_0 | PhantomAir | CT-compute-reduced-opening | 0 | 1 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_0 | PhantomAir | CT-exp-reverse-bits-len | 0 | 1 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash | 0 | 1 | PHANTOM | <div style='text-align: right'>30,240</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash-ext | 0 | 1 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_0 | PhantomAir | CT-poseidon2-hash-setup | 0 | 1 | PHANTOM | <div style='text-align: right'>1,425,312</div>  |
| internal_verifier_height_0 | PhantomAir | CT-single-reduced-opening-eval | 0 | 1 | PHANTOM | <div style='text-align: right'>127,008</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-c-build-rounds | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-2-fri-fold | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-3-verify-challenges | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-d-verify-pcs | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-stage-e-verify-constraints | 0 | 1 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch | 0 | 1 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-ext | 0 | 1 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-reduce-fast | 0 | 1 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-batch-reduce-fast-setup | 0 | 1 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_0 | PhantomAir | CT-verify-query | 0 | 1 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 0 | 1 | ADD | <div style='text-align: right'>480</div>  |
| internal_verifier_height_0 | Boundary | CastFV | 0 | 1 | ADD | <div style='text-align: right'>88</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 0 | 1 | BBE4DIV | <div style='text-align: right'>499,840</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivE | 0 | 1 | BBE4DIV | <div style='text-align: right'>235,004</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivE | 0 | 1 | BBE4DIV | <div style='text-align: right'>138,866</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>13,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>17,270</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>10,205</div>  |
| internal_verifier_height_0 | Boundary | DivEIN | 0 | 1 | BBE4DIV | <div style='text-align: right'>2,112</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 0 | 1 | STOREW | <div style='text-align: right'>57,072</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | DivEIN | 0 | 1 | STOREW | <div style='text-align: right'>6,996</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | DivEIN | 0 | 1 | STOREW | <div style='text-align: right'>3,744</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 0 | 1 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_0 | Boundary | DivFIN | 0 | 1 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 0 | 1 | ADD | <div style='text-align: right'>26,028,990</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 0 | 1 | BNE | <div style='text-align: right'>22,047,984</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | For | 0 | 1 | JAL | <div style='text-align: right'>909,750</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | For | 0 | 1 | JAL | <div style='text-align: right'>660</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | For | 0 | 1 | JAL | <div style='text-align: right'>780</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | 1 | LOADW | <div style='text-align: right'>182,532</div>  |
| internal_verifier_height_0 | Boundary | For | 0 | 1 | LOADW | <div style='text-align: right'>473</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 0 | 1 | STOREW | <div style='text-align: right'>3,547,443</div>  |
| internal_verifier_height_0 | Boundary | For | 0 | 1 | STOREW | <div style='text-align: right'>891</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | FriReducedOpening | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | FriReducedOpening | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>167,076</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | FriReducedOpening | 0 | 1 | FRI_REDUCED_OPENING | <div style='text-align: right'>13,999,104</div>  |
| internal_verifier_height_0 | PhantomAir | HintBitsF | 0 | 1 | PHANTOM | <div style='text-align: right'>516</div>  |
| internal_verifier_height_0 | PhantomAir | HintInputVec | 0 | 1 | PHANTOM | <div style='text-align: right'>286,662</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 0 | 1 | BNE | <div style='text-align: right'>1,015,404</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 0 | 1 | BNE | <div style='text-align: right'>7,414,602</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 0 | 1 | JAL | <div style='text-align: right'>890,630</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | IfEqI | 0 | 1 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | IfEqI | 0 | 1 | JAL | <div style='text-align: right'>13</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 0 | 1 | BEQ | <div style='text-align: right'>756,562</div>  |
| internal_verifier_height_0 | <JalNativeAdapterAir,JalCoreAir> | IfNe | 0 | 1 | JAL | <div style='text-align: right'>20</div>  |
| internal_verifier_height_0 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 0 | 1 | BEQ | <div style='text-align: right'>104,558</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>257,152</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>17,050</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>10,075</div>  |
| internal_verifier_height_0 | Boundary | ImmE | 0 | 1 | STOREW | <div style='text-align: right'>13,288</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 0 | 1 | STOREW | <div style='text-align: right'>2,901,816</div>  |
| internal_verifier_height_0 | Boundary | ImmF | 0 | 1 | STOREW | <div style='text-align: right'>2,475</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 0 | 1 | STOREW | <div style='text-align: right'>2,268,653</div>  |
| internal_verifier_height_0 | Boundary | ImmV | 0 | 1 | STOREW | <div style='text-align: right'>15,961</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>1,774,808</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>284,614</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>168,181</div>  |
| internal_verifier_height_0 | Boundary | LoadE | 0 | 1 | LOADW | <div style='text-align: right'>4,444</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>5,763,944</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>123,926</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>73,229</div>  |
| internal_verifier_height_0 | Boundary | LoadE | 0 | 1 | LOADW2 | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>2,250,818</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>103,488</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>61,152</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_0 | Boundary | LoadF | 0 | 1 | LOADW | <div style='text-align: right'>517</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>6,528,184</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>1,672</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>988</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_0 | Boundary | LoadF | 0 | 1 | LOADW2 | <div style='text-align: right'>550</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | 1 | LOADW | <div style='text-align: right'>2,317,402</div>  |
| internal_verifier_height_0 | Boundary | LoadV | 0 | 1 | LOADW | <div style='text-align: right'>15,290</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 0 | 1 | LOADW2 | <div style='text-align: right'>19,192,264</div>  |
| internal_verifier_height_0 | Boundary | LoadV | 0 | 1 | LOADW2 | <div style='text-align: right'>1,056</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>1,251,200</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>457,204</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>270,166</div>  |
| internal_verifier_height_0 | Boundary | MulE | 0 | 1 | BBE4MUL | <div style='text-align: right'>148,588</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 0 | 1 | MUL | <div style='text-align: right'>305,280</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEF | 0 | 1 | MUL | <div style='text-align: right'>49,236</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEF | 0 | 1 | MUL | <div style='text-align: right'>29,094</div>  |
| internal_verifier_height_0 | Boundary | MulEF | 0 | 1 | MUL | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>23,760</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>4,246</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>2,509</div>  |
| internal_verifier_height_0 | Boundary | MulEFI | 0 | 1 | MUL | <div style='text-align: right'>1,012</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>83,360</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>90,596</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>53,534</div>  |
| internal_verifier_height_0 | Boundary | MulEI | 0 | 1 | BBE4MUL | <div style='text-align: right'>5,148</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>341,776</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>45,749</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>26,988</div>  |
| internal_verifier_height_0 | Boundary | MulEI | 0 | 1 | STOREW | <div style='text-align: right'>33</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 0 | 1 | MUL | <div style='text-align: right'>8,688,000</div>  |
| internal_verifier_height_0 | Boundary | MulF | 0 | 1 | MUL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 0 | 1 | MUL | <div style='text-align: right'>960</div>  |
| internal_verifier_height_0 | Boundary | MulFI | 0 | 1 | MUL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | 0 | 1 | MUL | <div style='text-align: right'>79,980</div>  |
| internal_verifier_height_0 | Boundary | MulV | 0 | 1 | MUL | <div style='text-align: right'>14,641</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 0 | 1 | MUL | <div style='text-align: right'>1,271,100</div>  |
| internal_verifier_height_0 | Boundary | MulVI | 0 | 1 | MUL | <div style='text-align: right'>77</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 0 | 1 | MUL | <div style='text-align: right'>11,280</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | NegE | 0 | 1 | MUL | <div style='text-align: right'>2,398</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | NegE | 0 | 1 | MUL | <div style='text-align: right'>1,417</div>  |
| internal_verifier_height_0 | Boundary | NegE | 0 | 1 | MUL | <div style='text-align: right'>880</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>1,434,048</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>847,392</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>554,064</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 0 | 1 | COMP_POS2 | <div style='text-align: right'>19,345,872</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>941,160</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>558,324</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>370,056</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 0 | 1 | PERM_POS2 | <div style='text-align: right'>9,725,482</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 0 | 1 | PUBLISH | <div style='text-align: right'>1,456</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>2,111,008</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>40,744</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>24,076</div>  |
| internal_verifier_height_0 | Boundary | StoreE | 0 | 1 | STOREW | <div style='text-align: right'>566,368</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>3,120,920</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>336,336</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>198,744</div>  |
| internal_verifier_height_0 | Boundary | StoreE | 0 | 1 | STOREW2 | <div style='text-align: right'>83,336</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | 1 | STOREW | <div style='text-align: right'>2,267,956</div>  |
| internal_verifier_height_0 | Boundary | StoreF | 0 | 1 | STOREW | <div style='text-align: right'>608,476</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>5,533,524</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>554,972</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>330,122</div>  |
| internal_verifier_height_0 | AccessAdapter<8> | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>220,456</div>  |
| internal_verifier_height_0 | Boundary | StoreF | 0 | 1 | STOREW2 | <div style='text-align: right'>170,742</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 0 | 1 | ADD | <div style='text-align: right'>12,411,630</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 0 | 1 | SHINTW | <div style='text-align: right'>19,030,724</div>  |
| internal_verifier_height_0 | Boundary | StoreHintWord | 0 | 1 | SHINTW | <div style='text-align: right'>5,105,804</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | 1 | STOREW | <div style='text-align: right'>265,516</div>  |
| internal_verifier_height_0 | Boundary | StoreV | 0 | 1 | STOREW | <div style='text-align: right'>71,236</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 0 | 1 | STOREW2 | <div style='text-align: right'>5,431,270</div>  |
| internal_verifier_height_0 | Boundary | StoreV | 0 | 1 | STOREW2 | <div style='text-align: right'>1,321,012</div>  |
| internal_verifier_height_0 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>290,400</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>250,910</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>148,265</div>  |
| internal_verifier_height_0 | Boundary | SubE | 0 | 1 | FE4SUB | <div style='text-align: right'>25,344</div>  |
| internal_verifier_height_0 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 0 | 1 | LOADW | <div style='text-align: right'>1,310,688</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEF | 0 | 1 | LOADW | <div style='text-align: right'>117,216</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 0 | 1 | SUB | <div style='text-align: right'>319,680</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEF | 0 | 1 | SUB | <div style='text-align: right'>117,216</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEF | 0 | 1 | SUB | <div style='text-align: right'>138,528</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>13,440</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>1,892</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>1,118</div>  |
| internal_verifier_height_0 | Boundary | SubEFI | 0 | 1 | ADD | <div style='text-align: right'>132</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 0 | 1 | ADD | <div style='text-align: right'>83,520</div>  |
| internal_verifier_height_0 | AccessAdapter<2> | SubEI | 0 | 1 | ADD | <div style='text-align: right'>24,750</div>  |
| internal_verifier_height_0 | AccessAdapter<4> | SubEI | 0 | 1 | ADD | <div style='text-align: right'>14,625</div>  |
| internal_verifier_height_0 | Boundary | SubEI | 0 | 1 | ADD | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubF | 0 | 1 | SUB | <div style='text-align: right'>480</div>  |
| internal_verifier_height_0 | Boundary | SubF | 0 | 1 | SUB | <div style='text-align: right'>88</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 0 | 1 | SUB | <div style='text-align: right'>4,882,200</div>  |
| internal_verifier_height_0 | Boundary | SubV | 0 | 1 | SUB | <div style='text-align: right'>44</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 0 | 1 | SUB | <div style='text-align: right'>145,320</div>  |
| internal_verifier_height_0 | Boundary | SubVI | 0 | 1 | SUB | <div style='text-align: right'>15,147</div>  |
| internal_verifier_height_0 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 0 | 1 | SUB | <div style='text-align: right'>55,440</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> |  | 1 | 2 | JAL | <div style='text-align: right'>10</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | JAL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> |  | 1 | 2 | STOREW | <div style='text-align: right'>82</div>  |
| internal_verifier_height_1 | Boundary |  | 1 | 2 | STOREW | <div style='text-align: right'>22</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>971,680</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>422,884</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>249,886</div>  |
| internal_verifier_height_1 | Boundary | AddE | 1 | 2 | FE4ADD | <div style='text-align: right'>95,348</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>29,520</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>4,367</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>5,161</div>  |
| internal_verifier_height_1 | Boundary | AddEFFI | 1 | 2 | LOADW | <div style='text-align: right'>1,276</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | AddEFFI | 1 | 2 | STOREW | <div style='text-align: right'>88,560</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddEFFI | 1 | 2 | STOREW | <div style='text-align: right'>4,367</div>  |
| internal_verifier_height_1 | Boundary | AddEFFI | 1 | 2 | STOREW | <div style='text-align: right'>3,828</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>19,920</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>3,894</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>2,301</div>  |
| internal_verifier_height_1 | Boundary | AddEFI | 1 | 2 | ADD | <div style='text-align: right'>1,496</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddEI | 1 | 2 | ADD | <div style='text-align: right'>1,961,520</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AddEI | 1 | 2 | ADD | <div style='text-align: right'>412,456</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AddEI | 1 | 2 | ADD | <div style='text-align: right'>243,724</div>  |
| internal_verifier_height_1 | Boundary | AddEI | 1 | 2 | ADD | <div style='text-align: right'>160,952</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddFI | 1 | 2 | ADD | <div style='text-align: right'>4,574,280</div>  |
| internal_verifier_height_1 | Boundary | AddFI | 1 | 2 | ADD | <div style='text-align: right'>1,309</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddV | 1 | 2 | ADD | <div style='text-align: right'>1,034,340</div>  |
| internal_verifier_height_1 | Boundary | AddV | 1 | 2 | ADD | <div style='text-align: right'>22</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | AddVI | 1 | 2 | ADD | <div style='text-align: right'>21,531,060</div>  |
| internal_verifier_height_1 | Boundary | AddVI | 1 | 2 | ADD | <div style='text-align: right'>15,796</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 1 | 2 | ADD | <div style='text-align: right'>3,497,550</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | Alloc | 1 | 2 | LOADW | <div style='text-align: right'>4,779,985</div>  |
| internal_verifier_height_1 | Boundary | Alloc | 1 | 2 | LOADW | <div style='text-align: right'>1,320</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | Alloc | 1 | 2 | MUL | <div style='text-align: right'>2,061,720</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | Alloc | 1 | 2 | MUL | <div style='text-align: right'>66</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | Alloc | 1 | 2 | MUL | <div style='text-align: right'>78</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqE | 1 | 2 | BNE | <div style='text-align: right'>10,672</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AssertEqE | 1 | 2 | BNE | <div style='text-align: right'>2,552</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AssertEqE | 1 | 2 | BNE | <div style='text-align: right'>1,508</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqEI | 1 | 2 | BNE | <div style='text-align: right'>184</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | AssertEqEI | 1 | 2 | BNE | <div style='text-align: right'>44</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | AssertEqEI | 1 | 2 | BNE | <div style='text-align: right'>26</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqF | 1 | 2 | BNE | <div style='text-align: right'>435,689</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqFI | 1 | 2 | BNE | <div style='text-align: right'>161</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqV | 1 | 2 | BNE | <div style='text-align: right'>113,574</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertEqVI | 1 | 2 | BNE | <div style='text-align: right'>10,120</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | AssertNeVI | 1 | 2 | BEQ | <div style='text-align: right'>23</div>  |
| internal_verifier_height_1 | PhantomAir | CT-InitializePcsConst | 1 | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_1 | PhantomAir | CT-ReadProofsFromInput | 1 | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_1 | PhantomAir | CT-VerifyProofs | 1 | 2 | PHANTOM | <div style='text-align: right'>12</div>  |
| internal_verifier_height_1 | PhantomAir | CT-compute-reduced-opening | 1 | 2 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_1 | PhantomAir | CT-exp-reverse-bits-len | 1 | 2 | PHANTOM | <div style='text-align: right'>91,728</div>  |
| internal_verifier_height_1 | PhantomAir | CT-poseidon2-hash | 1 | 2 | PHANTOM | <div style='text-align: right'>30,240</div>  |
| internal_verifier_height_1 | PhantomAir | CT-poseidon2-hash-ext | 1 | 2 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_1 | PhantomAir | CT-poseidon2-hash-setup | 1 | 2 | PHANTOM | <div style='text-align: right'>1,426,320</div>  |
| internal_verifier_height_1 | PhantomAir | CT-single-reduced-opening-eval | 1 | 2 | PHANTOM | <div style='text-align: right'>127,008</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-c-build-rounds | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-d-1-verify-shape-and-sample-challenges | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-d-2-fri-fold | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-d-3-verify-challenges | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-d-verify-pcs | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-stage-e-verify-constraints | 1 | 2 | PHANTOM | <div style='text-align: right'>24</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-batch | 1 | 2 | PHANTOM | <div style='text-align: right'>6,048</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-batch-ext | 1 | 2 | PHANTOM | <div style='text-align: right'>22,176</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-batch-reduce-fast | 1 | 2 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-batch-reduce-fast-setup | 1 | 2 | PHANTOM | <div style='text-align: right'>52,416</div>  |
| internal_verifier_height_1 | PhantomAir | CT-verify-query | 1 | 2 | PHANTOM | <div style='text-align: right'>1,008</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | CastFV | 1 | 2 | ADD | <div style='text-align: right'>480</div>  |
| internal_verifier_height_1 | Boundary | CastFV | 1 | 2 | ADD | <div style='text-align: right'>88</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivE | 1 | 2 | BBE4DIV | <div style='text-align: right'>499,840</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | DivE | 1 | 2 | BBE4DIV | <div style='text-align: right'>235,004</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | DivE | 1 | 2 | BBE4DIV | <div style='text-align: right'>138,866</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>13,920</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>16,918</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>9,997</div>  |
| internal_verifier_height_1 | Boundary | DivEIN | 1 | 2 | BBE4DIV | <div style='text-align: right'>2,112</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | DivEIN | 1 | 2 | STOREW | <div style='text-align: right'>57,072</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | DivEIN | 1 | 2 | STOREW | <div style='text-align: right'>6,996</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | DivEIN | 1 | 2 | STOREW | <div style='text-align: right'>3,744</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | DivFIN | 1 | 2 | DIV | <div style='text-align: right'>21,840</div>  |
| internal_verifier_height_1 | Boundary | DivFIN | 1 | 2 | DIV | <div style='text-align: right'>297</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | For | 1 | 2 | ADD | <div style='text-align: right'>26,069,190</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | For | 1 | 2 | BNE | <div style='text-align: right'>22,080,736</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | For | 1 | 2 | JAL | <div style='text-align: right'>910,590</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | For | 1 | 2 | JAL | <div style='text-align: right'>660</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | For | 1 | 2 | JAL | <div style='text-align: right'>780</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 1 | 2 | LOADW | <div style='text-align: right'>182,532</div>  |
| internal_verifier_height_1 | Boundary | For | 1 | 2 | LOADW | <div style='text-align: right'>473</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | For | 1 | 2 | STOREW | <div style='text-align: right'>3,550,887</div>  |
| internal_verifier_height_1 | Boundary | For | 1 | 2 | STOREW | <div style='text-align: right'>891</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | FriReducedOpening | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>282,832</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | FriReducedOpening | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>167,128</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | FriReducedOpening | 1 | 2 | FRI_REDUCED_OPENING | <div style='text-align: right'>14,009,856</div>  |
| internal_verifier_height_1 | PhantomAir | HintBitsF | 1 | 2 | PHANTOM | <div style='text-align: right'>516</div>  |
| internal_verifier_height_1 | PhantomAir | HintInputVec | 1 | 2 | PHANTOM | <div style='text-align: right'>287,166</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEq | 1 | 2 | BNE | <div style='text-align: right'>974,832</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfEqI | 1 | 2 | BNE | <div style='text-align: right'>7,461,706</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | IfEqI | 1 | 2 | JAL | <div style='text-align: right'>896,910</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | IfEqI | 1 | 2 | JAL | <div style='text-align: right'>22</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | IfEqI | 1 | 2 | JAL | <div style='text-align: right'>26</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNe | 1 | 2 | BEQ | <div style='text-align: right'>758,494</div>  |
| internal_verifier_height_1 | <JalNativeAdapterAir,JalCoreAir> | IfNe | 1 | 2 | JAL | <div style='text-align: right'>20</div>  |
| internal_verifier_height_1 | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | IfNeI | 1 | 2 | BEQ | <div style='text-align: right'>104,558</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>262,400</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>17,314</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>10,231</div>  |
| internal_verifier_height_1 | Boundary | ImmE | 1 | 2 | STOREW | <div style='text-align: right'>13,640</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmF | 1 | 2 | STOREW | <div style='text-align: right'>2,901,816</div>  |
| internal_verifier_height_1 | Boundary | ImmF | 1 | 2 | STOREW | <div style='text-align: right'>2,475</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | ImmV | 1 | 2 | STOREW | <div style='text-align: right'>2,262,749</div>  |
| internal_verifier_height_1 | Boundary | ImmV | 1 | 2 | STOREW | <div style='text-align: right'>15,961</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>1,775,464</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>285,054</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>168,441</div>  |
| internal_verifier_height_1 | Boundary | LoadE | 1 | 2 | LOADW | <div style='text-align: right'>4,444</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>5,763,944</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>123,926</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>73,229</div>  |
| internal_verifier_height_1 | Boundary | LoadE | 1 | 2 | LOADW2 | <div style='text-align: right'>44</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>2,253,442</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>103,488</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>61,152</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>39,984</div>  |
| internal_verifier_height_1 | Boundary | LoadF | 1 | 2 | LOADW | <div style='text-align: right'>693</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>6,532,940</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>1,672</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>988</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>1,054</div>  |
| internal_verifier_height_1 | Boundary | LoadF | 1 | 2 | LOADW2 | <div style='text-align: right'>550</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 1 | 2 | LOADW | <div style='text-align: right'>2,320,846</div>  |
| internal_verifier_height_1 | Boundary | LoadV | 1 | 2 | LOADW | <div style='text-align: right'>15,290</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | LoadV | 1 | 2 | LOADW2 | <div style='text-align: right'>19,202,596</div>  |
| internal_verifier_height_1 | Boundary | LoadV | 1 | 2 | LOADW2 | <div style='text-align: right'>1,056</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>1,261,680</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>461,648</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>272,792</div>  |
| internal_verifier_height_1 | Boundary | MulE | 1 | 2 | BBE4MUL | <div style='text-align: right'>148,368</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEF | 1 | 2 | MUL | <div style='text-align: right'>305,280</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulEF | 1 | 2 | MUL | <div style='text-align: right'>49,016</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulEF | 1 | 2 | MUL | <div style='text-align: right'>28,964</div>  |
| internal_verifier_height_1 | Boundary | MulEF | 1 | 2 | MUL | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>23,760</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>4,246</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>2,509</div>  |
| internal_verifier_height_1 | Boundary | MulEFI | 1 | 2 | MUL | <div style='text-align: right'>1,012</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>83,360</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>90,552</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>53,508</div>  |
| internal_verifier_height_1 | Boundary | MulEI | 1 | 2 | BBE4MUL | <div style='text-align: right'>5,148</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>341,776</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>45,749</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>26,988</div>  |
| internal_verifier_height_1 | Boundary | MulEI | 1 | 2 | STOREW | <div style='text-align: right'>33</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulF | 1 | 2 | MUL | <div style='text-align: right'>8,798,880</div>  |
| internal_verifier_height_1 | Boundary | MulF | 1 | 2 | MUL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulFI | 1 | 2 | MUL | <div style='text-align: right'>960</div>  |
| internal_verifier_height_1 | Boundary | MulFI | 1 | 2 | MUL | <div style='text-align: right'>11</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulV | 1 | 2 | MUL | <div style='text-align: right'>79,980</div>  |
| internal_verifier_height_1 | Boundary | MulV | 1 | 2 | MUL | <div style='text-align: right'>14,641</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | MulVI | 1 | 2 | MUL | <div style='text-align: right'>1,273,620</div>  |
| internal_verifier_height_1 | Boundary | MulVI | 1 | 2 | MUL | <div style='text-align: right'>77</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | NegE | 1 | 2 | MUL | <div style='text-align: right'>11,280</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | NegE | 1 | 2 | MUL | <div style='text-align: right'>2,376</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | NegE | 1 | 2 | MUL | <div style='text-align: right'>1,404</div>  |
| internal_verifier_height_1 | Boundary | NegE | 1 | 2 | MUL | <div style='text-align: right'>792</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>1,437,744</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>849,576</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>555,492</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | Poseidon2CompressBabyBear | 1 | 2 | COMP_POS2 | <div style='text-align: right'>19,392,828</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>942,260</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>558,428</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>372,980</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | Poseidon2PermuteBabyBear | 1 | 2 | PERM_POS2 | <div style='text-align: right'>9,821,630</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 0>,PublicValuesCoreAir> | Publish | 1 | 2 | PUBLISH | <div style='text-align: right'>1,456</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>2,111,008</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>40,744</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>24,076</div>  |
| internal_verifier_height_1 | Boundary | StoreE | 1 | 2 | STOREW | <div style='text-align: right'>566,368</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>3,120,920</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>336,336</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>198,744</div>  |
| internal_verifier_height_1 | Boundary | StoreE | 1 | 2 | STOREW2 | <div style='text-align: right'>83,336</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 1 | 2 | STOREW | <div style='text-align: right'>2,267,956</div>  |
| internal_verifier_height_1 | Boundary | StoreF | 1 | 2 | STOREW | <div style='text-align: right'>608,476</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>5,538,280</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>556,072</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>330,226</div>  |
| internal_verifier_height_1 | AccessAdapter<8> | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>223,380</div>  |
| internal_verifier_height_1 | Boundary | StoreF | 1 | 2 | STOREW2 | <div style='text-align: right'>170,742</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | StoreHintWord | 1 | 2 | ADD | <div style='text-align: right'>12,435,750</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreHintWord | 1 | 2 | SHINTW | <div style='text-align: right'>19,067,132</div>  |
| internal_verifier_height_1 | Boundary | StoreHintWord | 1 | 2 | SHINTW | <div style='text-align: right'>5,115,572</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 1 | 2 | STOREW | <div style='text-align: right'>265,516</div>  |
| internal_verifier_height_1 | Boundary | StoreV | 1 | 2 | STOREW | <div style='text-align: right'>71,236</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | StoreV | 1 | 2 | STOREW2 | <div style='text-align: right'>5,438,158</div>  |
| internal_verifier_height_1 | Boundary | StoreV | 1 | 2 | STOREW2 | <div style='text-align: right'>1,317,316</div>  |
| internal_verifier_height_1 | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>290,400</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>250,822</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>148,213</div>  |
| internal_verifier_height_1 | Boundary | SubE | 1 | 2 | FE4SUB | <div style='text-align: right'>25,212</div>  |
| internal_verifier_height_1 | <NativeLoadStoreAdapterAir<1>,KernelLoadStoreCoreAir<1>> | SubEF | 1 | 2 | LOADW | <div style='text-align: right'>1,310,688</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubEF | 1 | 2 | LOADW | <div style='text-align: right'>117,216</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEF | 1 | 2 | SUB | <div style='text-align: right'>319,680</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubEF | 1 | 2 | SUB | <div style='text-align: right'>117,216</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | SubEF | 1 | 2 | SUB | <div style='text-align: right'>138,528</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>14,160</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>1,804</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>1,066</div>  |
| internal_verifier_height_1 | Boundary | SubEFI | 1 | 2 | ADD | <div style='text-align: right'>132</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubEI | 1 | 2 | ADD | <div style='text-align: right'>83,520</div>  |
| internal_verifier_height_1 | AccessAdapter<2> | SubEI | 1 | 2 | ADD | <div style='text-align: right'>24,574</div>  |
| internal_verifier_height_1 | AccessAdapter<4> | SubEI | 1 | 2 | ADD | <div style='text-align: right'>14,521</div>  |
| internal_verifier_height_1 | Boundary | SubEI | 1 | 2 | ADD | <div style='text-align: right'>4,224</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubF | 1 | 2 | SUB | <div style='text-align: right'>480</div>  |
| internal_verifier_height_1 | Boundary | SubF | 1 | 2 | SUB | <div style='text-align: right'>88</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubV | 1 | 2 | SUB | <div style='text-align: right'>4,937,640</div>  |
| internal_verifier_height_1 | Boundary | SubV | 1 | 2 | SUB | <div style='text-align: right'>44</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVI | 1 | 2 | SUB | <div style='text-align: right'>145,320</div>  |
| internal_verifier_height_1 | Boundary | SubVI | 1 | 2 | SUB | <div style='text-align: right'>15,147</div>  |
| internal_verifier_height_1 | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | SubVIN | 1 | 2 | SUB | <div style='text-align: right'>55,440</div>  |

| group | air_name | height | index | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | ProgramAir | 0 | 0 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 0 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>2,816</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 0 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 0 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 0 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 0 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 0 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 0 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 0 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 0 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 0 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 0 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | ProgramAir | 0 | 1 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmConnectorAir | 0 | 1 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>2,816</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_0 | VolatileBoundaryAir | 0 | 1 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<2> | 0 | 1 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_0 | AccessAdapterAir<4> | 0 | 1 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | AccessAdapterAir<8> | 0 | 1 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | PhantomAir | 0 | 1 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | 1 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_0 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_0 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | 1 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_0 | FriReducedOpeningAir | 0 | 1 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_0 | Poseidon2VmAir<BabyBearParameters> | 0 | 1 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_0 | VariableRangeCheckerAir | 0 | 1 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | ProgramAir | 1 | 2 | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmConnectorAir | 1 | 2 | 0 | <div style='text-align: right'>24</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 0>, PublicValuesCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>2,816</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>64</div>  |
| internal_verifier_height_1 | VolatileBoundaryAir | 1 | 2 | 0 | <div style='text-align: right'>19,922,944</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<2> | 1 | 2 | 0 | <div style='text-align: right'>28,311,552</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| internal_verifier_height_1 | AccessAdapterAir<4> | 1 | 2 | 0 | <div style='text-align: right'>15,204,352</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | AccessAdapterAir<8> | 1 | 2 | 0 | <div style='text-align: right'>4,325,376</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>16</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | PhantomAir | 1 | 2 | 0 | <div style='text-align: right'>7,340,032</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>8</div>  |  | <div style='text-align: right'>524,288</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeLoadStoreAdapterAir<1>, KernelLoadStoreCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>272,629,760</div>  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 1 | 2 | 0 | <div style='text-align: right'>106,954,752</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>2,097,152</div>  |
| internal_verifier_height_1 | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>5,767,168</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>209,715,200</div>  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>4,194,304</div>  |
| internal_verifier_height_1 | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 1 | 2 | 0 | <div style='text-align: right'>7,864,320</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  | <div style='text-align: right'>131,072</div>  |
| internal_verifier_height_1 | FriReducedOpeningAir | 1 | 2 | 0 | <div style='text-align: right'>36,700,160</div>  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  | <div style='text-align: right'>262,144</div>  |
| internal_verifier_height_1 | Poseidon2VmAir<BabyBearParameters> | 1 | 2 | 0 | <div style='text-align: right'>38,993,920</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>65,536</div>  |
| internal_verifier_height_1 | VariableRangeCheckerAir | 1 | 2 | 0 | <div style='text-align: right'>1,179,648</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>131,072</div>  |

| group | height | index | segment | execute_and_trace_gen_time_ms | stark_prove_excluding_trace_time_ms | total_cells |
| --- | --- | --- | --- | --- | --- | --- |
| internal_verifier_height_0 | 0 | 0 | 0 | <div style='text-align: right'>1,762.0</div>  | <div style='text-align: right'>26,291.0</div>  | <div style='text-align: right'>759,630,616</div>  |
| internal_verifier_height_0 | 0 | 1 | 0 | <div style='text-align: right'>1,767.0</div>  | <div style='text-align: right'>26,263.0</div>  | <div style='text-align: right'>759,630,616</div>  |
| internal_verifier_height_1 | 1 | 2 | 0 | <div style='text-align: right'>1,744.0</div>  | <div style='text-align: right'>26,387.0</div>  | <div style='text-align: right'>759,630,616</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-fibonacci_continuation_program.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_0.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-internal_verifier_height_1.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-leaf_verifier.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/d0f8f72aef387c8ffb696fb2a79f3547b3f57841/fib_e2e-2-2-2-2-64cpu-linux-arm64-mimalloc-root_verifier.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/d0f8f72aef387c8ffb696fb2a79f3547b3f57841

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11959225207)
