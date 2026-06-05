# ternary-muse

**AI-powered creative inspiration for ternary music.**

The Muse doesn't compose — she *suggests*. She takes a seed, a mutation rate, and an aesthetic sensibility, then evolves patterns through generations of variation until something beautiful emerges. This is your creative partner in the ternary sound world: part genetic algorithm, part collaborator, part happy accident.

## Why This Matters

Most generative music tools either give you total control (every note specified) or none at all (pure randomness). `ternary-muse` lives in the fertile middle ground — *guided emergence*. You set the aesthetic weights (do you prize symmetry? complexity? balance?), provide a seed, and let the engine evolve. The result is patterns that feel *discovered* rather than *designed*. That's the magic.

This crate is the creative spine of the SuperInstance ternary ecosystem. It's what you reach for when you've exhausted your own ideas and need the machine to surprise you — not with random noise, but with structures that pass an aesthetic filter.

## What's Inside

### Core Types

- **`Ternary`** — The fundamental trit: `Negative` (-1), `Zero` (0), `Positive` (+1). Convert to/from `i8`, negate.
- **`Pattern`** — A sequence of ternary values. Compute `symmetry()`, `balance()`, and `complexity()` scores.

### AestheticScorer

- **`AestheticScorer::default()`** — Default weights: symmetry 0.4, complexity 0.35, balance 0.25.
- **`AestheticScorer::new(sym_w, comp_w, bal_w)`** — Custom aesthetic priorities.
- **`score(&Pattern) → f64`** — Rate a pattern 0.0–1.0 based on your weights.

### MutationEngine

- **`MutationEngine::new(rate)`** — Create with mutation probability (0.0–1.0).
- **`flip_mutate(&Pattern, seed) → Pattern`** — Negate elements at deterministic positions.
- **`rotate(&Pattern, shift) → Pattern`** — Circular rotation.
- **`reverse(&Pattern) → Pattern`** — Mirror the pattern.
- **`insert_mutate(&Pattern, value, seed) → Pattern`** — Insert a value at a seed-determined position.
- **`delete_mutate(&Pattern, seed) → Pattern`** — Remove a value at a seed-determined position.

### PatternGenerator

- **`PatternGenerator::new(seed)`** — Deterministic generator from seed.
- **`generate(length) → Pattern`** — Generate a raw pattern.
- **`generate_symmetric(half_length) → Pattern`** — Generate a palindromic pattern.
- **`generate_weighted(length, neg_w, zero_w, pos_w) → Pattern`** — Control the ratio of each value.

### StyleTransfer

- **`StyleTransfer::new()`** — Empty style ruleset.
- **`add_rule(source, target)`** — Map a ternary value to another.
- **`apply(&Pattern) → Pattern`** — Apply rules to transform a pattern.
- **`blend(a, b) → Pattern`** — Element-wise average of two patterns.
- **`overlay(a, b, offset) → Pattern`** — Stamp pattern b onto a at position.

### CrossDomainMapper

- **`CrossDomainMapper::new()`** — Fresh mapper.
- **`register_domain(name, neg_label, zero_label, pos_label)`** — Define a domain's interpretation.
- **`translate(&Pattern, from, to) → Vec<String>`** — Re-label a pattern in another domain.
- **`label_for(domain, value) → &str`** — Look up a single label.
- **`domain_names() → Vec<&str>`** — List all registered domains.

### Muse (The Engine)

- **`Muse::new(seed, mutation_rate)`** — Create with default aesthetic weights.
- **`Muse::with_scoring(seed, rate, sym_w, comp_w, bal_w)`** — Create with custom aesthetics.
- **`create_and_evolve(length, generations) → Pattern`** — Generate, mutate, keep the best.
- **`variants(&Pattern, count) → Vec<Pattern>`** — Spin off rotated+mutated variants.
- **`score(&Pattern) → f64`** — Evaluate using internal scorer.

## Quick Example

