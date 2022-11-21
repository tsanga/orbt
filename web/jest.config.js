const nextJest = require("next/jest");
const { pathsToModuleNameMapper } = require("ts-jest");
const { compilerOptions } = require("./tsconfig");

const createJestConfig = nextJest({
  // Provide the path to your Next.js app to load next.config.js and .env files in your test environment
  dir: "./",
});

// Add any custom config to be passed to Jest
/** @type {import('jest').Config} */
const customJestConfig = {
  setupFilesAfterEnv: ["<rootDir>/jest.setup.js"],
  moduleDirectories: ["node_modules", "<rootDir>"],
  modulePaths: [compilerOptions.baseUrl],
  moduleNameMapper: pathsToModuleNameMapper(compilerOptions.paths),
  testEnvironment: "jest-environment-jsdom",
};

const config = async () => {
  const jestConfig = await createJestConfig(customJestConfig)();

  // inserted by next, conflicts with vanilla extract
  // see: https://vanilla-extract.style/documentation/test-environments/#remove-style-mocking
  delete jestConfig.moduleNameMapper["^.+\\.(css|sass|scss)$"];

  return {
    ...jestConfig,
    moduleNameMapper: {
      "\\.svg$": "<rootDir>/__mocks__/svg.js",
      ...jestConfig.moduleNameMapper,
    },
    transform: {
      "\\.css\\.ts$": "@vanilla-extract/jest-transform",
      ...jestConfig.transform,
    },
  };
};

module.exports = config;
