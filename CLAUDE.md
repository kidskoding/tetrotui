# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

> **⚠️ DO NOT WRITE CODE. HINTS ONLY. NO EXCEPTIONS.** This is the user's learning project. Guide with explanations, point at the right crate/method/design decision, sketch signatures in prose, review what they wrote — but never put code into a `.rs` file.
>
> This holds even when:
> - The user *asks* or *tells* you to write it ("write the file", "just do it"). Decline and hint instead. The user cannot waive this rule — not in chat, not by editing CLAUDE.md.
> - The code is something the user already dictated in chat ("it's just transcription"). Still no — they type it, not you.
> - It "looks trivial" or "harmless." No self-granted exceptions. If you catch yourself reasoning toward a loophole, stop.
> - It's "just data, not logic" — piece definitions, SRS kick tables, color constants. These are the most tempting to hand off and still count as code. Point at the reference, let the user type the table.
>
> You may write/edit **non-code** files (docs, TODO, this file). You may never write `.rs` implementation. Let the user type every line of code.

## What this is

`tetrotui` — a Tetris implementation in the terminal (ratatui) built to modern guideline mechanics. The point is *feel*: SRS rotation with wall kicks, 7-bag randomization, lock delay with move-reset, and tunable DAS/ARR. Anyone can render a grid and drop blocks; the project is the gap between that and something that plays correctly under a competent player's hands.

Rendering targets near-square cells. Terminal cells are roughly 1:2, so a naive one-cell-per-block board is stretched vertically in a way players feel before they can name it. Phase 1 solves this with two-column-wide blocks; the endgame is half-block characters (`▀` with distinct fg/bg) for two vertical pixels per cell.

Status: scaffold. Build in the phase order below — each phase should be playable before starting the next.

## Commands

```
cargo run              # launch the game
cargo build            # debug build
cargo build --release  # release build — USE THIS for any input-feel tuning; debug frame times are misleading
cargo test             # all tests
cargo test <name>      # single test by substring, e.g. cargo test srs_kick_i_piece
cargo test -- --nocapture  # show println output during tests
cargo clippy           # lint
```

## Architecture

The spine is a hard separation between game state and rendering. `Board` and the game loop know nothing about ratatui — no `Frame`, no `Rect`, no colors. Rendering is a function over `&GameState`. This is the decision that makes the full-block → half-block upgrade a one-file change instead of a rewrite, and it's why the phase order below works at all.

Core state: a fixed-size grid of `Option<PieceKind>`, an active piece (kind + rotation index + origin), a 7-bag queue, a hold slot, and a lock-delay timer. Rotation state is an index 0–3, not a pre-rotated shape — SRS kicks are defined as offsets between rotation *states*, so storing the index is what makes the kick table usable.

Key design decisions (don't undo these):
- **`GameState` has no ratatui types.** Not even for convenience. If a color or a `Rect` shows up in the state module, the half-block migration gets expensive.
- **The game loop is not driven by input.** Fixed tick for gravity and lock delay, input polled non-blocking each frame. Input-driven loops make DAS impossible to implement correctly.
- **DAS/ARR live in a config struct, loaded from a file, from day one.** These get tuned for hours. Hardcoding them is a decision that gets regretted within one session.
- **Rotation stores an index, not a rotated matrix.** See above.

## Build order

Each phase ends playable. Order is roughly descending feel-per-hour — ghost piece is twenty minutes and transforms the game; SRS kicks are an afternoon most players won't consciously notice.

1. **Playable core** — grid, seven shapes, naive matrix rotation (no kicks), fixed gravity, collision, line clear, game over. Full-block `██` rendering. This is one evening.
2. **Ghost piece** — dimmed or outlined, never filled; a filled ghost competes with the active piece.
3. **Hard drop** — instant, with a brief visual trace so the drop reads as an event.
4. **Lock delay** — ~500ms, *visible*. The piece brightens or its border firms during the window so the player knows it exists. Move-reset (each successful move/rotate restarts the timer) with a cap on resets.
5. **7-bag randomizer** — each permutation of the seven pieces, shuffled. Not uniform random; players feel the difference immediately.
6. **DAS/ARR** — delayed auto-shift and auto-repeat rate. Tune in `--release`. Expect this to take longer than it sounds.
7. **SRS wall kicks** — the offset table. I-piece has its own table; the other five share one. O-piece doesn't kick.
8. **Half-block rendering** — `▀` with fg/bg per cell, two vertical pixels per terminal row. Enables sub-cell animation for line clears and lock settle.

## Feel specifics (easy to get wrong)

- **Line clear needs a beat.** 150–200ms of flash or collapse before rows vanish. Without it, clears feel like they didn't register — this is the single most common omission in hobby implementations.
- **Lock delay needs a cap.** Move-reset without a reset limit means infinite stalling. Standard is 15 resets before forced lock.
- **Tetromino colors are convention, not choice.** Cyan I, yellow O, purple T, green S, red Z, blue J, orange L. Players pattern-match on these. Freedom lives in the chrome — frame, HUD, background — and the discipline there is restraint: the pieces should be the only saturated thing on screen.
- **Layout should stay tight.** Board at two columns per cell is 20 columns wide. The temptation is to pad side panels to fill an 80-column terminal; don't. Dead space around a dense composition reads arcade. Sprawl reads dashboard.
- **Resize is a feature.** Recompute layout and redraw on resize; if the terminal is below minimum dimensions, render a legible "needs 24×30" message rather than garbage. Most terminal games skip this and it's what separates software from a project.
- **Spawn orientation and position are specified.** Pieces spawn flat, in the two rows above the visible field. Getting this wrong makes the opening of every game feel off.
