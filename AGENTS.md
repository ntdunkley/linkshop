# AGENTS.md

## 1. General Principles

All code must prioritise maintainability, readability, correctness, and
simplicity.

Follow established Rust idioms and good software engineering principles:

-   **DRY:** Avoid duplicated logic and functionality. Extract shared
    behaviour where appropriate.
-   **SOLID:** Apply relevant principles, particularly single
    responsibility and dependency inversion.
-   **KISS:** Prefer simple, understandable solutions over unnecessary
    complexity.
-   **Separation of Concerns:** Keep responsibilities clearly separated.
-   **Composition over inheritance:** Use structs and traits to share
    behaviour where appropriate.

Additional guidelines:

-   Prefer strongly typed solutions over primitive types where
    appropriate.
-   Keep functions focused and reasonably small.
-   Avoid unnecessary cloning, allocations, and mutable state.
-   Avoid magic strings and numbers.
-   Prefer explicit error handling.
-   Do not introduce unnecessary abstractions.
-   Prioritise readability over premature optimisation.
-   DB migrations should be created with "sqlx migrate add <name>"

## 2. Modules and Project Structure

Use modern Rust module organisation.

**Never use `mod.rs` files.** Use named module files instead.

Be liberal with creating modules to keep the codebase organised and
responsibilities separated.

-   Group related functionality into dedicated modules.
-   Database repositories must live in a dedicated `repos` module, not
    in the project root.
-   HTTP handlers must be kept separate from business logic and data
    access.
-   Authentication functionality should have its own module.
-   Avoid large files containing unrelated functionality.
-   Use descriptive module and file names.
-   Avoid generic dumping-ground modules such as `helpers`, `common`, or
    `misc`.
-   Prefer private visibility and only expose functionality where
    necessary.

Create modules whenever they provide a meaningful separation of
responsibility. Do not keep unrelated files together simply to minimise
the number of modules.

## 3. Structs and Traits

Use traits and structs appropriately to encourage code reuse and reduce
duplication.

-   Define traits for shared behaviour and meaningful abstractions.
-   Where multiple structs require the same functionality, consider
    whether that functionality belongs in a shared trait or component.
-   Prefer composition and reusable implementations over duplicated
    methods.
-   Use traits to establish clear boundaries between components where
    appropriate.
-   Keep struct responsibilities focused.
-   Keep fields private by default.
-   Avoid unnecessary traits, generics, and abstraction layers.

Do not introduce traits simply for the sake of having an interface.
Abstractions should solve a genuine design or testing problem.

## 4. Application Organisation

Keep HTTP-specific concerns within handlers.

-   Handlers should deal with HTTP requests, responses, and request
    validation.
-   Database queries and persistence operations belong in repositories.
-   Business logic should live in an appropriate location, using a
    separate service module only when the complexity or responsibilities
    justify one.
-   Avoid putting business logic directly into handlers.
-   Avoid putting database queries outside repositories.
-   Keep responsibilities clearly separated without introducing
    unnecessary architectural layers.

## 5. Error Handling

-   Use `Result` and `Option` appropriately.
-   **Never use `unwrap()` or `expect()` in production code.** Always
    handle errors explicitly and propagate them where appropriate.
-   Using `unwrap()` and `expect()` in tests is encouraged where it
    reduces unnecessary boilerplate.
-   Never silently ignore errors.
-   Use consistent application error types.
-   Preserve useful error context.
-   Never expose sensitive internal error details to API consumers.

## 6. Logging and Observability

Use structured logging and tracing.

-   Prefer the `tracing` ecosystem.
-   Include useful context when logging errors.
-   Use appropriate log levels.
-   Avoid excessive logging.
-   Never log passwords, authentication tokens, secrets, or other
    sensitive information.
-   Include request correlation or trace identifiers where appropriate.

## 7. Testing

Tests are an essential part of development.

-   Add unit tests for new business logic and complex functionality.
-   Add integration tests where interactions between components need
    verification.
-   Update existing tests when behaviour changes.
-   Keep tests focused, readable, and independent.
-   Ensure tests cover relevant error and edge cases.

## 8. Comments and Documentation

Comments must describe the code and its behaviour, not the history of
its development.

-   Comments should explain non-obvious behaviour, important decisions,
    constraints, or business rules.
-   Avoid comments that simply repeat what the code already
    communicates.
-   Use Rust documentation comments for public APIs where appropriate.

**Never include Jira references, developer prompt descriptions,
investigation history, or task-management context in source code
comments.**

Keep documentation accurate and update it when relevant functionality
changes.

## 9. Formatting and Linting

Before committing, always run:

-   `cargo fmt`
-   `cargo clippy`

Both must complete successfully before a commit is made.

## 10. Git

Keep commits focused and meaningful.

## 11. Working with Existing Code

When modifying existing functionality:

-   Understand the relevant code and module structure before making
    changes.
-   Follow existing conventions unless they conflict with these
    guidelines.
-   Reuse existing functionality rather than duplicating it.
-   Avoid unnecessary rewrites or unrelated refactoring.
-   Update relevant tests and documentation when behaviour changes.
-   Address existing issues when directly relevant to the requested
    changes.

## 12. Adding New Functionality

When implementing new functionality:

-   Identify the appropriate module and responsibility.
-   Look for existing functionality that can be reused.
-   Use appropriate structs and traits to avoid duplication.
-   Maintain clear separation of concerns.
-   Handle errors explicitly.
-   Add relevant tests.
-   Run `cargo fmt` and `cargo clippy` before committing.

Always favour a simple, maintainable implementation over unnecessary
architectural complexity.
