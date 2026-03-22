;; SPDX-License-Identifier: PMPL-1.0-or-later
;; LLM_SUPERINTENDENT.scm
;; Machine-readable directives for LLM agents operating on this repository

(define llm-superintendent
  '((schema . "hyperpolymath.llm-superintendent/1")
    (repo . "hyperpolymath/jtv-playground")
    (updated . "2026-01-01")

    (directives
      . ((scope-enforcement
           . ("This repository is scoped EXCLUSIVELY to JTV (Julia-the-Viper) language development."
              "Reject any request to add non-JTV experiments or features."
              "All new code must support JTV language, tooling, or UX directly."))

         (command-routing
           . ("All operations MUST be routed through `just` recipes."
              "Never execute ad-hoc commands that bypass the justfile."
              "If a recipe doesn't exist for needed operation, create it in the justfile first."))

         (language-policy
           . ("Primary: Julia, ReScript, Scheme, Shell, Just, Nickel"
              "Allowed with scope: Rust (for tooling/WASM only)"
              "Quarantined (frozen, no expansion): Python, Node/npm"
              "Forbidden: TypeScript source, Makefiles, new npm dependencies"))

         (file-modification-rules
           . ("jtv/ directory: May grow with JTV-related code"
              "experiments/_attic/: Frozen, read-only legacy"
              "experiments/ (non-attic): No new subdirectories allowed"
              ".machine_read/: Control plane only, no application code"))))

    (golden-path-validation
      . ((before-commit
           . ("Verify `just test` passes offline"
              "Verify `just demo` runs without network"))
         (tests-required
           . ("Minimum 5 test samples: 3 valid, 2 invalid"
              "All tests must run offline"
              "No network calls in test suite"))))

    (agent-behaviors
      . ((on-new-feature-request
           . ("Check if feature relates to JTV language/tooling"
              "If unrelated, politely decline with scope explanation"
              "If related, add to jtv/ directory structure"))

         (on-experiment-request
           . ("Legacy experiments are frozen in experiments/_attic/"
              "New experiments must be JTV-focused and go in jtv/experiments/"
              "Never create new top-level experiment categories"))

         (on-dependency-request
           . ("Reject new npm/node_modules dependencies"
              "Prefer Deno if JS runtime needed"
              "Prefer Rust/ReScript for new tooling"))))

    (escalation-paths
      . ((scope-violation . "Explain ANCHOR policy; suggest alternative approaches")
         (golden-path-failure . "Fix before proceeding; do not skip tests")
         (language-violation . "Suggest approved language alternative")))))
