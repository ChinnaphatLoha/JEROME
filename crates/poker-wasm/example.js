// ────────────────────────────────────────────────────────────────────
// JEROME WebAssembly — Example Usage (JavaScript / ES Module)
// ────────────────────────────────────────────────────────────────────
//
// Build the package:
//   wasm-pack build crates/poker-wasm --target web --out-dir pkg
//
// Run this example with Node.js 18+:
//   node --experimental-wasm-modules example.js
//
// Or import in a browser via <script type="module">.
// ────────────────────────────────────────────────────────────────────

import init, { analyze } from "./pkg/jerome_poker_wasm.js";

async function main() {
  // 1. Initialize the WASM module (required before calling analyze)
  await init();

  console.log("♠️  JEROME Poker Engine — WASM Analysis\n");

  // 2. Analyze a flop scenario
  try {
    const result = analyze({
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
    });

    // 3. Read the result
    console.log("--- Decision Result ---");
    console.log("Recommended Action:", result.recommended_action);
    console.log(
      "Estimated Equity:  ",
      (result.estimated_equity * 100).toFixed(1) + "%"
    );
    console.log("Required Equity:   ", (result.required_equity * 100).toFixed(1) + "%");
    console.log("Estimated EV:      ", result.estimated_ev.toFixed(2));

    console.log("\nAlternatives (sorted by EV):");
    for (const alt of result.alternatives) {
      const size = alt.action.amount ? ` ${alt.action.amount}` : "";
      console.log(`  ${alt.label} (${alt.action.type}${size}) → EV: ${alt.ev.toFixed(2)}`);
    }

    console.log("\nExplanation Factors:");
    for (const factor of result.explanation.factors) {
      console.log(`  [${factor.factor}] ${factor.description}`);
    }
  } catch (error) {
    // 4. Handle errors — errors are JavaScript Error objects with a `code` property
    console.error("Analysis failed:", error.message);
    if (error.code) {
      console.error("Error code:", error.code);
      // error.code is one of:
      //   "INVALID_CARD"           — bad card string
      //   "INVALID_ENUM"           — bad street/position/status
      //   "DESERIALIZATION_ERROR"   — malformed input object
      //   "ENGINE_ERROR"           — invalid game state
    }
  }

  // 5. Demonstrate error handling
  console.log("\n--- Error Handling Demo ---");
  try {
    analyze({
      hero_cards: ["Xx", "Ah"], // "Xx" is not a valid card
      board: [],
      street: "preflop",
      pot: 3,
      current_bet: 2,
      big_blind: 2,
      hero_index: 0,
      players: [
        { id: 0, position: "UTG", stack: 998, status: "active", bet_this_round: 0 },
        { id: 1, position: "BTN", stack: 998, status: "active", bet_this_round: 0 },
      ],
    });
  } catch (error) {
    console.log(`Caught error: ${error.message}`);
    console.log(`Error code:   ${error.code}`); // "INVALID_CARD"
  }
}

main();
