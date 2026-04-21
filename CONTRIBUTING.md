# Contributing to b2v

Welcome to b2v! We welcome contributions from everyone. Whether you are a new contributor or an experienced engineer, your input is valuable in making this project better.

## Code of Conduct

Please read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) to understand our expectations for respectful interaction.

## How to Contribute

We welcome contributions through Pull Requests (PRs). Please follow these steps for submitting code, documentation updates, or bug reports.

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

### 5. Run Tests and Verify CI
Ensure all existing tests pass and add new tests to cover your changes. The process depends on the language:
*   **Rust:** Run `cargo test`.
*   **TypeScript/JavaScript:** Run `npm test` or relevant commands.

After making changes, ensure that the Continuous Integration (CI) pipeline runs successfully. For CI related issues, please check the workflow files in the `.github/workflows/` directory if applicable.

### 6. Commit Your Changes
Make sure your commit messages are clear and descriptive. Follow Conventional Commits where possible (e.g., `feat: add new feature`, `fix: correct a bug`).
```bash
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
*   Include relevant test results or CI status if applicable.
*   Provide clear context for the changes (e.g., what was changed and why). Documentation updates should also be clearly noted.**

## Development Setup

### Rust (src/)
If you are working on Rust code, ensure your environment is set up correctly. You can refer to the project's setup instructions or documentation (e.g., `README.md`).

### TypeScript/JavaScript (tests/)
If you are working on tests or related JS files, ensure you have Node.js and npm installed.

## Code Style and Formatting

We strive for high-quality, readable code. Please adhere to the following standards:

*   **Linting/Formatting:** Use established formatting tools to maintain consistent style across the project:
    *   For Rust: Use `rustfmt`.
    *   For TypeScript/JS: Use Prettier and ESLint.
*   **Code Reviews:** All contributions must be reviewed by at least one other core team member before merging.

## Running CI

Contributions that modify code or documentation should trigger the CI pipeline. If you encounter issues with CI, please report them in an issue first.