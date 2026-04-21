# Contributing to b2v

Welcome to b2v!

We welcome contributions from everyone. Whether you are a new contributor or an experienced engineer, your input is valuable in making this project better.

## Code of Conduct

Please read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) to understand our expectations for respectful interaction.

## How to Contribute

### 1. Fork the Repository
Fork the repository on GitHub to start working on your changes.

### 2. Clone Your Fork
Clone your fork locally:
```bash
git clone https://github.com/YOUR_USERNAME/b2v.git
cd b2v
```

### 3. Create a New Branch
Create a new branch for your feature or fix:
```bash
git checkout -b feature/my-contribution
```

### 4. Make Changes
Implement your changes in the relevant files.

### 5. Run Tests
Ensure all existing tests pass and add new tests to cover your changes. If you are working on Rust code, run `cargo test`. If you are working on TypeScript/JS code, run `npm test` or relevant commands.

### 6. Commit Your Changes
Make sure your commit messages are clear and descriptive. Follow Conventional Commits where possible (e.g., `feat: add new feature`, `fix: correct a bug`).
```bash
go
git add .
git commit -m "feat: descriptive message"
```

### 7. Push to Your Fork
Push your branch to your fork:
```bash
git push origin feature/my-contribution
```

### 8. Open a Pull Request (PR)
Open a Pull Request against the main branch of the original repository, clearly explaining your changes in the PR description.

**When opening a PR, please ensure you:
*   Address any open issues.
*   Include relevant test results if applicable.
*   Provide clear context for the changes.**

## Development Setup

### Rust (src/)
If you are working on Rust code, ensure your environment is set up correctly. You can refer to the project's setup instructions or documentation (e.g., `README.md`).

### TypeScript/JavaScript (tests/)
If you are working on tests or related JS files, ensure you have Node.js and npm installed.

## Code Style and Formatting

We strive for high-quality, readable code. Please adhere to the following:

*   **Linting/Formatting:** Use tools like `rustfmt` (for Rust) and Prettier/ESLint (for TypeScript/JS) to maintain consistent formatting.
*   **Code Reviews:** All contributions must be reviewed by at least one other core team member before merging.

## Submitting Changes

For bug reports or feature requests that don't involve code changes, please open an issue in the repository.
