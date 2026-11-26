# Project Agents & Environment Orchestration

This repository is orchestrated with autonomous multi-agent loops and governed by Dendrite.

## Standard Tooling
- **Default CI/CD Assistant**: GitHub Copilot (`.github/copilot-instructions.md`)
- **Runtime Stack**: RUST
- **Enforced Toolchains**: Rust (`cargo`)
- **Authoritative Issue Tracker**: Plane v2 (`http://localhost:8090`)
- **Token Governor**: LiteLLM Gateway (`http://localhost:4000`)

## Specs & Design Artifacts
Committed specs and EGM (`/egm:*`) design documents live under `docs/specs/`; per-ticket micro-plans live on the Plane ticket (`dendrite chronos plan push`), never under `.dendrite/`.

EGM artifact directory: docs/specs

## Preparation & Sync
To synchronize agent rules, skills, and enforce CI/CD tooling defaults, execute:
```bash
dendrite soma prepare   # alias: dendrite env prepare
```

## Dendrite-First Workflow (all models: Claude, Antigravity/AGY, Gemini, Copilot, humans)
1. Start every session with `dendrite session resume`; it also removes merged, clean working copies and prints the Engram memory snapshot when Engram is installed (`dendrite soma install engram`; Engram holds agent memory, rag-rs holds code search).
2. Pick work from Plane with `dendrite chronos next` (alias `plane next`, or the `dendrite_ticket_smart_select` MCP tool), never from `DEV_STATUS.md`.
3. Every change follows one ticket through its lifecycle, and a write lock enforces it: `dendrite hook pre-edit` refuses file changes in a working copy that has no claim. Pull the ticket (`dendrite chronos next`) or create it (`dendrite chronos create --name "..." --desc "goal and acceptance criteria"`), design the plan, then start it with `dendrite work new <ticket>`, which moves it to In Progress and claims it for the new copy (a claim by hand: `dendrite chronos claim <ticket>` or the `dendrite_ticket_claim` MCP tool; `dendrite_query_context` returns the ticket, its criteria and any saved plan). Each `dendrite quantal` commit and each `dendrite session checkpoint` leaves a comment on the ticket, and a plan you did not finish is stored on it (`dendrite chronos plan push`) so the next session resumes from it. When the work is finished run `dendrite chronos finish <ticket>` (MCP `dendrite_ticket_finish`): it runs the quality gate and checks the diff against each `[AC-n]`, then sets `done` when the project allows it (directive `[done] auto_done`) and the ticket has no `needs-human-review` label, `to verify` otherwise, and `blocked` with a handoff (completed, challenges, missing) when a check fails. To stop for another reason use `dendrite chronos handoff <ticket>` (blocked: write what it waits on and what is missing), `dendrite chronos ticket set <ticket> cancelled` when it is dropped or `backlog` when it is parked. Record every question you asked and the answer you got on the ticket: Claude Code's `AskUserQuestion` is stored by a hook, any other agent runs `dendrite chronos decision <ticket> --question "..." --answer "..."` (MCP `dendrite_ticket_decision`), and `dendrite chronos comments <ticket>` reads them and any handoff back. The limits every change is held to (production lines per file, coverage, duplication, whether agents may set `done`) are the project's directives in `.github/dendrite/directives.toml`; repeated code is removed with a helper function, or with a macro where a function cannot express it, and Rust tests use the `rstest` crate (`rstest = "0.28.0"`: `#[case]`, `#[values]`, `#[fixture]`) rather than hand-written copies or imitating helpers (directive `[duplication] strategies`); change them with `dendrite directive "<rule>"`, never by hand-editing CI, and check a change against them with `dendrite allostasis gate` (`--since origin/develop` judges only what you changed). A challenge found mid-plan is a new ticket linked with `dendrite chronos link`. Bypass the guards only deliberately, with `--no-ticket-guard`.
4. Every repository we can modify is a Plane project: run `dendrite chronos project ensure` in it (it writes the local `.dendrite/plane.json` that `dendrite chronos`, `sync` and agents read; never commit anything under `.dendrite/`, and in a fresh clone this command recovers the project's identity). Repositories that depend on each other belong to the same synapse (`dendrite synapse create`). `dendrite chronos project audit` lists any repository that breaks this rule.
5. Work on your own branch, never directly on `develop`, `main` or `master` (nor on the trunk of another ticket): run `dendrite work new <ticket>` for each ticket. It creates a git worktree in `dendrite-work/` next to the main checkout, on a branch named after the ticket: its own trunk `<TICKET>-<slug>/main`, which merges into `develop`, or, for a sub-ticket of a Plane parent, `<ROOT>-<slug>/<TICKET>-<slug>`, which merges into the trunk `<ROOT>-<slug>/main` of its top-most parent (`dendrite branch show --ticket <ticket>` says which; `dendrite branch parent <ticket> <parent>` links tickets, `dendrite branch parent <ticket> --clear` unlinks one; `--target <branch>` overrides). Never hand-make `git clone` or `git worktree add ../<dir>` copies: those mix with human projects and are never cleaned up.
6. Commit with `dendrite quantal` (alias `commit`; it references the ticket and pushes your branch to origin, never to main/master).
7. When the ticket is finished, merge your branch into its target (`dendrite branch show`; `develop` unless the ticket is a sub-ticket, which merges into its parent's trunk) and push, so everyone's work syncs through it: from your copy run `git fetch origin && git merge origin/<target>`, fix conflicts and re-run the tests on your branch, then `git push origin HEAD:<target>`. The parent ticket's trunk merges into `develop` the same way when every sub-ticket is done. If the push is rejected, someone else merged first: fetch, merge and push again. Never merge into or push `main`/`master`.
8. Explore with Dendrite before any raw text search, cheapest first: `dendrite rag search`/`dendrite_rag_retrieve` (detail `map`, then `dendrite rag show`/`dendrite_rag_show` for the bodies you need), `dendrite rag symbols` for definitions and references, `dendrite graph` (`dendrite_graph_explain`, `dendrite_graph_path`) for callers and structure, `dendrite rag history` for past changes, and `dendrite ask` for questions. `dendrite rag grep` is the exact-pattern fallback (index-aware, capped). Use `rg`/`grep`/`find` only when those return nothing, and then scoped to a path with a result cap, because unscoped `rg` output costs far more tokens than a retrieval map.
9. Questions about tickets (status, blockers, what a ticket is for) go to Plane through Dendrite: `dendrite ask --corpus tickets "..."` or `dendrite rag search --corpus tickets "..."` (MCP: `dendrite_rag_retrieve` with corpus `tickets`) for open questions, and `dendrite chronos`, `dendrite_plane_issues` or `dendrite_ticket_smart_select` for exact lookups. Never use `DEV_STATUS.md` or a search of the repository for this.
10. Route every LLM call through Dendrite: the LiteLLM gateway (`http://localhost:4000`) for model access and `dendrite bouton jobs submit --isolate` (alias `agent jobs`) to delegate to Antigravity (AGY) or other models. Do not call provider APIs or spawn other agent CLIs directly.
11. Reverse-engineering (REA) tickets are the ones the `[rea]` directive names (label `rea` or a `[REA]` title prefix). If you are a Claude session and refuse an REA ticket, do not leave it half done: run `dendrite retro handoff <ticket> --refused --reason "..."` (MCP `dendrite_rea_handoff`). AGY takes ownership: the refusal is recorded on the ticket, an AGY job starts in an isolated copy, the ticket is labelled `owner:agy` and a Claude session cannot claim it again without `--steal`. The Stop hook does the same when a session ends on a refusal. Change the rule with `dendrite directive set rea.refusal_owner=agy|none`; the `dendrite-rea` skill has the commands and how AGY works an REA ticket directly.
12. Run `dendrite session checkpoint` after each finished ticket and before long waits.
13. Save what the next session must know (decisions with their reason, verified findings, ticket outcomes) with `dendrite memory save --type decision|note|outcome "<title>" "<body>"` or Engram's own `mem_save`: it is journaled at once and flushed into Engram when the session ends (`dendrite memory flush` sends it now); `dendrite memory mode` says which memory stores are in use.
14. When a copy is finished and merged, `dendrite work gc` (or the next resume/checkpoint) deletes it; `dendrite work done <name>` removes one explicitly.

See the `dendrite-workflow` skill for commands and cleanup rules.

## Commit & VCS Guidance
- `dendrite quantal` (alias `commit`) pushes the new commit to the configured remote (origin preferred) after every successful commit; use `--no-push` or `DENDRITE_COMMIT_PUSH=false` to opt out; it never force-pushes and refuses to push main/master.
- `dendrite quantal --synapse[=<id>]` commits the current project, then walks its synapse peers: dirty peers are committed through the same AI-assisted engine after confirmation (`--yes` skips the prompt), and clean peers ahead of their upstream are pushed.
- `dendrite sync` pushes unpushed commits and refreshes the Plane board (canonical states + DEV_STATUS when it changed) for the current project and every synapse peer, then records synapse health (`active`/`broken`). `dendrite sync --all --watch` keeps every synapse current (`DENDRITE_SYNC_INTERVAL`, default 10m).
- Plane workflow states are canonical across projects (`plane.CanonicalStates`); fix drift with `dendrite chronos states reconcile --all`.

## Session Start & Handoff
Every new session (Claude, AGY, or human) starts with:
```bash
dendrite session resume
```
Orchestrators run `dendrite session checkpoint` after each finished ticket and before long waits:
```bash
dendrite session checkpoint [--worktree dir] [--decision "..."]... [--next "DEN-XXX"]... [--note "..."]
```

## Guide Parity & Incremental Benchmarks Rule
- **Dual-solution policy**: Every new tutorial or feature must attempt both a pure-Rust (`image`/`imageproc`) solution and an `oximedia-cv`/`oxipix` solution side by side (sharing input parameters where comparable).
- **Missing feature tickets**: When `oximedia-cv`/`oxipix` lacks a feature, file a request on `oxipix` via `dendrite chronos request oxipix --synapse cv-guide-oxipix` and record `oxipix_ticket: OXI-<id>` in `catalog/coverage/*.yaml` (enforced by `cvg-tools check`).
- **Incremental performance reference**: Every new feature must run `comparisons/opencv` (`bench_cpp` + `cargo run --release`) and update `catalog/benchmarks.json` with OpenCV vs pure Rust vs `oximedia-cv`/`oxipix` timings (enforced by `cvg-tools check`).
- **Divergence warnings & upstream reporting**: Any benchmark row diverging by `>= 3.0x` between Rust and `oximedia`/`oxipix` (or `>= 20.0x` vs OpenCV) must include a root-cause `warning_note` in `catalog/benchmarks.json` (rendered automatically as a `:::warning Performance difference` admonition on the tutorial page), and a reproduction + solution issue must be filed upstream on `cool-japan/oximedia` (when inside `oximedia`) or on `oxipix` (when outside).
