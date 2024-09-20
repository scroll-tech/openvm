## Benchmark for Predicate
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 19_202_048 | 873.00 | 171.00 | 8.32 | 32.50 | 661.18 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| PageIndexScanInputAir | 32_768     | 9_830_400   | 0     | [97, 131] | [72] |
| PageIndexScanOutputAir | 32_768     | 8_192_000   | 0     | [97, 81] | [72] |
| VariableRangeCheckerAir | 131_072    | 1_179_648   | 2     | [1] | [8] |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/498a772a562e9c42ccbcc8d58c9594e950d490c7
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10963912298)
