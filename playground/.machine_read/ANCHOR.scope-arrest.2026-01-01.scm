;; SPDX-License-Identifier: PMPL-1.0-or-later
;; ANCHOR.scope-arrest.2026-01-01.scm
;; Repo: hyperpolymath/jtv-playground
;; Purpose: stop scope storm; make this a controlled playground with a single golden path.

(define anchor
  '((schema . "hyperpolymath.anchor/1")
    (repo . "hyperpolymath/jtv-playground")
    (date . "2026-01-01")
    (authority . "repo-superintendent")
    (purpose
      . ("Scope arrest: this repo is a controlled playground for JTV (Julia-the-Viper) workbench activity."
         "Quarantine unrelated experiments; keep only what supports JTV language + tooling + UX."
         "Route all actions through just/must (no ad-hoc commands)."
         "Prevent toolchain drift (esp. TS/npm sprawl)."))

    (identity
      . ((project . "JTV Playground")
         (kind . "language-playground")
         (one-sentence . "A controlled experimentation workbench for JTV language + tooling, with strict task routing.")
         (upstream . "hyperpolymath/julia-the-viper")
         (satellite-of . "hyperpolymath/language-playgrounds")
         (status . "f0-control-pass")))

    (scope-policy
      . ((core-allowed
           . ("JTV language experiments"
              "Parser/AST/type experiments supporting JTV"
              "REPL / playground UI"
              "Conformance harnesses + corpora"
              "Benchmark harnesses relevant to JTV"
              "Interpreters/transpilers that are explicitly for JTV"))
         (legacy-quarantine
           . ("All non-JTV experiments are moved under experiments/_attic/ and frozen"
              "No new categories added under experiments/ unless explicitly JTV-related"))
         (forbidden
           . ("Turning this into a general polyglot kitchen-sink repo"
              "Adding new frameworks/toolchains unless they are required for JTV and routed via just/must"
              "Network-required execution paths for the golden demo"))))

    (authority-stack
      . ((order
           . ("./.machine_read/ANCHOR*.scm"
              "./STATE.scm" "./META.scm" "./ECOSYSTEM.scm" "./PLAYBOOK.scm" "./AGENTIC.scm" "./NEUROSYM.scm"
              "./Mustfile" "./justfile"
              "./**/*.ncl"))
         (routing
           . ("All user-visible operations must be exposed as `just` recipes."
              "All deployment / physical-state transitions must be exposed as `must` operations."
              "Config manifests must be Nickel; generated docs may be derived from Nickel elsewhere."))))

    (implementation-policy
      . ((primary-languages . ("Julia" "ReScript" "Scheme" "Shell" "Just" "Nickel"))
         (allowed
           . ("Julia" "ReScript" "Scheme" "Shell" "Just" "Nickel"
              "Rust"))
         (quarantined
           . ("Python (legacy only; no expansion)"
              "Node/npm (legacy only; prefer Deno/ReScript; no expansion)"
              "Any frontend framework demos not serving JTV"))
         (forbidden
           . ("TypeScript as authored source (generated output only)"
              "Makefiles"
              "New npm lock-in; do not add package-lock/yarn/pnpm; prefer deno tasks if JS is needed"
              "Second independent 'playground system' that bypasses just/must"))))

    (golden-path
      . ((smoke-test-command
           . ("just --list"
              "just test"
              "just demo"))
         (success-criteria
           . ("`just demo` runs offline and demonstrates one canonical JTV feature end-to-end."
              "`just test` runs offline and validates >=5 samples (>=3 valid, >=2 invalid)."
              "If Mustfile is present, `must` operations exist but are not required for f0 demo."))))

    (repo-shaping-directives
      . ("Create a top-level `jtv/` workbench contract: jtv/ is the only area that may grow in f0."
         "Move unrelated experiment trees into experiments/_attic/ and mark frozen."
         "Ensure README/ROADMAP stop advertising broad stacks unless explicitly quarantined."
         "If CI exists, it must only run `just test` (and optionally `just lint`) as the single entrypoint."))

    (mandatory-files
      . ("./.machine_read/LLM_SUPERINTENDENT.scm"
         "./.machine_read/AUTHORITY_STACK.mustfile-nickel.scm"
         "./.machine_read/ROADMAP.f0.scm"
         "./.machine_read/SPEC.playground.scm"))))
