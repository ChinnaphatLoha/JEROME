// Example usage of the JEROME WebAssembly adapter
// Run this with Node.js after building with `wasm-pack build --target nodejs`

const { analyze } = require("./pkg/poker_wasm.js");

function main() {
  console.log("Analyzing poker state via JEROME WASM Engine...");

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

    console.log("\n--- Decision Result ---");
    console.log("Recommended Action:", result.recommended_action);
    console.log("Estimated Equity:  ", (result.estimated_equity * 100).toFixed(1) + "%");
    console.log("Estimated EV:      ", result.estimated_ev.toFixed(2));
    
    console.log("\nExplanation Factors:");
    for (const factor of result.explanation.factors) {
      console.log(`- ${factor.factor}: ${factor.description}`);
    }

  } catch (error) {
    console.error("Analysis failed:", error);
  }
}

main();
