# eisenstein-integers

## Definition

The Eisenstein Integers, E, are the set a + bw, where w is the first third root of unity, $$e^{\tau i / 3}$$.  

They form a triangular lattice on the complex plane.  

## Goal

Use Eisenstein Integers to model navigation around a hex grid.  

## API

    struct EisensteinInteger {
        a: u32,
        b: u32,
    }

`EisensteinInteger` should be `Add`, `Mul`.  

    struct Hex(Eisenstein)

`Hex` is represented internally by the point at its center, but exposes a series of operations to navigate in terms of hexes.