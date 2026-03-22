;; SPDX-License-Identifier: PMPL-1.0-or-later
;; SPEC.playground.scm
;; Specification for JTV Playground workbench structure

(define spec-playground
  '((schema . "hyperpolymath.spec/1")
    (repo . "hyperpolymath/jtv-playground")
    (updated . "2026-01-01")

    (identity
      . ((name . "JTV Playground")
         (type . "language-playground")
         (purpose . "Experimentation workbench for Julia-the-Viper language development")
         (upstream . "hyperpolymath/julia-the-viper")
         (scope . "JTV language, tooling, and UX experiments only")))

    (directory-structure
      . ((jtv/
           . ((description . "Primary workbench - only area that may grow")
              (subdirs
                . ((parser/ . "Lexer and parser experiments")
                   (samples/ . "JTV code samples and snippets")
                   (spec/ . "Language specification drafts")
                   (tests/ . "Test corpus and conformance tests")
                   (repl/ . "REPL prototype")
                   (tools/ . "Development tools and utilities")))))

         (experiments/
           . ((description . "Legacy experiments - frozen")
              (subdirs
                . ((_attic/ . "Quarantined non-JTV experiments")))))

         (.machine_read/
           . ((description . "Control plane - machine-readable policies")
              (files
                . (("ANCHOR*.scm" . "Scope and authority anchors")
                   ("LLM_SUPERINTENDENT.scm" . "LLM agent directives")
                   ("AUTHORITY_STACK.*.scm" . "Authority hierarchy")
                   ("ROADMAP.*.scm" . "Milestone roadmaps")
                   ("SPEC.*.scm" . "Specifications")))))

         (docs/
           . ((description . "Human-readable documentation")))

         (infrastructure/
           . ((description . "Deployment and infrastructure config")))))

    (file-types
      . ((primary
           . ((".jl" . "Julia - tooling scripts")
              (".res" . "ReScript - UI components")
              (".scm" . "Scheme - state and config")
              (".sh" . "Shell - automation scripts")
              (".ncl" . "Nickel - typed configuration")))
         (allowed
           . ((".rs" . "Rust - performance-critical tooling")
              (".jtv" . "JTV - language samples")))
         (legacy-frozen
           . ((".py" . "Python - in _attic/ only")
              (".js" . "JavaScript - in _attic/ only")
              (".ts" . "TypeScript - forbidden")))))

    (command-interface
      . ((primary . "just")
         (mandatory . "must")
         (recipes
           . ((user-facing
                . (("just demo" . "Run JTV demonstration")
                   ("just test" . "Run test suite")
                   ("just lint" . "Run linters")
                   ("just fmt" . "Format code")
                   ("just build" . "Build JTV tools")))
              (internal
                . (("just _validate" . "Validate sample corpus")
                   ("just _parse" . "Run parser on samples")))))))

    (test-corpus
      . ((location . "jtv/tests/corpus/")
         (requirements
           . ((minimum-samples . 5)
              (valid-samples . ">=3")
              (invalid-samples . ">=2")
              (offline . #t)))
         (structure
           . ((valid/ . "Valid JTV programs")
              (invalid/ . "Invalid JTV programs with expected errors")
              (edge-cases/ . "Edge cases and corner cases")))))

    (golden-path
      . ((smoke-test . "just --list && just test && just demo")
         (offline-required . #t)
         (network-forbidden . #t)))))
