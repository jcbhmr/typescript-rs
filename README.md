# typescript-rs
🟦 TypeScript project exposed to Rust

## How it works

```mermaid
flowchart TD
    AA["typescript (npm)"] --> A["typescript-X.Y.Z.tgz"] --> B["typescript-sys"]
    C["QuickJS"] --> D["rquickjs"] --> E["Node.js env shim"]
    EE["CommonJS shim"] --> E
    EEE["Node.js API shims"] --> E
    E --> B
    B --> F["typescript (Cargo)"]
    FF["typescript-$OS-$ARCH"] --> F
    AA --> G["Node.js SEA tsc"]
    AA --> H["Node.js SEA tsserver"]
    G --> FF
    H --> FF
```

1. We grab the `typescript-X.Y.Z.tgz` tarball from the npm registry. This is the actual TypeScript project code.
2. In typescript-sys, we wrap the tarball from npm in a minimal Node.js-like CommonJS-like environment using QuickJS via rquickjs.
3. We expose 

## Development
