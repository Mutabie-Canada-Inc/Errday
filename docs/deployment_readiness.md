# 🚀 Deployment & Security Checklist

This document prepares **Errday** for production deployment, covering security basics and performance expectations.

## 🔒 1. Security Checklist

### A. Local Data Security
- **Data Location:** Currently stored in standard User Data directories via `directories` crate.
- **Privacy:** Since Errday is offline-first, the primary security boundary is the user's OS profile. 
- **Recommendation:** In future versions, consider adding a local encryption layer (e.g., using `age` or `aes-gcm`) for the `tasks.json` file if sensitive founder data is expected.

### B. WebView Isolation
- **Context:** Dioxus Desktop runs in a WebView (WRY). 
- **XSS Prevention:** Rust and Dioxus prevent most XSS by design (no `innerHTML` usage). Avoid `eval` or `raw_html` features unless strictly necessary.
- **External Links:** Ensure all external links use `open::that()` or Dioxus's default handler to open in the user's default browser, protecting the app from untrusted web content.

### C. Dependency Auditing
- **Tooling:** Before every release, run `cargo audit` to check for security advisories in the dependency tree.
- **Supply Chain:** Keep dependencies pinned or carefully updated.

---

## 🏎️ 2. Performance Optimizations implemented

| Optimization | Status | Benefit |
| :--- | :--- | :--- |
| **Release Profile** | ✅ Applied | Optimized `opt-level = 3` for maximum execution speed. |
| **LTO (Link Time Opt)** | ✅ Applied | Cross-crate optimization for smaller, faster binaries. |
| **Symbol Stripping** | ✅ Applied | Removed debug symbols to minimize executable size (Target: <30MB). |
| **Panic Abort** | ✅ Applied | Simplified binary structure for faster startup and smaller size. |
| **Min. Cloning** | ✅ In Progress | Reviewing components to ensure `Signal` and `Props` are used without unnecessary cloning. |

---

## 🛠️ 3. Deployment Steps (The Roadmap)

### Phase 1: Artifact Bundling
On macOS, a simple binary isn't enough for a premium feel. We need a `.app` bundle.
- Use `cargo-bundle` or the Dioxus CLI (`dx bundle --release`).
- **Icons:** Prepare a high-quality `.icns` file for macOS and `.ico` for Windows.

### Phase 2: Signing & Notarization (macOS)
To avoid "unidentified developer" warnings:
1. Obtain an Apple Developer Program membership.
2. Sign the app with your Developer ID certificate.
3. Notarize the app via Apple's notary service.

### Phase 3: Installers
- **Windows:** Create an MSIX or a simple `.exe` using Inno Setup or NSIS.
- **Linux:** Distribute as an AppImage or `.deb` for maximum compatibility.

### Phase 4: Distribution
Since it's offline-first, you can distribute via:
- Direct Download from a website.
- GitHub Releases.
- Future: Mac App Store / Windows Store.

---

## 📦 4. Dependency Cleanup
- `dioxus-logger`: Kept for `dev`, but we should ensure it's conditionally included or minimized in `release`.
- `directories`: Core for data persistence.
- `uuid` & `chrono`: Core for task management.
- All other libraries in `Cargo.toml` are currently in use by the core workflow.
