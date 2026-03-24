# VSCode Settings for Rust

This directory contains VSCode workspace settings for optimal Rust development experience.

## Files

### settings.json

Configures Rust Analyzer (the Rust language server) for this workspace.

```json
{
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.check.features": "all",
    "rust-analyzer.cargo.buildScripts.overrideCommand": null
}
```

#### Settings Explained

| Setting | Value | Description |
|---------|-------|-------------|
| `rust-analyzer.cargo.features` | `"all"` | Enable all Cargo features when analyzing code. This allows Rust Analyzer to see optional dependencies and feature-gated code. |
| `rust-analyzer.check.features` | `"all"` | Enable all features when running `cargo check` for diagnostics. |
| `rust-analyzer.cargo.buildScripts.overrideCommand` | `null` | Use default build script command. |

### Why Enable All Features?

By default, Rust Analyzer only enables the default features when analyzing your code. This means:

- **Optional dependencies** (like `dial9-tokio-telemetry`) won't be resolved
- **Feature-gated code** will show errors or be grayed out
- **Conditional compilation** (`#[cfg(feature = "...")]`) won't work properly

Setting `"all"` ensures Rust Analyzer can see and analyze all code in your project, regardless of feature flags.

## rust-analyzer.toml (Project Root)

This file provides the same configuration at the project level, ensuring consistent behavior even when opening the project in editors other than VSCode.

```toml
[cargo]
features = "all"
```

## Alternative: Per-Feature Configuration

If you don't want to enable all features, you can specify specific features:

```json
{
    "rust-analyzer.cargo.features": ["dial9", "some-other-feature"]
}
```

Or use the default features only:

```json
{
    "rust-analyzer.cargo.features": "default"
}
```

## Troubleshooting

### "Cannot find crate" errors for optional dependencies

**Solution**: Ensure `rust-analyzer.cargo.features` is set to `"all"` or includes the required feature.

### Changes not taking effect

1. **Restart Rust Analyzer**: Press `Ctrl+Shift+P` → "Rust Analyzer: Restart Server"
2. **Reload Window**: Press `Ctrl+Shift+P` → "Developer: Reload Window"
3. **Clear cache**: Delete `target/` directory and restart VSCode

### High memory usage

Enabling all features can increase memory usage. If this is an issue:

```json
{
    "rust-analyzer.cargo.features": ["dial9"],
    "rust-analyzer.procMacro.enable": false
}
```

## References

- [Rust Analyzer Manual - Cargo Features](https://rust-analyzer.github.io/manual.html#cargo-features)
- [Rust Analyzer Configuration](https://rust-analyzer.github.io/manual.html#configuration)
