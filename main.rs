name: 'Build & Release Desktop App'

on:
  push:
    branches:
      - release
  workflow_dispatch: {}

jobs:
  publish-tauri:
    permissions:
      contents: write
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: 'macos-latest'
            args: '--target aarch64-apple-darwin'
          - platform: 'macos-latest'
            args: '--target x86_64-apple-darwin'
          - platform: 'ubuntu-22.04'
            args: ''
          - platform: 'windows-latest'
            args: ''

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4

      - name: install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.platform == 'macos-latest' && 'aarch64-apple-darwin,x86_64-apple-darwin' || '' }}

      - name: install Linux build dependencies
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev patchelf

      - name: setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: install frontend dependencies
        run: npm install

      - uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          # Add these two repo secrets later if you want signed macOS/Windows builds
          # (unsigned builds still work fine for beta testing, users just click through
          # an OS warning the first time they open the app):
          # APPLE_CERTIFICATE / APPLE_CERTIFICATE_PASSWORD / APPLE_SIGNING_IDENTITY
          # TAURI_SIGNING_PRIVATE_KEY / TAURI_SIGNING_PRIVATE_KEY_PASSWORD
        with:
          tagName: app-v__VERSION__
          releaseName: 'Tube Bend Calculator v__VERSION__'
          releaseBody: 'Beta build. See assets below for your platform (Windows .msi/.exe, macOS .dmg, Linux .deb/.AppImage).'
          releaseDraft: true
          prerelease: true
          args: ${{ matrix.args }}
