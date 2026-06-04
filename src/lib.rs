#![forbid(unsafe_code)]

//! Creative generation and artistic exploration with ternary systems.
//!
//! Provides a `Muse` struct (creativity engine), `PatternGenerator` for novel
//! ternary patterns, `StyleTransfer` for cross-domain pattern application,
//! `AestheticScorer` for evaluating ternary pattern beauty, `MutationEngine`
//! for controlled creative randomness, and `CrossDomainMapper` for translating
//! patterns between domains (e.g., music → visual).

use std::collections::HashMap;

// ── Ternary Value ──────────────────────────────────────────────────────────

/// A balanced ternary digit: Negative (-1), Zero (0), or Positive (+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ternary {
    Negative,
    Zero,
    Positive,
}

impl Ternary {
    /// Convert to integer value.
    pub fn to_i8(self) -> i8 {
        match self {
            Ternary::Negative => -1,
            Ternary::Zero => 0,
            Ternary::Positive => 1,
        }
    }

    /// Convert from integer, clamping to {-1, 0, 1}.
    pub fn from_i8(v: i8) -> Self {
        match v {
            ..=-1 => Ternary::Negative,
            0 => Ternary::Zero,
            1.. => Ternary::Positive,
        }
    }

    /// Negate the value.
    pub fn negate(self) -> Self {
        match self {
            Ternary::Negative => Ternary::Positive,
            Ternary::Zero => Ternary::Zero,
            Ternary::Positive => Ternary::Negative,
        }
    }
}

// ── Pattern ────────────────────────────────────────────────────────────────

/// A ternary pattern: a sequence of ternary values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    pub values: Vec<Ternary>,
}

impl Pattern {
    pub fn new(values: Vec<Ternary>) -> Self {
        Self { values }
    }

    /// Length of the pattern.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Is the pattern empty?
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Convert to integer vector.
    pub fn to_i8_vec(&self) -> Vec<i8> {
        self.values.iter().map(|t| t.to_i8()).collect()
    }

    /// Compute symmetry score: how well the pattern mirrors around its center.
    /// Returns 0.0 (no symmetry) to 1.0 (perfect symmetry).
    pub fn symmetry(&self) -> f64 {
        if self.values.len() < 2 {
            return 1.0;
        }
        let n = self.values.len();
        let matches = (0..n / 2)
            .filter(|&i| self.values[i] == self.values[n - 1 - i])
            .count();
        matches as f64 / (n / 2) as f64
    }

    /// Compute balance: ratio of zero values. 1.0 = all zeros.
    pub fn balance(&self) -> f64 {
        if self.values.is_empty() {
            return 1.0;
        }
        let zeros = self.values.iter().filter(|&&t| t == Ternary::Zero).count();
        zeros as f64 / self.values.len() as f64
    }

    /// Compute complexity: ratio of transitions between different values.
    pub fn complexity(&self) -> f64 {
        if self.values.len() < 2 {
            return 0.0;
        }
        let transitions = self
            .values
            .windows(2)
            .filter(|w| w[0] != w[1])
            .count();
        transitions as f64 / (self.values.len() - 1) as f64
    }
}

// ── Aesthetic Scorer ───────────────────────────────────────────────────────

/// Evaluates the aesthetic quality of a ternary pattern.
///
/// Combines symmetry, complexity, and balance into a single score.
/// Weights are configurable.
#[derive(Debug, Clone)]
pub struct AestheticScorer {
    pub symmetry_weight: f64,
    pub complexity_weight: f64,
    pub balance_weight: f64,
}

impl Default for AestheticScorer {
    fn default() -> Self {
        Self {
            symmetry_weight: 0.4,
            complexity_weight: 0.35,
            balance_weight: 0.25,
        }
    }
}

impl AestheticScorer {
    pub fn new(symmetry_weight: f64, complexity_weight: f64, balance_weight: f64) -> Self {
        Self {
            symmetry_weight,
            complexity_weight,
            balance_weight,
        }
    }

    /// Score a pattern from 0.0 to 1.0.
    pub fn score(&self, pattern: &Pattern) -> f64 {
        let total_weight = self.symmetry_weight + self.complexity_weight + self.balance_weight;
        if total_weight == 0.0 {
            return 0.0;
        }
        let raw = self.symmetry_weight * pattern.symmetry()
            + self.complexity_weight * pattern.complexity()
            + self.balance_weight * pattern.balance();
        raw / total_weight
    }
}

// ── Mutation Engine ────────────────────────────────────────────────────────

