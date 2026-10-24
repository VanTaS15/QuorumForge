# Policy fields

`consensus_threshold` and `dissent_ceiling` are ratios in `[0, 1]`; `minimum_mass` is a
non-negative number of decisive weight. All three must be finite - a NaN threshold would compare
false against every candidate and silently change the classifier, so `Policy::validate` rejects it
and names the field.

Defaults: 0.66 consensus, 0.34 dissent, 1e-9 minimum mass.
