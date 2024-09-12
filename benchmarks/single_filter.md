## Benchmark for Predicate
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 19_202_048 | 875.00 | 174.00 | 8.49 | 32.80 | 659.71 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| PageIndexScanInputAir | 32_768     | 9_830_400   | 0     | [97, 131] | [72] |
| PageIndexScanOutputAir | 32_768     | 8_192_000   | 0     | [97, 81] | [72] |
| VariableRangeCheckerAir | 131_072    | 1_179_648   | 2     | [1] | [8] |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/2138724b2780e8a20aff3b4886ab9dcf547d58de
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10839487732)