/// Controlled randomness for creative exploration of ternary patterns.
///
/// Mutates patterns by flipping, rotating, inserting, or deleting values
/// based on a mutation rate.
#[derive(Debug, Clone)]
pub struct MutationEngine {
    /// Probability of each element being mutated (0.0 to 1.0).
    pub mutation_rate: f64,
}

impl MutationEngine {
    pub fn new(mutation_rate: f64) -> Self {
        Self {
            mutation_rate: mutation_rate.clamp(0.0, 1.0),
        }
    }

    /// Deterministic flip: negate values at indices determined by seed.
    pub fn flip_mutate(&self, pattern: &Pattern, seed: usize) -> Pattern {
        let values: Vec<Ternary> = pattern
            .values
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                if self.should_mutate(i, seed) {
                    v.negate()
                } else {
                    v
                }
            })
            .collect();
        Pattern::new(values)
    }

    /// Rotate pattern by `shift` positions.
    pub fn rotate(&self, pattern: &Pattern, shift: usize) -> Pattern {
        if pattern.values.is_empty() {
            return pattern.clone();
        }
        let n = pattern.values.len();
        let s = shift % n;
        let mut values = pattern.values.clone();
        values.rotate_right(s);
        Pattern::new(values)
    }

    /// Reverse the pattern.
    pub fn reverse(&self, pattern: &Pattern) -> Pattern {
        let mut values = pattern.values.clone();
        values.reverse();
        Pattern::new(values)
    }

    /// Insert a value at a position determined by seed.
    pub fn insert_mutate(&self, pattern: &Pattern, value: Ternary, seed: usize) -> Pattern {
        let mut values = pattern.values.clone();
        if !values.is_empty() {
            let pos = seed % values.len();
            values.insert(pos, value);
        } else {
            values.push(value);
        }
        Pattern::new(values)
    }

    /// Remove a value at a position determined by seed.
    pub fn delete_mutate(&self, pattern: &Pattern, seed: usize) -> Pattern {
        let mut values = pattern.values.clone();
        if !values.is_empty() {
            let pos = seed % values.len();
            values.remove(pos);
        }
        Pattern::new(values)
    }

    /// Deterministic mutation decision based on index and seed.
    fn should_mutate(&self, index: usize, seed: usize) -> bool {
        // Simple hash: use multiplication and xor to spread bits
        let hash = (index.wrapping_mul(31)).wrapping_add(seed).wrapping_mul(2654435761);
        // Map to 0..1000 and compare against rate
        let normalized = (hash % 1000) as f64 / 1000.0;
        normalized < self.mutation_rate
    }
}

// ── Pattern Generator ──────────────────────────────────────────────────────

/// Generates novel ternary patterns using deterministic seeds.
#[derive(Debug, Clone)]
pub struct PatternGenerator {
    /// Base seed for generation.
    pub seed: usize,
}

impl PatternGenerator {
    pub fn new(seed: usize) -> Self {
        Self { seed }
    }

    /// Generate a pattern of given length using the seed.
    pub fn generate(&self, length: usize) -> Pattern {
        let values: Vec<Ternary> = (0..length)
            .map(|i| {
                let hash = (i.wrapping_mul(7919)).wrapping_add(self.seed).wrapping_mul(1103515245);
                match hash % 3 {
                    0 => Ternary::Negative,
                    1 => Ternary::Zero,
                    _ => Ternary::Positive,
                }
            })
            .collect();
        Pattern::new(values)
    }

    /// Generate a symmetric pattern.
    pub fn generate_symmetric(&self, half_length: usize) -> Pattern {
        let half: Vec<Ternary> = (0..half_length)
            .map(|i| {
                let hash = (i.wrapping_mul(6271)).wrapping_add(self.seed).wrapping_mul(2147483647);
                match hash % 3 {
                    0 => Ternary::Negative,
                    1 => Ternary::Zero,
                    _ => Ternary::Positive,
                }
            })
            .collect();
        let mut values = half.clone();
        values.extend(half.iter().rev());
        Pattern::new(values)
    }

