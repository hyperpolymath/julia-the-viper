;; SPDX-License-Identifier: PMPL-1.0-or-later
;; STATE.scm - Project state for julia-the-viper
;; Media-Type: application/vnd.state+scm

(state
  (metadata
    (version "0.0.1")
    (schema-version "1.0")
    (created "2026-01-03")
    (updated "2026-02-07")
    (project "julia-the-viper")
    (repo "github.com/hyperpolymath/julia-the-viper"))

  (project-context
    (name "julia-the-viper")
    (tagline "Reversible systems programming with purity guarantees")
    (tech-stack (Rust WASM))
    (implementation-language Rust)
    (target-domains (systems-programming formal-verification reversible-computing))
    (related-repos
      (julia-zig-ffi "FFI bindings")
      (jtv-playground "Examples and experimentation")
      (nextgen-languages/julia-the-viper "OUTDATED - embedded snapshot")))

  (current-position
    (phase "core-working-gaps-remain")
    (overall-completion 60)
    (loc 4589)
    (files 22)
    (size "666M")
    (components
      ((parser (status complete) (loc 850))
       (typechecker (status complete) (loc 620))
       (interpreter (status complete) (loc 980))
       (formatter (status complete) (loc 340))
       (purity-checker (status complete) (loc 450))
       (reversible-computing (status complete) (loc 520))
       (number-system (status complete) (loc 380))
       (repl (status complete) (loc 280))
       (cli (status complete) (loc 169))
       (wasm-backend (status partial) (completion 30))
       (lsp-server (status missing))
       (debugger (status missing))
       (package-manager (status complete) (implementation "viper-pkg") (language "Julia") (opsm-integrated true))
       (vscode-extension (status missing))))
    (working-features
      (parsing "Full recursive descent parser")
      (type-checking "Hindley-Milner with extensions")
      (interpretation "Complete interpreter for core language")
      (formatting "AST pretty-printing")
      (purity-analysis "Effect tracking and verification")
      (reversibility "Reversible computation primitives")
      (repl "Interactive REPL with expression evaluation")
      (cli "Command-line interface with multiple subcommands")))

  (route-to-mvp
    (milestones
      ((complete-wasm-backend
        (priority critical)
        (effort "15-20 hours")
        (description "Complete WASM code generation"))
       (add-lsp-server
        (priority high)
        (effort "20-25 hours")
        (description "Implement LSP for editor integration"))
       (add-debugger
        (priority medium)
        (effort "12-15 hours")
        (description "Interactive debugger with reversibility inspection"))
       (add-package-manager
        (priority medium)
        (effort "15-20 hours")
        (description "Dependency resolution and package management"))
       (documentation
        (priority high)
        (effort "10-15 hours")
        (description "API docs, tutorials, language reference")))))

  (blockers-and-issues
    (critical
      (wasm-backend-incomplete
        (severity high)
        (impact "Cannot compile to WASM target")
        (description "WASM code generation at 30% - missing control flow, memory management")))
    (high
      (no-editor-integration
        (severity medium)
        (impact "Poor developer experience without LSP")
        (description "Missing LSP server for diagnostics, completion, hover")))
    (medium
      (scattered-examples
        (severity low)
        (impact "Examples in jtv-playground need consolidation")
        (description "Examples should be in main repo")))
    (low))

  (critical-next-actions
    (immediate
      (complete-wasm-codegen
        (description "Finish WASM backend - control flow and memory management")
        (files ("crates/jtv-core/src/wasm.rs"))
        (effort "15-20 hours")))
    (this-week
      (begin-lsp-implementation
        (description "Start LSP server for editor integration")
        (effort "20-25 hours"))
      (consolidate-examples
        (description "Move examples from jtv-playground to main repo")
        (effort "2-3 hours")))
    (this-month
      (add-debugger
        (description "Interactive debugger with reversibility inspection")
        (effort "12-15 hours"))
      (write-documentation
        (description "Tutorials and API documentation")
        (effort "10-15 hours"))))

  (session-history
    ((2026-02-07
      (focus "Verified actual implementation status")
      (discoveries
        "STATE.scm claimed 0% but reality is 60% - 22 Rust files, 4,589 LOC, full compiler pipeline")
      (actions
        "Updated STATE.scm to reflect reality"
        "Identified consolidation needs")))))
