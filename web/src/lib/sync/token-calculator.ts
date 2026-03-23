interface ModelPricing {
  input: number; // per 1M tokens
  output: number;
  cacheWrite: number;
  cacheRead: number;
}

const MODEL_PRICING: Record<string, ModelPricing> = {
  "claude-opus-4-6": {
    input: 15.0,
    output: 75.0,
    cacheWrite: 18.75,
    cacheRead: 1.5,
  },
  "claude-sonnet-4-5-20250514": {
    input: 3.0,
    output: 15.0,
    cacheWrite: 3.75,
    cacheRead: 0.3,
  },
  "claude-sonnet-4-6": {
    input: 3.0,
    output: 15.0,
    cacheWrite: 3.75,
    cacheRead: 0.3,
  },
  "claude-haiku-4-5-20251001": {
    input: 0.8,
    output: 4.0,
    cacheWrite: 1.0,
    cacheRead: 0.08,
  },
};

const DEFAULT_PRICING: ModelPricing = MODEL_PRICING["claude-sonnet-4-5-20250514"];

function findPricing(model: string): ModelPricing {
  if (MODEL_PRICING[model]) return MODEL_PRICING[model];
  // Fuzzy match: check if model starts with a known prefix
  for (const [key, pricing] of Object.entries(MODEL_PRICING)) {
    if (model.startsWith(key.split("-").slice(0, 3).join("-"))) {
      return pricing;
    }
  }
  return DEFAULT_PRICING;
}

export function estimateCost(
  model: string,
  inputTokens: number,
  outputTokens: number,
  cacheCreationTokens: number,
  cacheReadTokens: number
): number {
  const pricing = findPricing(model);
  return (
    (inputTokens / 1_000_000) * pricing.input +
    (outputTokens / 1_000_000) * pricing.output +
    (cacheCreationTokens / 1_000_000) * pricing.cacheWrite +
    (cacheReadTokens / 1_000_000) * pricing.cacheRead
  );
}
