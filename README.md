# SYF-Minimal

## Related Repositories

- **SYF-Core** — Canonical law (immutable)  
  https://github.com/AnathemaOfficial/SYF-Core

- **SYF-Lab** — Experimental and observational framework  
  https://github.com/AnathemaOfficial/SYF-Lab


SYF-Minimal is a non-canonical, non-normative reference implementation designed to
illustrate and test the invariant **R** defined in **SYFCORE**.

This repository does **not** define, modify, or extend the SYF law.
Any discrepancy with SYFCORE must be treated as an implementation error.

## Purpose

- Demonstrate deterministic computation of **R**
- Provide test vectors and minimal reference code
- Support external verification and blind testing

## Law

**R = (F × E) / K**

Constraints enforced by the reference implementations:

- F ≥ 0
- 0 ≤ E ≤ 1
- K > 0

## Layout

```
syf-minimal/
├── README.md
├── python/
│   └── syf_core.py
├── rust/
│   └── lib.rs
├── notebook/
│   └── syf_demo.ipynb
├── HOW_TO_DERIVE_F_E.md
└── TEST_VECTORS.md
```

## Status

Illustrative only. Non-canonical. Non-normative.

## Conformance Notes

- K is provided by the caller; the canonical law does not define its value.
- Inputs outside the allowed domain MUST be rejected (error/exception). No clamping.
- Numerical tolerance, if any, is implementation-defined and must be documented.
- This repository is illustrative and non-production by design.

---