    /// Generate a pattern with a specific ratio of each value type.
    pub fn generate_weighted(&self, length: usize, neg_weight: f64, zero_weight: f64, pos_weight: f64) -> Pattern {
        let total = neg_weight + zero_weight + pos_weight;
        if total == 0.0 {
            return Pattern::new(vec![Ternary::Zero; length]);
        }
        let neg_thresh = neg_weight / total;
        let zero_thresh = neg_thresh + zero_weight / total;
        let values: Vec<Ternary> = (0..length)
            .map(|i| {
                let hash = (i.wrapping_mul(3571)).wrapping_add(self.seed).wrapping_mul(48271);
                let normalized = (hash % 1000) as f64 / 1000.0;
                if normalized < neg_thresh {
                    Ternary::Negative
                } else if normalized < zero_thresh {
                    Ternary::Zero
                } else {
                    Ternary::Positive
                }
            })
            .collect();
        Pattern::new(values)
    }
}

// ── Style Transfer ─────────────────────────────────────────────────────────

/// Applies patterns from one domain onto another by mapping ternary values.
#[derive(Debug, Clone)]
pub struct StyleTransfer {
    /// Maps source domain patterns to target domain transformations.
    rules: HashMap<String, Ternary>,
}

impl StyleTransfer {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Add a style rule: source value → target value.
    pub fn add_rule(&mut self, source: &str, target: Ternary) {
        self.rules.insert(source.to_string(), target);
    }

    /// Apply style rules to a pattern, replacing values that match rules.
    pub fn apply(&self, pattern: &Pattern) -> Pattern {
        let values: Vec<Ternary> = pattern
            .values
            .iter()
            .map(|&v| {
                let key = format!("{:?}", v);
                match self.rules.get(&key) {
                    Some(&replacement) => replacement,
                    None => v,
                }
            })
            .collect();
        Pattern::new(values)
    }

    /// Blend two patterns by taking the element-wise average (rounded).
    pub fn blend(a: &Pattern, b: &Pattern) -> Pattern {
        let len = a.len().min(b.len());
        let values: Vec<Ternary> = (0..len)
            .map(|i| {
                let avg = (a.values[i].to_i8() as i16 + b.values[i].to_i8() as i16) / 2;
                Ternary::from_i8(avg as i8)
            })
            .collect();
        Pattern::new(values)
    }

    /// Overlay pattern b onto a at the given offset.
    pub fn overlay(a: &Pattern, b: &Pattern, offset: usize) -> Pattern {
        let mut values = a.values.clone();
        for (i, &v) in b.values.iter().enumerate() {
            let pos = offset + i;
            if pos < values.len() {
                values[pos] = v;
            }
        }
        Pattern::new(values)
    }
}

// ── Cross-Domain Mapper ────────────────────────────────────────────────────

/// Translates ternary patterns between different domains.
///
/// Domains are identified by string names. Each domain has a mapping that
/// interprets ternary values in its own context.
#[derive(Debug, Clone)]
pub struct CrossDomainMapper {
    /// Domain definitions: name → interpretation of {-1, 0, +1}.
    domains: HashMap<String, DomainMapping>,
}

/// How a domain interprets the three ternary values.
#[derive(Debug, Clone)]
pub struct DomainMapping {
    pub negative_label: String,
    pub zero_label: String,
    pub positive_label: String,
}

impl CrossDomainMapper {
    pub fn new() -> Self {
        Self {
            domains: HashMap::new(),
        }
    }

    /// Register a domain with its ternary value interpretations.
    pub fn register_domain(
        &mut self,
        name: &str,
        negative_label: &str,
        zero_label: &str,
        positive_label: &str,
    ) {
        self.domains.insert(
            name.to_string(),
            DomainMapping {
                negative_label: negative_label.to_string(),
                zero_label: zero_label.to_string(),
                positive_label: positive_label.to_string(),
            },
        );
    }

    /// Translate a pattern from one domain's interpretation to another's.
    /// The ternary values stay the same; only the labels change.
    pub fn translate(&self, pattern: &Pattern, from_domain: &str, to_domain: &str) -> Option<Vec<String>> {
        let _from = self.domains.get(from_domain)?;
        let to = self.domains.get(to_domain)?;
        Some(
            pattern
                .values
                .iter()
                .map(|&v| match v {
                    Ternary::Negative => to.negative_label.clone(),
                    Ternary::Zero => to.zero_label.clone(),
                    Ternary::Positive => to.positive_label.clone(),
                })
                .collect(),
        )
    }

    /// Get the label for a value in a specific domain.
    pub fn label_for(&self, domain: &str, value: Ternary) -> Option<&str> {
        let mapping = self.domains.get(domain)?;
        Some(match value {
            Ternary::Negative => &mapping.negative_label,
            Ternary::Zero => &mapping.zero_label,
            Ternary::Positive => &mapping.positive_label,
        })
    }

