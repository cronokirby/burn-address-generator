# burn-address-generator

This can generate penumbra addresses that no one, verifiably, has spend ownership over.

This because the signing key is generated via:
```
seed: String -> SHA-512: [u8; 64] -> (Fq, Fq) --hash_to_curve--> Element
```
creating an element where the associated scalar generating it is not known.

## Usage

```
burn-address-generator <STRING>
```

will generate a burn address deterministically.
Providing the `<STRING>` is evidence that the address isn't spendable.
