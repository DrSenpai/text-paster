import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Exportiere als statische Assets statt Server-Side Rendering
  output: "export",
  distDir: "out",
};

export default nextConfig;