    /// List registered domain names.
    pub fn domain_names(&self) -> Vec<&str> {
        self.domains.keys().map(|s| s.as_str()).collect()
    }
}

// ── Muse ───────────────────────────────────────────────────────────────────

/// The central creativity engine.
///
/// Combines pattern generation, mutation, style transfer, and aesthetic
/// scoring into a single creative workflow.
#[derive(Debug, Clone)]
pub struct Muse {
    pub generator: PatternGenerator,
    pub mutator: MutationEngine,
    pub scorer: AestheticScorer,
    pub style: StyleTransfer,
}

impl Muse {
    pub fn new(seed: usize, mutation_rate: f64) -> Self {
        Self {
            generator: PatternGenerator::new(seed),
            mutator: MutationEngine::new(mutation_rate),
            scorer: AestheticScorer::default(),
            style: StyleTransfer::new(),
        }
    }

    /// Create a muse with custom aesthetic weights.
    pub fn with_scoring(seed: usize, mutation_rate: f64, sym_w: f64, comp_w: f64, bal_w: f64) -> Self {
        Self {
            generator: PatternGenerator::new(seed),
            mutator: MutationEngine::new(mutation_rate),
            scorer: AestheticScorer::new(sym_w, comp_w, bal_w),
            style: StyleTransfer::new(),
        }
    }

    /// Generate a base pattern and evolve it through mutation cycles.
    /// Returns the best-scoring variant found.
    pub fn create_and_evolve(&self, length: usize, generations: usize) -> Pattern {
        let base = self.generator.generate(length);
        let mut best = base.clone();
        let mut best_score = self.scorer.score(&best);

        for gen in 0..generations {
            let candidate = self.mutator.flip_mutate(&best, gen);
            let candidate_score = self.scorer.score(&candidate);
            if candidate_score > best_score {
                best = candidate;
                best_score = candidate_score;
            }
        }
        best
    }

    /// Generate multiple creative variants from a seed pattern.
    pub fn variants(&self, base: &Pattern, count: usize) -> Vec<Pattern> {
        (0..count)
            .map(|i| {
                let rotated = self.mutator.rotate(base, i);
                self.mutator.flip_mutate(&rotated, i)
            })
            .collect()
    }

