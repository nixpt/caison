# caison

**CAISON — Crush AI-native Semantic Object Notation.** A human-readable configuration and
serialization format that blends JSON structure with AI-native primitives:
**confidence weights, semantic keys, annotations, and synthesized values.**

> **Formerly CSON.** Renamed on 2026-09-27: "CSON" already means CoffeeScript Object
> Notation (npm/PyPI `cson`, and GitHub's `.cson` file type) and the crates.io `cson`
> crate belongs to an unrelated 2014 format. Files are now `.caison`, the version
> directive is `@caison`, and the packages are `caison` in every language.

This repository is the **language-agnostic format home**: the specification and
the **Rust reference parser**. CAISON is not owned by any one language — the
grammar and primitives are language-neutral, and other languages are expected to
ship their own parsers (`caison-py`, …). A language's toolchain that consumes CAISON
is a *user* of the format, not its owner.

- **Spec:** [`SPEC.md`](SPEC.md) — grammar, primitives, JSON projection, errors.
- **Conformance:** [`conformance/`](conformance/) — language-neutral test vectors
  every implementation must pass.

## The four primitives

| Primitive | Syntax | Meaning |
|---|---|---|
| Semantic key | `~"billing or refund issues": "route to support"` | a fuzzy-match anchor — the key *means* this intent |
| Confidence | `temperature: 21.5 ~0.8` | the producer's probability for this node |
| Annotation | `port: 8080 @wip { owner: "foreman" }` | metadata (provenance, status, rationale) |
| Synthesized value | `accent: @synthesize("a complimentary color to #3366cc")` | a value to be *generated* to satisfy the description |

Everything else is JSON-shaped: objects, arrays, strings, numbers, booleans,
`null`, plus comments and optional trailing commas.

## Library

```rust
use caison::CaisonParser;

let doc = CaisonParser::new(r#"
    name: "avalanche"
    port: 8080 @wip
"#).parse()?;

assert_eq!(doc.version, "1.0");
let json = doc.root.to_json()?;   // SPEC §6 projection
# Ok::<(), Box<dyn std::error::Error>>(())
```

Zero-dependency beyond `serde`/`serde_json` (used for the JSON projection). The
format library is embeddable anywhere.

## Build / test

```bash
cargo test                    # unit tests + the conformance corpus
cargo test --test conformance # just the corpus
```

## Conformance

An implementation is conforming iff it passes every vector in
[`conformance/`](conformance/). The corpus is language-neutral — read
`conformance/README.md` to run it from your language.

## Provenance

The Rust parser was extracted from `crush-ast`'s `crush-cson` crate (where it
lived as the Crush front-end's parser). The parser itself was already
language-agnostic; the CrusH-specific `cson.parse` VM host capability stays in
`crush-ast` as a consumer of this crate.

## License

MIT OR Apache-2.0.
