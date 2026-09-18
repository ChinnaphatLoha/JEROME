// ────────────────────────────────────────────────────────────────────
// JEROME WebAssembly — Example Usage (TypeScript)
// ────────────────────────────────────────────────────────────────────
//
// This example demonstrates typed usage of the jerome-poker-wasm package.
// Types are provided by the accompanying .d.ts declarations.
//
// Build the package:
//   wasm-pack build crates/poker-wasm --target web --out-dir pkg
// ────────────────────────────────────────────────────────────────────

import init, {
  analyze,
  type AnalysisInput,
  type AnalysisResult,
  type AnalysisError,
} from "./pkg/jerome_poker_wasm.js";

async function main(): Promise<void> {
  // 1. Initialize the WASM module
  await init();

  // 2. Build a typed input
  const input: AnalysisInput = {
    hero_cards: ["As", "Kh"],
    board: ["Qs", "Th", "5d"],
    street: "flop",
    pot: 120,
    current_bet: 40,
    big_blind: 2,
    min_raise: 40,
    hero_index: 0,
    players: [
      {
        id: 0,
        position: "BTN",
        stack: 980,
        status: "active",
        bet_this_round: 0,
      },
      {
        id: 1,
        position: "BB",
        stack: 960,
        status: "active",
        bet_this_round: 40,
      },
    ],
    config: {
      mc_samples: 5000,
      seed: 42,
    },
  };

  // 3. Run the analysis — result is fully typed
  try {
    const result: AnalysisResult = analyze(input);

    console.log("Recommended:", result.recommended_action.type);
    console.log(`Equity: ${(result.estimated_equity * 100).toFixed(1)}%`);
    console.log(`EV: ${result.estimated_ev.toFixed(2)}`);

    // Access alternatives with full type safety
    for (const alt of result.alternatives) {
      console.log(`  ${alt.label}: EV ${alt.ev.toFixed(2)}`);
    }

    // Access explanation factors
    for (const { factor, description } of result.explanation.factors) {
      console.log(`  [${factor}] ${description}`);
    }
  } catch (e: unknown) {
    // 4. Handle errors with typed error codes
    if (e instanceof Error && "code" in e) {
      const err = e as AnalysisError;
      switch (err.code) {
        case "INVALID_CARD":
          console.error("Bad card:", err.message);
          break;
        case "INVALID_ENUM":
          console.error("Bad enum value:", err.message);
          break;
        case "DESERIALIZATION_ERROR":
          console.error("Malformed input:", err.message);
          break;
        case "ENGINE_ERROR":
          console.error("Engine error:", err.message);
          break;
      }
    }
  }
}

main();
