import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  typescript: {
    ignoreBuildErrors: true, // ⛔ disables type checking at build
  },
    eslint: {
    ignoreDuringBuilds: true, // ⛔ disables linting at build
  },
};

export default nextConfig;
