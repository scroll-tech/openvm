## Benchmark for Predicate
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 19_202_048 | 876.00 | 176.00 | 8.90 | 32.50 | 658.60 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| PageIndexScanInputAir | 32_768     | 9_830_400   | 0     | [97, 131] | [72] |
| PageIndexScanOutputAir | 32_768     | 8_192_000   | 0     | [97, 81] | [72] |
| VariableRangeCheckerAir | 131_072    | 1_179_648   | 2     | [1] | [8] |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/5fd59fca599e23a09ba49b015e89cba9d6f12d19
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/11000471176)
