# Repo Agent Contract

## State Machine
```
ISSUE OPENED ──> PLAN MODE
                    │
                    ▼
              [Plan posted]
                    │
    ┌───────────────┴───────────────┐
    │                               │
    ▼                               ▼
 comment                        bot::run
 (not bot::run)                     │
    │                               ▼
    └──> PLAN MODE           IMPLEMENT MODE
         (refine plan)              │
                                    ▼
                              [PR created/updated]
                                    │
                    ┌───────────────┴───────────────┐
                    │                               │
                    ▼                               ▼
                 comment                       bot::merge
                 (not bot::merge)                   │
                    │                               ▼
                    └──> PLAN MODE              [Merged]
                         (back to planning)
```

## Golden Rules
1. Every Issue starts in PLAN MODE - produces plan with issue-specific verification
2. `bot::run` triggers IMPLEMENT MODE - code gets written, PR created/updated
3. Any other comment after run -> back to PLAN MODE (conversation continues)
4. `bot::run` again -> updates existing PR with refined implementation
5. `bot::merge` only works if verification passes

## Execution Contract
- Standard checks: `just verify` (fmt, clippy, test, build)
- Issue-specific: Each plan MUST define verification criteria for that specific issue
- The implementer creates/updates tests or examples that prove the issue is resolved

## Git Hygiene
- Branch: `agent/issue-<NUMBER>`
- Commits: `[issue-<NUMBER>] <description>`
- One PR per issue, updated on each `bot::run`