    /// Score a pattern using the internal scorer.
    pub fn score(&self, pattern: &Pattern) -> f64 {
        self.scorer.score(pattern)
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_to_i8() {
        assert_eq!(Ternary::Negative.to_i8(), -1);
        assert_eq!(Ternary::Zero.to_i8(), 0);
        assert_eq!(Ternary::Positive.to_i8(), 1);
    }

    #[test]
    fn test_ternary_from_i8() {
        assert_eq!(Ternary::from_i8(-5), Ternary::Negative);
        assert_eq!(Ternary::from_i8(0), Ternary::Zero);
        assert_eq!(Ternary::from_i8(3), Ternary::Positive);
    }

    #[test]
    fn test_ternary_negate() {
        assert_eq!(Ternary::Negative.negate(), Ternary::Positive);
        assert_eq!(Ternary::Zero.negate(), Ternary::Zero);
        assert_eq!(Ternary::Positive.negate(), Ternary::Negative);
    }

    #[test]
    fn test_pattern_symmetry_perfect() {
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Zero, Ternary::Positive]);
        assert!((p.symmetry() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_pattern_symmetry_none() {
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Negative]);
        assert!((p.symmetry() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_pattern_balance() {
        let p = Pattern::new(vec![Ternary::Zero, Ternary::Positive, Ternary::Zero, Ternary::Negative]);
        assert!((p.balance() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_pattern_complexity_max() {
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Negative, Ternary::Positive, Ternary::Negative]);
        assert!((p.complexity() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_aesthetic_scorer() {
        let scorer = AestheticScorer::default();
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Zero, Ternary::Positive]);
        let score = scorer.score(&p);
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_aesthetic_scorer_custom_weights() {
        let scorer = AestheticScorer::new(1.0, 0.0, 0.0);
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Zero, Ternary::Positive]);
        assert!((scorer.score(&p) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_mutation_flip() {
        let engine = MutationEngine::new(1.0); // 100% mutation
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Zero]);
        let mutated = engine.flip_mutate(&p, 42);
        assert_eq!(mutated.values[0], Ternary::Negative); // always flipped at 100%
    }

    #[test]
    fn test_mutation_rotate() {
        let engine = MutationEngine::new(0.5);
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Zero, Ternary::Negative]);
        let rotated = engine.rotate(&p, 1);
        assert_eq!(rotated.values[0], Ternary::Negative);
        assert_eq!(rotated.values[1], Ternary::Positive);
    }

    #[test]
    fn test_mutation_reverse() {
        let engine = MutationEngine::new(0.5);
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Zero, Ternary::Negative]);
        let reversed = engine.reverse(&p);
        assert_eq!(reversed.values[0], Ternary::Negative);
        assert_eq!(reversed.values[2], Ternary::Positive);
    }

    #[test]
    fn test_mutation_insert() {
        let engine = MutationEngine::new(0.5);
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Negative]);
        let inserted = engine.insert_mutate(&p, Ternary::Zero, 0);
        assert_eq!(inserted.len(), 3);
    }

    #[test]
    fn test_mutation_delete() {
        let engine = MutationEngine::new(0.5);
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Negative]);
        let deleted = engine.delete_mutate(&p, 0);
        assert_eq!(deleted.len(), 1);
    }

    #[test]
    fn test_pattern_generator_deterministic() {
        let gen = PatternGenerator::new(42);
        let a = gen.generate(10);
        let b = gen.generate(10);
        assert_eq!(a, b);
    }

    #[test]
    fn test_pattern_generator_symmetric() {
        let gen = PatternGenerator::new(7);
        let p = gen.generate_symmetric(3);
        assert_eq!(p.len(), 6);
        assert!((p.symmetry() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_pattern_generator_weighted() {
        let gen = PatternGenerator::new(99);
        let p = gen.generate_weighted(20, 1.0, 0.0, 0.0);
        assert!(p.values.iter().all(|&v| v == Ternary::Negative));
    }

    #[test]
    fn test_style_transfer_blend() {
        let a = Pattern::new(vec![Ternary::Positive, Ternary::Negative]);
        let b = Pattern::new(vec![Ternary::Negative, Ternary::Positive]);
        let blended = StyleTransfer::blend(&a, &b);
        assert_eq!(blended.values[0], Ternary::Zero);
        assert_eq!(blended.values[1], Ternary::Zero);
    }

    #[test]
    fn test_style_transfer_overlay() {
        let a = Pattern::new(vec![Ternary::Zero, Ternary::Zero, Ternary::Zero, Ternary::Zero]);
        let b = Pattern::new(vec![Ternary::Positive, Ternary::Negative]);
        let overlaid = StyleTransfer::overlay(&a, &b, 1);
        assert_eq!(overlaid.values[0], Ternary::Zero);
        assert_eq!(overlaid.values[1], Ternary::Positive);
        assert_eq!(overlaid.values[2], Ternary::Negative);
    }

    #[test]
    fn test_cross_domain_mapper() {
        let mut mapper = CrossDomainMapper::new();
        mapper.register_domain("music", "flat", "natural", "sharp");
        mapper.register_domain("visual", "dark", "neutral", "bright");
        let p = Pattern::new(vec![Ternary::Positive, Ternary::Zero, Ternary::Negative]);
        let translated = mapper.translate(&p, "music", "visual").unwrap();
        assert_eq!(translated, vec!["bright", "neutral", "dark"]);
    }

    #[test]
    fn test_cross_domain_mapper_label_for() {
        let mut mapper = CrossDomainMapper::new();
        mapper.register_domain("spatial", "left", "center", "right");
        assert_eq!(mapper.label_for("spatial", Ternary::Positive), Some("right"));
        assert_eq!(mapper.label_for("nonexistent", Ternary::Positive), None);
    }

    #[test]
    fn test_muse_create_and_evolve() {
        let muse = Muse::new(42, 0.3);
        let result = muse.create_and_evolve(8, 10);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_muse_variants() {
        let muse = Muse::new(42, 0.5);
        let base = Pattern::new(vec![Ternary::Positive, Ternary::Zero, Ternary::Negative]);
        let variants = muse.variants(&base, 5);
        assert_eq!(variants.len(), 5);
        for v in &variants {
            assert_eq!(v.len(), 3);
        }
    }

    #[test]
    fn test_muse_scoring_integration() {
        let muse = Muse::new(1, 0.2);
        let p = muse.generator.generate(6);
        let score = muse.score(&p);
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_empty_pattern_metrics() {
        let p = Pattern::new(vec![]);
        assert!(p.is_empty());
        assert!((p.symmetry() - 1.0).abs() < 1e-9);
        assert!((p.balance() - 1.0).abs() < 1e-9);
        assert!((p.complexity() - 0.0).abs() < 1e-9);
    }
}
