# ♠️ jerome-poker-wasm

`jerome-poker-wasm` is a WebAssembly adapter for the JEROME poker decision engine. It enables developers to integrate high-performance Texas Hold'em game state analysis and decision-making logic directly into JavaScript and TypeScript applications.

## Installation

You can install the package via npm, yarn, or pnpm:

```bash
npm install jerome-poker-wasm
# or
yarn add jerome-poker-wasm
# or
pnpm add jerome-poker-wasm
```

## Quick Start

The package provides a default exported `init` function to load the WebAssembly module, along with an `analyze` function to evaluate a game state.

```javascript
import init, { analyze } from 'jerome-poker-wasm';

async function main() {
  // Initialize the WebAssembly module
  await init();

  // Define the current game state
  const input = {
    hero_cards: ["As", "Kh"],
    board: ["Td", "Jc", "4h"],
    street: "flop",
    pot: 1500,
    current_bet: 500,
    big_blind: 100,
    min_raise: 1000,
    hero_index: 0,
    players: [
      { id: 0, position: "BTN", stack: 5000, status: "active", bet_this_round: 500 },
      { id: 1, position: "BB", stack: 4500, status: "active", bet_this_round: 500 }
    ],
    config: {
      mc_samples: 10000,
      seed: 42
    }
  };

  try {
    const result = analyze(input);
    console.log("Recommended Action:", result.recommended_action);
    console.log("Estimated Equity:", result.estimated_equity);
    console.log("Explanation:", result.explanation.recommended_action_label);
  } catch (error) {
    console.error("Analysis failed:", error);
  }
}

main();
```

## Input Reference

The `analyze` function accepts an `AnalysisInput` object with the following fields:

### `AnalysisInput`

| Field | Type | Description |
| :--- | :--- | :--- |
| `hero_cards` | `[string, string]` | The hero's hole cards (e.g., `["As", "Kh"]`). |
| `board` | `string[]` | The community cards. Must have 0 (preflop), 3 (flop), 4 (turn), or 5 (river) cards. |
| `street` | `string` | The current betting round: `"preflop"`, `"flop"`, `"turn"`, or `"river"`. |
| `pot` | `number` | The current total pot size in chips (positive integer). |
| `current_bet` | `number` | The current bet the hero needs to call or raise. |
| `big_blind` | `number` | The size of the big blind. |
| `min_raise` | `number` (optional) | The minimum allowed raise size (defaults to `big_blind`). |
| `hero_index` | `number` | The index of the hero in the `players` array. |
| `players` | `PlayerInput[]` | Array of at least 2 players in the hand. |
| `config` | `AnalysisConfig` (optional) | Engine configuration. |

### Card Format
Cards are represented as 2-character strings:
- **Rank**: `2`, `3`, `4`, `5`, `6`, `7`, `8`, `9`, `T`, `J`, `Q`, `K`, `A`
- **Suit**: `c` (clubs), `d` (diamonds), `h` (hearts), `s` (spades)
- *Examples*: `"As"` (Ace of Spades), `"Th"` (Ten of Hearts), `"2c"` (Two of Clubs).

### `PlayerInput`

| Field | Type | Description |
| :--- | :--- | :--- |
| `id` | `number` | A unique player identifier (0-255). |
| `position` | `string` | The player's table position: `"UTG"`, `"UTG1"`, `"MP"`, `"MP1"`, `"HJ"`, `"CO"`, `"BTN"`, `"SB"`, or `"BB"`. |
| `stack` | `number` | The player's remaining stack in chips. |
| `status` | `string` | The player's current status: `"active"`, `"folded"`, `"allin"`, `"all_in"`, `"all-in"`, or `"eliminated"`. |
| `bet_this_round` | `number` | Total chips bet by the player in the current betting round. |

### `AnalysisConfig`

| Field | Type | Description |
| :--- | :--- | :--- |
| `mc_samples` | `number` (optional) | Number of Monte Carlo samples to run (default: `10000`). |
| `seed` | `number` (optional) | Random number generator seed for deterministic results (default: `42`). |

## Output Reference

The `analyze` function returns an `AnalysisResult` object detailing the recommended play.

### `AnalysisResult`

