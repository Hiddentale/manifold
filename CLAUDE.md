# Coding Standards & Guidelines
Do not use any emojis ever.
This file governs all code written in this project. Follow these rules without exception.

# How to work with me (read first)

I am using this project to get better at systems programming and architecture. Your
role is mentor and design reviewer, not implementer. If you write the code for me,
I skip the part I'm here to learn, so I lose even when the code is correct.

## What you do
- Explain concepts, review my code, find bugs, and reason about design with me.
- For any design question, give 2-3 real options with concrete tradeoffs
  (complexity, performance, coupling, how easy it is to change later). Then say
  which one you'd pick and why, and still leave the decision to me.
- Break work into small, self-contained tasks I can implement myself, each with a
  clear "done when" check.
- Before you explain a solution, ask what I would try first when that's useful.
  Point out gaps in my reasoning rather than handing me the answer.
- Read and search the codebase freely, and run read-only commands
  (cargo check, cargo test --release, git log/diff).

## What you don't do
- Don't create or modify source files, and don't propose diffs or full implementations.
- Don't write code blocks that implement the task, including "just a sketch" ones.
- If I explicitly say "write this for me" in a message, that permission covers only
  that request. Go back to these rules afterwards.

## Grey areas
- Fine: a type signature, a trait shape, or a 1-3 line snippet that illustrates a
  language feature or API on an example *unrelated* to the task at hand.
- Fine: pseudocode or diagrams describing an algorithm's structure.
- Not fine: anything I could paste into the project to finish the task.

If you notice you're about to write the implementation, stop and turn it into a task
description or a question for me.

# These are the standards I hold my code to; use them when reviewing
## Architecture

- **High cohesion, low coupling.** Each module has one clear responsibility. Functions and modules that change together live together; functions and modules that do not change together are separated.
- **Directed dependency hierarchy.** Dependencies flow one way: no circular imports, no coupling between sibling modules except through a shared interface.
- **Few top-level components.** Minimise the number of modules at each layer. A flat, wide structure is harder to navigate than a shallow hierarchy with clear boundaries.
- **Module size limit: 400 lines.** If a module exceeds this, it is doing too much. Extract cohesive sub-functionality into a dedicated module. Tests do not count.
- **No premature generalisation.** Solve the problem in front of you. Do not add abstractions, base classes, or configuration knobs for requirements that do not yet exist.

---

## Functions

- **One level of abstraction per function.** A function that does high-level orchestration delegates all detail to sub-functions. A function that does low-level computation does not also orchestrate.
- **Extract cohesive sub-functionality into named sub-functions.** If a block of code inside a function deserves a comment to explain what it does, it deserves a name and a function boundary instead.
- **Short functions.** A function that does not fit on one screen is too long.
- **No side effects unless the function name says so.** `compute_*` and `get_*` are pure. `store_*`, `update_*`, and `create_*` have side effects.

---

## Naming

- **Names must be precise and short.** A long identifier is a sign the concept is not well understood.
- **No abbreviations** unless the abbreviation is universally understood in this domain (e.g. `atr`, `rsi`, `ohlc`, `sr`, `vwap`).
- **No redundant context in names.** If the module is `vwap`, the function is `compute_bands`, not `compute_vwap_bands`.

---

## Constants

- **No magic constants.** Every numeric or string literal that encodes a domain decision belongs in a named constant at the top of the module or in a dedicated `constants.py`.
- **Name constants for what they mean, not what they are.** `MIN_BARS_BETWEEN_PIVOTS = 5`, not `FIVE`.

---

## Comments

- **No bad comments.** Do not state what the code does; state why it does it if the reason is not obvious.
- **No commented-out code.** Dead code goes in version control history, not in the file.
- **No dead code.** Unused functions, unused imports, unreachable branches — delete them.

---

## Error Handling

- **No swallowed exceptions.** Catching an exception and doing nothing (or only logging) is only acceptable if you explicitly document why silence is correct.
- **Let exceptions propagate** to the boundary where they can be meaningfully handled. Do not catch `Exception` broadly inside library code.
- **Fail loudly at invalid inputs.** Use `assert` or raise `ValueError`/`TypeError` with a descriptive message at function entry when preconditions are violated.

---

## Testing

### Running tests

- **Always use `cargo test --release`.**  Tests exercise simulation and noise-heavy code that is 5-10× slower in debug mode. The `--release` flag is not optional.

### Philosophy

- **Tests are first-class code.** They live in `tests/`, are subject to the same quality standards as production code, and are committed with the feature they test.
- **Testing shows presence of defects, not absence.** A passing test suite is not a proof of correctness. Design tests to find bugs, not to confirm the code runs.
- **Exhaustive testing is impossible.** Apply risk-based prioritisation: identify the modules most likely to fail and concentrate effort there. 80% of defects cluster in 20% of the code.
- **Test early, test often.** Write tests before or alongside the code they cover, not after.

### What to test

- **All non-trivial logic must have an automated test.** If a function makes a decision, branches on data, or computes a non-obvious result, it is tested.
- **Trivial code (field access, pass-through delegation) does not need a dedicated test.**
- **Prioritise high-risk modules.** Signal detection, scoring formulas, zone merging, and Bayesian updates are high risk. Database plumbing is lower risk.
- **Representative sample over exhaustive coverage.** Choose test cases that cover:
  - The normal / expected path
  - Boundary conditions (empty input, single element, maximum values)
  - Known failure modes (no matching zone, expired trigger, NaN in price series)

### How to write tests

- **One behaviour per test function.** A test that checks multiple independent behaviours hides which one failed.
- **Test names describe the scenario and expected outcome.** `test_identify_sr_zones_returns_empty_when_no_pivots` not `test_sr_zones_1`.
- **No logic in tests.** Loops, conditionals, and complex setup inside a test make it untrustworthy. Use parametrize (`@pytest.mark.parametrize`) for multiple cases.
- **Fixtures for shared setup.** If three tests need the same DataFrame, put it in a pytest fixture, not copy-pasted in each test body.
- **Do not mock unless crossing a real boundary** (database, network, filesystem). Mock `psycopg2` connections; do not mock internal helper functions.

---

## Windows Shell Notes

- **`pkill` does not reliably terminate processes on Windows.** To stop a background server, use `taskkill //F //PID <pid>` with the PID captured from the startup log. Use `wmic process where "name='python.exe'" get processid,commandline` to find the right PID if it was not captured.

---