# Legacy Experiments (FROZEN)

> **ANCHOR Scope Policy**: These experiments are frozen and may not be expanded.
> Per `.machine_read/ANCHOR.scope-arrest.2026-01-01.scm`

## Status

All code in this directory is **LEGACY QUARANTINE**:

- No new features
- No bug fixes (except security-critical)
- No dependency updates
- No new files or directories

## Reason

This repository is now scoped exclusively to **JTV (Julia-the-Viper)** language development. Non-JTV experiments are preserved here for historical reference but are not maintained.

## Contents

| Directory | Description | Original Purpose |
|-----------|-------------|------------------|
| `algorithms/` | Algorithm implementations | Python algorithm demos |
| `api-demos/` | REST/WebSocket demos | Node.js API experiments |
| `database-demos/` | Database experiments | SQLite/ArangoDB demos |
| `design-patterns/` | Pattern implementations | Python design patterns |
| `julia-demos/` | Julia language demos | General Julia experiments |
| `python-demos/` | Python utilities | CLI tools, pipelines |
| `utilities/` | Utility libraries | Form validation, etc. |

## Migration Path

If any of this code is needed for JTV development:

1. Copy relevant portions to `jtv/` directory
2. Rewrite in approved language (Julia, ReScript, Rust, Scheme)
3. Do NOT modify code in `_attic/`

## See Also

- `.machine_read/ANCHOR.scope-arrest.2026-01-01.scm` - Scope policy
- `.machine_read/LLM_SUPERINTENDENT.scm` - Agent directives
- `jtv/README.adoc` - Active workbench documentation
