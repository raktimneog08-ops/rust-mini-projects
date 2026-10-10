#!/usr/bin/env python3
"""
Simple HTTP server with correct WebAssembly MIME types to play Cosmic Defender WASM locally.
Usage:
    python serve.py
"""

import http.server
import socketserver
import os
import webbrowser

PORT = 8080
DIRECTORY = os.path.join(os.path.dirname(__file__), "www")

class WasmHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def end_headers(self):
        # Enable CORS and proper caching
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
        super().end_headers()

    def guess_type(self, path):
        if path.endswith(".wasm"):
            return "application/wasm"
        return super().guess_type(path)

if __name__ == "__main__":
    with socketserver.TCPServer(("", PORT), WasmHandler) as httpd:
        url = f"http://localhost:{PORT}"
        print(f"🚀 Cosmic Defender Arcade Server running at: {url}")
        print("Press Ctrl+C to stop the server.")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer shutting down.")
