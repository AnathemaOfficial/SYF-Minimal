# SYF Test Vectors

All values assume exact arithmetic.

## Valid Cases

### Case 1
```
F = 10
E = 1.0
K = 2
R = 5.0
```

### Case 2
```
F = 0
E = 0.5
K = 1
R = 0.0
```

### Case 3
```
F = 4
E = 0.25
K = 2
R = 0.5
```

## Invalid Cases

- F < 0
- E < 0 or E > 1
- K <= 0

## Note: Implementations should document numeric tolerance (if any). These vectors are chosen to be exactly representable in common binary floating-point formats.
