# CLAUDE.md

This file provides context and guidelines for AI assistants (particularly Claude) working with this repository.

## Project Overview

**Repository**: Hyperpolymath/jtv-playground
**Purpose**: Development playground for experimentation and testing

This is a playground repository for testing ideas, prototyping features, and experimenting with different technologies.

## Repository Structure

```
jtv-playground/
├── .git/           # Git repository data
└── CLAUDE.md       # This file - AI assistant context
```

## Development Guidelines

### General Principles

1. **Experimentation First**: This is a playground - feel free to try new approaches
2. **Clean Commits**: Use clear, descriptive commit messages
3. **Branch Strategy**: Create feature branches for experiments (prefix with `claude/` for AI-assisted work)
4. **Documentation**: Document significant experiments and learnings

### Git Workflow

#### Branch Naming
- Feature branches: `claude/<descriptive-name>-<session-id>`
- Experimental branches: `experiment/<description>`
- Bug fixes: `fix/<description>`

#### Commit Messages
Follow conventional commit format:
- `feat:` new features or experiments
- `fix:` bug fixes
- `docs:` documentation changes
- `refactor:` code refactoring
- `test:` adding or updating tests
- `chore:` maintenance tasks

Example:
```
feat: add initial project structure
docs: update CLAUDE.md with project guidelines
```

### Code Quality

- Keep experiments focused and well-organized
- Clean up unused code regularly
- Add comments to explain non-obvious decisions
- Include README files for significant experiments

## Common Tasks

### Starting a New Experiment

1. Create a new branch:
   ```bash
   git checkout -b experiment/your-experiment-name
   ```

2. Create a directory for your experiment:
   ```bash
   mkdir -p experiments/your-experiment-name
   ```

3. Document your experiment with a README

### Committing Changes

1. Stage your changes:
   ```bash
   git add .
   ```

2. Commit with a descriptive message:
   ```bash
   git commit -m "feat: describe what you built"
   ```

3. Push to remote:
   ```bash
   git push -u origin <branch-name>
   ```

### Working with AI Assistants

When working with Claude or other AI assistants:

1. **Be Specific**: Clearly describe what you want to build or test
2. **Provide Context**: Share relevant background information
3. **Review Changes**: Always review AI-generated code before committing
4. **Iterate**: Don't hesitate to ask for refinements or alternatives

## Project Conventions

### File Organization

- `/experiments/` - Individual experiment directories
- `/docs/` - Documentation and notes
- `/scripts/` - Utility scripts
- `/src/` - Main source code (if applicable)

### Documentation

- Each experiment should have its own README.md
- Include purpose, approach, and key learnings
- Document any dependencies or setup requirements

## Technology Stack

This section will be updated as technologies are added to the playground.

Current stack:
- Git for version control
- (Add technologies as they are introduced)

## Resources and References

Add useful links and resources here as the project evolves.

## Notes for AI Assistants

### Important Context

- This is a playground repository - experimentation is encouraged
- Code quality matters, but perfect isn't required
- Document learnings and interesting findings
- Ask clarifying questions when requirements are unclear

### Common Patterns

When asked to implement something:
1. Understand the requirements
2. Check existing code/experiments for similar patterns
3. Implement with clear structure
4. Test the implementation
5. Document the approach

### Git Operations

- Always work on feature branches
- Commit frequently with clear messages
- Push to remote when work is complete
- Branch names for AI work should start with `claude/`

---

**Last Updated**: 2025-11-21
**Maintained By**: Repository contributors and AI assistants
