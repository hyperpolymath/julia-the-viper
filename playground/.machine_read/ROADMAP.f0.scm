;; SPDX-License-Identifier: PMPL-1.0-or-later
;; ROADMAP.f0.scm
;; Foundation milestone (f0) roadmap for JTV Playground

(define roadmap-f0
  '((schema . "hyperpolymath.roadmap/1")
    (repo . "hyperpolymath/jtv-playground")
    (milestone . "f0")
    (name . "Foundation / Scope Control")
    (updated . "2026-01-01")

    (objectives
      . ((primary
           . ("Establish scope control (ANCHOR policy)")
             "Route all operations through just/must"
             "Quarantine non-JTV legacy code"
             "Create golden path: `just demo` + `just test`"))
         (secondary
           . ("Set up JTV workbench structure in jtv/"
              "Implement minimal JTV parser/lexer demo"
              "Create conformance test corpus")))

    (deliverables
      . ((required
           . (("just demo" . "Offline demonstration of JTV parsing")
              ("just test" . ">=5 test samples (3 valid, 2 invalid)")
              (".machine_read/" . "Control plane files")
              ("jtv/" . "Workbench directory structure")))
         (optional
           . (("jtv/parser/" . "Basic JTV lexer/parser")
              ("jtv/samples/" . "JTV code samples")
              ("jtv/spec/" . "Language specification draft")))))

    (scope-boundaries
      . ((in-scope
           . ("JTV language experiments"
              "Parser/lexer development"
              "Test corpus creation"
              "Conformance validation tooling"
              "REPL prototype"))
         (out-of-scope
           . ("Production compiler"
              "IDE integration"
              "Package ecosystem"
              "Standard library"))))

    (success-criteria
      . ((golden-path
           . ("`just --list` shows available recipes"
              "`just test` runs offline, validates >=5 samples"
              "`just demo` runs offline, shows JTV feature"))
         (code-quality
           . ("No network dependencies in core path"
              "All code follows language policy"
              "Legacy experiments frozen in _attic/"))
         (documentation
           . ("README reflects JTV focus"
              "ROADMAP shows f0 milestone"
              "ANCHOR explains scope policy"))))

    (blocked-activities
      . ("Adding new non-JTV experiments"
         "Expanding legacy Python/Node code"
         "Creating alternative command interfaces"
         "Network-dependent test paths"))

    (next-milestone
      . ((name . "f1")
         (focus . "JTV Parser Implementation")
         (prerequisites . ("f0 complete" "Golden path validated"))))))
