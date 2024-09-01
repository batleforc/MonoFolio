---
date: 2024-07-16T22:51:00Z
title: "argocd"
spec:
  blog: false
  project: false
  doc: true
---

Quick description of the techno used in the different projects.

```mermaid
flowchart TD;
    subgraph "Development flow"
      A([Write code in a new branch]) --> B[Create a Pull Request];
    end
    subgraph "Test flow"
      B --> C{Test Start};
      C --> GA[Test by Github Actions]
      C --> TKT[Test flow by Tekton];
      GA --> D{Test ok?};
      TKT --> D;
      D -- If test nok --> E[PR is updated with information];
    end
    subgraph "Release flow"
      D -- If test ok --> F[PR is merged into the `main` branch];
      F -- If Front changed --> G[Build the Frontend];
      F -- If Back changed --> H[Build the Backend];
      G --> I[Push the images to a conteneur registry];
      H --> I;
      I --> J[Deploy the helm chart];
    end

```

```javascript
console.log('Hello, World!');
```

```bash
echo "Hello, World!"
```

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: my-pod
spec:
  containers:
  - name: my-container
    image: nginx
```

```rust
fn main() {
    println!("Hello, World!");
}
```
