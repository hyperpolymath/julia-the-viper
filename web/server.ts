// SPDX-License-Identifier: PMPL-1.0-or-later
// Deno server for Julia the Viper PWA

import { serveDir } from "https://deno.land/std@0.224.0/http/file_server.ts";

const PORT = 8000;

Deno.serve({ port: PORT }, (req) => {
  const pathname = new URL(req.url).pathname;

  // Serve WASM files from pkg directory
  if (pathname.startsWith('/pkg/')) {
    return serveDir(req, {
      fsRoot: "../",
      showDirListing: false,
    });
  }

  // Serve web files
  return serveDir(req, {
    fsRoot: ".",
    showDirListing: false,
  });
});

console.log(`🐍 Julia the Viper PWA server running at http://localhost:${PORT}/`);
console.log(`📦 Serving WASM from ../pkg/`);
