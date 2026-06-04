# ternary-muse: Creative generation and artistic exploration with ternary systems

## Why This Exists

Rooms in SuperInstance need to feel alive — not static, not random, but creatively responsive. This crate provides the creative layer that generates novel ternary patterns, evaluates their aesthetic qualities, and translates them across domains (music to visual, visual to spatial). Without it, every room produces the same mechanical output.

## Core Concepts

**Balanced ternary**: A number system with three values: -1 (Negative), 0 (Zero), +1 (Positive). All patterns are sequences of these three values.

**Pattern**: A vector of ternary values. Patterns have measurable properties: symmetry (mirror quality), complexity (transition frequency), and balance (ratio of zeros).

**Muse**: The central creativity engine that combines generation, mutation, scoring, and style transfer into a single workflow.

**Aesthetic scoring**: Evaluates patterns by combining symmetry, complexity, and balance with configurable weights.

**Cross-domain mapping**: Translates the same ternary pattern into different interpretations — e.g., Negative/Zero/Positive becomes "dark/neutral/bright" in visual or "flat/natural/sharp" in music.

## Quick Start

```toml
[dependencies]
ternary-muse = "0.1"
```

```rust
use ternary_muse::{Muse, Ternary, Pattern, CrossDomainMapper};

// Create a muse with a seed and 30% mutation rate
let muse = Muse::new(42, 0.3);

// Evolve a creative pattern over 10 generations
let pattern = muse.create_and_evolve(8, 10);
println!("Score: {:.2}", muse.score(&pattern));

// Set up cross-domain translation
let mut mapper = CrossDomainMapper::new();
mapper.register_domain("visual", "dark", "neutral", "bright");
mapper.register_domain("spatial", "left", "center", "right");
let labels = mapper.translate(&pattern, "visual", "spatial").unwrap();
```

## API Overview

| Type | What it is |
|------|-----------|
| `Ternary` | A single balanced ternary digit (-1, 0, +1) |
| `Pattern` | A sequence of ternary values with symmetry/complexity/balance metrics |
| `Muse` | Central creativity engine combining all tools |
| `PatternGenerator` | Deterministic pattern generation from seeds |
| `MutationEngine` | Controlled pattern mutation (flip, rotate, reverse, insert, delete) |
| `AestheticScorer` | Scores patterns by combining symmetry, complexity, balance |
| `StyleTransfer` | Applies rules and blends/overlays patterns |
| `CrossDomainMapper` | Translates ternary patterns between domain interpretations |

## How It Works

Pattern generation uses deterministic hashing from seeds, making results reproducible. The `MutationEngine` uses a hash-based mutation rate: for each element, a hash of (index, seed) determines whether to mutate. This avoids requiring a random number generator.

Aesthetic scoring combines three metrics with configurable weights (default: 40% symmetry, 35% complexity, 25% balance). Each metric returns 0.0–1.0, and the weighted average is the final score.

The `Muse` evolve loop generates a base pattern, then tries mutations across generations, keeping only higher-scoring variants — a simple hill-climbing search through the pattern space.

## Known Limitations

- **No true randomness**: All generation and mutation is deterministic from seeds. This is by design for reproducibility, but means creative variety comes from seed diversity, not from the algorithms themselves.
- **Hill-climbing can get stuck**: The evolve loop only accepts improvements. Complex pattern spaces may have local optima that prevent finding globally best patterns.
- **No learning or adaptation**: The scorer doesn't learn from feedback. Weights must be manually tuned.
- **Limited cross-domain support**: Currently only label-based translation. No structural transformation between domains.

## Use Cases

- **Room ambiance generation**: Create unique ternary patterns for room lighting/sound that feel intentional, not random.
- **Artistic pattern evolution**: Start from a seed pattern and evolve it toward aesthetic goals.
- **Cross-modal translation**: Take a music pattern (flat/natural/sharp) and translate it to spatial positioning (left/center/right).
- **Style transfer**: Apply one room's aesthetic rules to another room's patterns.

## Ecosystem Context

Part of the SuperInstance ternary ecosystem. Generates creative content that other crates consume:
- `ternary-agent` agents can use generated patterns as behavior modifiers
- `ternary-tidelight` timing coordinates when creative generation happens
- `ternary-flux` can flow generated patterns through the system

No external dependencies — pure Rust.

## License

MIT
