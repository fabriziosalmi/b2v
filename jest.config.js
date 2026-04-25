module.exports = {
  preset: "ts-jest",
  testEnvironment: "node",
  testMatch: [
    "**/?(*.)+(spec|test).?(m|k)js"
  ],
  coverageDirectory: "coverage",
  collectCoverageFrom: [
    "src/**/*.ts"
  ],
  coverageReporters: ["text", "html"],
  coverageThreshold: {
    global: {
      branches: 100,
      functions: 100,
      lines: 100
    }
  }
};