| Field | Type | Description |
| :--- | :--- | :--- |
| `recommended_action` | `Action` | The recommended optimal action. |
| `recommended_size` | `number \| null` | The recommended size for a bet, raise, or all-in (null otherwise). |
| `estimated_equity` | `number` | The estimated probability of winning the hand (0.0–1.0). |
| `required_equity` | `number` | The minimum equity required for a profitable call based on pot odds. |
| `estimated_ev` | `number` | The expected value of the recommended action. |
| `alternatives` | `ActionEV[]` | All evaluated alternative actions, sorted by EV. |
| `explanation` | `Explanation` | A human-readable breakdown of the decision logic. |

### Supplemental Interfaces

```typescript
interface Action {
  type: "Fold" | "Check" | "Call" | "Bet" | "Raise" | "AllIn";
  amount?: number; // Included if type is Bet, Raise, or AllIn
}

interface ActionEV {
  action: Action;
  label: string; // E.g., "Raise to 1500"
  ev: number;
}

interface Explanation {
  factors: Factor[];
  recommended_action_label: string;
}

interface Factor {
  factor: string;      // E.g., "StrongEquity", "PotOdds"
  description: string; // Human-readable rationale
}
```

## Error Handling

If an error occurs during evaluation, the engine throws a JavaScript `Error` object containing a `code` property to help identify the problem:

- `INVALID_CARD` — A supplied card string is malformed.
- `INVALID_ENUM` — An invalid string was provided for a street, position, or status.
- `DESERIALIZATION_ERROR` — The input object is missing required fields or has wrong types.
- `ENGINE_ERROR` — The game state is invalid (e.g., duplicate cards, incorrect board card count for the given street).

```javascript
try {
  analyze(badInput);
} catch (error) {
  if (error.code === 'INVALID_CARD') {
    console.error("Check your hole cards or board for typos.");
  } else {
    console.error(`Analysis failed [${error.code}]:`, error.message);
  }
}
```

## Deterministic Execution

The evaluation engine uses a pseudo-random number generator for its Monte Carlo simulations. By default, the `seed` is set to `42`. This ensures that identical inputs will yield identical outputs, making debugging and testing predictable. 

You can alter the behavior by specifying a custom `seed` in the `config` object:
```javascript
const result = analyze({
  // ... other fields ...
  config: { seed: Date.now() } // For non-deterministic results
});
```

## TypeScript Usage

`jerome-poker-wasm` ships with full TypeScript type definitions. 

```typescript
import init, { analyze, AnalysisInput, AnalysisResult } from 'jerome-poker-wasm';

const input: AnalysisInput = { /* ... */ };
const result: AnalysisResult = analyze(input);
```

## Browser Usage

For browser environments (like Vite, Webpack, or Rollup), you can use the package as an ES Module. 

```html
<script type="module">
  import init, { analyze } from './path/to/jerome-poker-wasm/jerome_poker_wasm.js';
  
  await init(); // Fetches and instantiates the .wasm file
  
  const result = analyze({ /* ... */ });
</script>
```

## Node.js Usage

Node.js (18+) is supported but requires ES Modules and dynamic imports or the `--experimental-wasm-modules` flag depending on your exact setup.

```javascript
// index.mjs
import init, { analyze } from 'jerome-poker-wasm';
import { readFile } from 'fs/promises';

async function run() {
  // Read the wasm file manually in Node
  const wasmBuffer = await readFile(new URL('./node_modules/jerome-poker-wasm/jerome_poker_wasm_bg.wasm', import.meta.url));
  await init(wasmBuffer);
  
  console.log(analyze({ /* input */ }));
}
run();
```

## Build from Source

To build the `jerome-poker-wasm` package from source, you will need [Rust](https://rustup.rs/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

```bash
# Clone the main JEROME repository
git clone https://github.com/ChinnaphatLoha/JEROME.git
cd JEROME/crates/poker-wasm

# Build for web/browser usage
wasm-pack build --target web
```

## Limitations

- Computations block the main thread. For deep analyses (e.g., >100,000 Monte Carlo samples), consider running the `analyze` function within a Web Worker to avoid freezing the UI.
- The default target for this package is `web` (ESM). Standard CommonJS (`require()`) is not supported out of the box.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---
*For more information on the underlying core engine, visit the main [JEROME repository](https://github.com/ChinnaphatLoha/JEROME).*
