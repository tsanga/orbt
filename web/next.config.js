const { VanillaExtractPlugin } = require("@vanilla-extract/webpack-plugin");
const {
  getGlobalCssLoader,
} = require("next/dist/build/webpack/config/blocks/css/loaders");
const { lazyPostCSS } = require("next/dist/build/webpack/config/blocks/css");

// @ts-check
/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    appDir: true,
  },
  compiler: {
    relay: {
      src: "./src/",
      schema: "../api/orbt.graphql",
      language: "typescript",
      artifactDirectory: "./src/__generated__",
      customScalars: { "Id": "string" }
    },
  },
  output: "standalone",
  webpack: (config, { isServer, dev, dir, supportedBrowsers }) => {
    config.module.rules.unshift({
      test: /\.svg$/,
      issuer: /\.[jt]sx?$/,
      use: [{ loader: "@svgr/webpack", options: { typescript: true } }],
    });

    const cssRules = config.module.rules.find(
      (rule) =>
        Array.isArray(rule.oneOf) &&
        rule.oneOf.some(
          ({ test }) =>
            typeof test === "object" &&
            typeof test.test === "function" &&
            test.test("filename.css")
        )
    ).oneOf;

    cssRules.unshift({
      test: /\.vanilla\.css$/i,
      sideEffects: true,
      use: getGlobalCssLoader(
        {
          assetPrefix: config.assetPrefix,
          isClient: !isServer,
          isServer,
          isDevelopment: dev,
          future: nextConfig.future || {},
          experimental: nextConfig.experimental || {},
          hasAppDir: true,
        },
        () => lazyPostCSS(dir, supportedBrowsers),
        []
      ),
    });

    config.plugins.push(
      new VanillaExtractPlugin({ outputCss: !isServer, identifiers: "short" })
    );

    return config;
  },
};

module.exports = nextConfig;
