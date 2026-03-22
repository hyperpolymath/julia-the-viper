;; SPDX-License-Identifier: PMPL-1.0-or-later
;; AUTHORITY_STACK.mustfile-nickel.scm
;; Defines the authority hierarchy for repo configuration and operations

(define authority-stack
  '((schema . "hyperpolymath.authority-stack/1")
    (repo . "hyperpolymath/jtv-playground")
    (updated . "2026-01-01")

    (hierarchy
      . ((level-0 . "ANCHOR files (.machine_read/ANCHOR*.scm)")
         (level-1 . "SCM state files (STATE.scm, META.scm, ECOSYSTEM.scm, etc.)")
         (level-2 . "Mustfile (mandatory operations)")
         (level-3 . "justfile (user-visible recipes)")
         (level-4 . "Nickel manifests (*.ncl)")
         (level-5 . "Individual source files")))

    (precedence-rules
      . ("Higher levels override lower levels in case of conflict."
         "ANCHOR files have absolute authority over repo scope."
         "Mustfile operations are non-negotiable requirements."
         "justfile exposes user interface; must not contradict Mustfile."
         "Nickel manifests provide typed configuration."))

    (mustfile-contract
      . ((location . "./Mustfile")
         (purpose . "Define mandatory operations that must pass before merging")
         (required-checks
           . ((security . "just lint")
              (tests . "just test")
              (format . "just fmt")))
         (enforcement
           . ("CI must run all Mustfile checks."
              "PR merge blocked if any check fails."
              "Local commit hooks recommended."))))

    (justfile-contract
      . ((location . "./justfile")
         (purpose . "User-visible command interface")
         (required-recipes
           . ((default . "Show available recipes")
              (test . "Run offline test suite with >=5 samples")
              (demo . "Run offline JTV demonstration")
              (lint . "Run linters")
              (fmt . "Format code")))
         (naming-convention
           . ("Use kebab-case for recipe names"
              "Prefix internal recipes with underscore"
              "Group related recipes with comments"))))

    (nickel-contract
      . ((purpose . "Typed configuration manifests")
         (locations . ("./**/*.ncl"))
         (usage
           . ("Configuration that needs validation"
              "Complex settings with interdependencies"
              "Anything requiring type safety"))))

    (routing-rules
      . ("All user commands go through `just <recipe>`."
         "All mandatory checks go through `must` (Mustfile)."
         "Configuration changes go through Nickel manifests."
         "State changes update relevant .scm files."))))
