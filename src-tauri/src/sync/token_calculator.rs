struct ModelPricing {
    input: f64,
    output: f64,
    cache_write: f64,
    cache_read: f64,
}

const PRICING: &[(&str, ModelPricing)] = &[
    (
        "claude-opus-4-6",
        ModelPricing {
            input: 15.0,
            output: 75.0,
            cache_write: 18.75,
            cache_read: 1.5,
        },
    ),
    (
        "claude-sonnet-4-5-20250514",
        ModelPricing {
            input: 3.0,
            output: 15.0,
            cache_write: 3.75,
            cache_read: 0.3,
        },
    ),
    (
        "claude-sonnet-4-6",
        ModelPricing {
            input: 3.0,
            output: 15.0,
            cache_write: 3.75,
            cache_read: 0.3,
        },
    ),
    (
        "claude-haiku-4-5-20251001",
        ModelPricing {
            input: 0.8,
            output: 4.0,
            cache_write: 1.0,
            cache_read: 0.08,
        },
    ),
];

fn find_pricing(model: &str) -> &ModelPricing {
    // Exact match
    for (key, pricing) in PRICING {
        if model == *key {
            return pricing;
        }
    }
    // Fuzzy match: check if model starts with first 3 segments of a known key
    for (key, pricing) in PRICING {
        let prefix: String = key.splitn(4, '-').take(3).collect::<Vec<_>>().join("-");
        if model.starts_with(&prefix) {
            return pricing;
        }
    }
    // Default to sonnet pricing
    &PRICING[1].1
}

pub fn estimate_cost(
    model: &str,
    input_tokens: i64,
    output_tokens: i64,
    cache_creation_tokens: i64,
    cache_read_tokens: i64,
) -> f64 {
    let pricing = find_pricing(model);
    (input_tokens as f64 / 1_000_000.0) * pricing.input
        + (output_tokens as f64 / 1_000_000.0) * pricing.output
        + (cache_creation_tokens as f64 / 1_000_000.0) * pricing.cache_write
        + (cache_read_tokens as f64 / 1_000_000.0) * pricing.cache_read
}