```rust
use ternary_muse::{Muse, Pattern, Ternary, CrossDomainMapper, AestheticScorer};

// Create a muse that prizes complexity and symmetry
let muse = Muse::with_scoring(42, 0.3, 0.5, 0.4, 0.1);

// Evolve an 8-step pattern over 20 generations
let pattern = muse.create_and_evolve(8, 20);
println!("Best pattern: {:?}", pattern.to_i8_vec());
println!("Aesthetic score: {:.3}", muse.score(&pattern));

// Generate 5 variants from a hand-crafted seed
let seed = Pattern::new(vec![
    Ternary::Positive, Ternary::Zero, Ternary::Negative, Ternary::Positive,
]);
let variants = muse.variants(&seed, 5);
for (i, v) in variants.iter().enumerate() {
    println!("Variant {}: score={:.3}", i, muse.score(v));
}

// Translate into another domain
let mut mapper = CrossDomainMapper::new();
mapper.register_domain("music", "flat", "natural", "sharp");
mapper.register_domain("dance", "left", "hold", "right");
let translated = mapper.translate(&seed, "music", "dance").unwrap();
println!("As dance moves: {:?}", translated);
```

## The Deeper Truth

Here's the thing about creativity in a constrained system: the constraints *are* the creativity. When you only have three values to work with — minus, zero, plus — every placement becomes a meaningful decision. There's no hiding behind velocity curves or microtonal shades. The pattern either *works* or it doesn't, and the `AestheticScorer` is there to help you define what "works" means.

The `Muse` engine's `create_and_evolve` method is doing something deceptively simple: generate a pattern, then flip-mutate it across multiple generations, keeping whichever variant scores highest. But the power is in the composition. You're not just mutating randomly — you're evolving toward an aesthetic ideal you defined. That's not brute force; that's *taste*. Change the weights, and the muse chases a different kind of beauty.

The `CrossDomainMapper` is the sleeper hit of this crate. It's a bridge between worlds. A ternary pattern generated for music (flat/natural/sharp) can be re-interpreted as a dance sequence (left/hold/right), a visual layout (dark/neutral/bright), or anything else you can label. This means the muse's creative output isn't locked to one medium — it's a *structure* that can wear many costumes.

The deterministic seeding deserves attention. Every `PatternGenerator` and `MutationEngine` produces identical output for identical inputs. This isn't a limitation — it's a feature. You can bookmark a seed, share it, reproduce it. "Hey, check out seed 7219 with 0.35 mutation rate" is a recipe, not a suggestion. Collaborative creativity needs reproducibility.

Finally, the `StyleTransfer` tools — blend and overlay — give you a way to combine ideas. Take two patterns that each have something interesting and merge them. The blend operation averages element-wise, producing a kind of compromise. Overlay stamps one pattern onto another at an offset, like layering transparency in a design tool. These aren't just mathematical operations; they're creative ones. The best patterns often emerge from combining two half-good ideas into one great one.

## Use Cases

1. **Live coding performances** — Generate and evolve patterns in real-time, using seeds as "presets" you can recall between sets. The deterministic nature means you can rehearse with exact reproduction.

2. **Generative album art** — Use `CrossDomainMapper` to translate musical ternary patterns into visual domains (warm/neutral/cool colors, dark/mid/light tones), then feed those into a visualizer.

3. **Algorithmic composition workshops** — Teach the principles of emergence and aesthetic selection without getting bogged down in music theory. Students see evolution in action.

4. **Game procedural audio** — Generate terrain-reactive music by seeding the muse from level data. Different biomes = different seeds = different feel.

5. **Brainstorming tool for stuck composers** — Staring at a blank DAW? Spin up a muse with a random seed, generate 20 variants, and something will spark an idea. The ternary constraint forces you to think about *structure*, not details.

## See Also

- **[ternary-jam](https://github.com/clarkeressel/ternary-jam)** — Live jamming with ternary patterns in real-time
- **[ternary-ear](https://github.com/clarkeressel/ternary-ear)** — Ear training for ternary musical intuition
- **[ternary-harmonic](https://github.com/clarkeressel/ternary-harmonic)** — Harmonic analysis in ternary space
- **[ternary-rhythm](https://github.com/clarkeressel/ternary-rhythm)** — Rhythmic pattern generation and manipulation
- **[ternary-music](https://github.com/clarkeressel/ternary-music)** — Music theory operations in ternary
- **[ternary-rack](https://github.com/clarkeressel/ternary-rack)** — Modular synth-style composition with ternary signals
- **[ternary-sampler](https://github.com/clarkeressel/ternary-sampler)** — Sample playback driven by ternary control signals

## Install

```toml
[dependencies]
ternary-muse = "0.1"
```

## License

MIT
