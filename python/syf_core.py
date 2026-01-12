# syf_core.py

def syf_core(F: float, E: float, K: float) -> float:
    """
    Pure SYF invariant computation.
    """

    if F < 0:
        raise ValueError("F must be >= 0")

    if not (0.0 <= E <= 1.0):
        raise ValueError("E must be in [0, 1]")

    if K <= 0:
        raise ValueError("K must be > 0")

    return (F * E) / K
