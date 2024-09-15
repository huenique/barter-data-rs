# Contributing Guidelines

## General Guidelines

- **No Breaking Changes Outside `/exchanges`**: Ensure that your changes do not break existing functionality or modules outside the `/exchanges` directory.
- **No Unassigned Exchange Changes**: Do not modify other exchange integrations unless explicitly assigned to work on them.
- **Lint and Format Code**: Before pushing any changes, run:
  - `cargo clippy` to lint the code for potential issues.
  - `cargo +nightly fmt` to ensure the code is formatted consistently.

## Development Workflow

We follow a **trunk-based development** workflow, which means you can push changes directly to the `main` branch without using feature branches. Here's how the process works:

1. **Small, Self-Contained Changes**: Make sure your changes are small and self-contained to reduce the risk of introducing bugs.
2. **Test Your Changes**: Run all relevant unit and integration tests to ensure your changes don't introduce issues. If you're adding new functionality, write appropriate tests.
3. **Write Clear Commit Messages**: Use concise and descriptive commit messages to explain the purpose of your changes.
4. **Push to `main`**: Once your changes are tested and formatted, you can push them directly to the `main` branch. Ensure that the code integrates smoothly with existing functionality.

## Code Standards

- Follow the existing code style across the library.
- Ensure that all public functions, traits, and structs are well-documented.
- New exchange integrations must follow the same structure and conventions as existing ones.

## Testing

- **Unit Tests**: Focus on testing areas where errors are most likely, such as message parsing, data transformations, and subscription handling.
- **Integration Tests**: Use the library itself or example modules to test how all components work together in a real-world environment.
