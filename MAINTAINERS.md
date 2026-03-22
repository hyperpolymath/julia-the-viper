# Maintainers

This file lists the current maintainers of Julia the Viper.

## Core Team

### Lead Maintainer

**To Be Determined**
- **Role**: Project leadership, architecture decisions, release management
- **GitHub**: TBD
- **Email**: TBD
- **Timezone**: TBD
- **Focus**: Language design, security model, roadmap

## Component Maintainers

### Parser & Core Language

**To Be Determined**
- **Components**: `packages/jtv-lang/src/parser.rs`, `shared/grammar/`
- **Responsibilities**: Grammar evolution, parser performance, AST design

### Interpreter & Runtime

**To Be Determined**
- **Components**: `packages/jtv-lang/src/interpreter.rs`, `packages/jtv-lang/src/number.rs`
- **Responsibilities**: Execution correctness, performance, number system support

### Standard Library

**To Be Determined**
- **Components**: `packages/jtv-lang/stdlib/`
- **Responsibilities**: API design, documentation, testing

### Tooling

**To Be Determined**
- **Components**: `tools/cli/`, `tools/vscode-extension/`, `tools/lsp/`
- **Responsibilities**: Developer experience, IDE integration, debugging

### Documentation

**To Be Determined**
- **Components**: `docs/`, `README_JTV.md`, examples
- **Responsibilities**: Tutorials, guides, API docs, examples

### Smart Contracts

**To Be Determined**
- **Components**: `packages/jtv-safe/`, `examples/contracts/`
- **Responsibilities**: Blockchain integration, security proofs, contract patterns

## Emeritus Maintainers

(None yet - first generation of maintainers)

## Maintainer Responsibilities

### All Maintainers

- **Code Review**: Review PRs in your component within 48 hours
- **Issue Triage**: Label and prioritize issues
- **Communication**: Respond to questions from contributors
- **Documentation**: Keep docs updated for your component
- **Security**: Report vulnerabilities privately
- **Conduct**: Enforce Code of Conduct

### Lead Maintainer

- **Releases**: Manage version releases
- **Roadmap**: Set project direction
- **Community**: Represent project publicly
- **Conflict Resolution**: Final arbiter on technical disputes
- **Onboarding**: Train new maintainers

## Becoming a Maintainer

### Requirements

1. **Contributions**: 10+ merged PRs with significant impact
2. **Expertise**: Deep knowledge of one or more components
3. **Communication**: History of helpful code reviews and issue responses
4. **Reliability**: Consistently available for your component
5. **Values Alignment**: Demonstrates commitment to project values

### Process

1. **Nomination**: Any maintainer can nominate a contributor
2. **Discussion**: Core team discusses privately
3. **Consensus**: Must have approval from all current maintainers
4. **Announcement**: Publicly announced and added to this file
5. **Onboarding**: 30-day mentorship period

## Stepping Down

Maintainers can step down at any time:

1. **Notify**: Email core team with 2 weeks notice (or immediate if urgent)
2. **Transition**: Help find replacement or redistribute responsibilities
3. **Emeritus**: Listed as emeritus maintainer with continued recognition
4. **Re-joining**: Always welcome to return as maintainer

## Communication Channels

### Public

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: General questions, ideas
- **Discord**: Real-time chat (link TBD)

### Maintainer-Only

- **Email**: maintainers@julia-viper.dev
- **Private Discord**: For sensitive discussions
- **Monthly Sync**: First Monday of each month, 16:00 UTC

## Decision Making

### Consensus

Most decisions use **lazy consensus**:
- Proposal posted publicly
- 72-hour comment period
- If no objections, approved
- Any maintainer can object

### Voting

For major decisions (breaking changes, license changes, etc.):
- Simple majority of core team
- Lead maintainer has tie-breaking vote

### Technical Disputes

1. **Discussion**: Try to reach consensus
2. **Expert Opinion**: Consult component maintainer
3. **Lead Decision**: Lead maintainer makes final call
4. **Document**: Rationale recorded in decision log

## Maintainer Expectations

### Time Commitment

- **Minimum**: 4 hours/week
- **Preferred**: 8+ hours/week
- **Flexible**: Life happens, communication is key

### Response Times

- **Critical Security**: 24 hours
- **PR Reviews**: 48 hours
- **Issue Triage**: 72 hours
- **General Questions**: Best effort

### Conflict of Interest

Maintainers must disclose:
- Employment at companies using JtV commercially
- Grants or funding related to JtV
- Competing projects

## Removal

Maintainers can be removed for:
- **Inactivity**: No activity for 6+ months without notice
- **Code of Conduct Violations**: Serious or repeated violations
- **Loss of Trust**: Consensus of other maintainers

Process:
1. Private discussion among other maintainers
2. Attempt to resolve concerns
3. If unresolved, vote (75% required)
4. Private notification
5. Public announcement (if appropriate)

## Current Vacancies

**All positions currently vacant - seeking founding maintainers!**

Interested? See CONTRIBUTING.md for how to get involved.

---

Last updated: 2025-01-22
Next review: 2025-04-22
