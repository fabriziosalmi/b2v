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
  coverageReporters: ["text", "html"]
};