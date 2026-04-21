module.exports = {
  preset: "*",
  testEnvironment: "node",
  setupFilesAfterEnv: [
    "./jest.setup.js"
  ],
  // Add other specific configurations if needed for Rust/TS interaction, but keeping it simple for now.
};
