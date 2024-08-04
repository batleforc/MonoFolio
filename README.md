# Portfolio - v3 Monorepo Over Heaven

[![codecov](https://codecov.io/github/batleforc/MonoFolio/graph/badge.svg?token=UONUFAX0K2)](https://codecov.io/github/batleforc/MonoFolio)

## TODO

- Define the content of the portfolio
- Reproduce existing portfolio or re-start from zero ? Make pros and cons

### Archi

- [DONE] Create a [local plugin](https://nx.dev/extending-nx/intro/getting-started)
  - Create a true Rust Lint command that will integrate what's in cog.toml
  - Create a true Vue Lint command that will extend the front:lint
- [DONE] Change GitHooks to include all the project lint/check
  - Lint should be applied before commit
  - GitLeaks should block commit
  - Test should block if error and display a warn if warning
- Create a CI/CD that will trigger subsequent task depending on the code change (If possible in tekton and if not in github)
  - If front change
    - Build in it's own container (nginx)
    - Redeploy container (Helm chart)
    - Sonar scann
  - If back change
    - Build in it's own container (including all bin ? follow the Dockerfile from Sandbox bot discord)
    - Redeploy container (Helm chart with every sub component)
    - Sonar scann
  - Translate [github ci workflow](https://github.com/batleforc/NeoNet/tree/main/.github/workflows)
- Create a doc regarding the good practice that should be followed in order to work on this project

### Front

- [Discover ThreeJS for vue](https://docs.tresjs.org/guide/getting-started.html)
- Include [Vue Router](https://router.vuejs.org/)
- Include [Pinia](https://pinia.vuejs.org/) if needed
- Include dotenv

### Back

- BootStrap tracing based on DiscordBot Sandbox (Set up a lib ?)
- Set up api
  - Utoipa + UI
  - Actix
  - A database ?
  - Include dotenv
- DDD based on sub-lib ?

## Idea

### Milestone

- [BACK] Start the project [2024-07-14]
- [BACK] First version of the base content [2024-08-05]
- [BACK] Rewrite path in markdown
  - [BACK] Broke link if the file doesn't exist
  - [BACK] Create the link if the file exist and write it
- [BACK] Create an API
  - [BACK] Serve the home content
  - [BACK] Serve the different blog/project/doc content
  - [BACK] Serve the media content
- [FRONT] Render the home page
- [FRONT] Render the blog page and timeline
- [FRONT] Render the doc page

### Keeping the same layout

#### Pros - layout

- The layout is already done
- The layout is already responsive
- I can focus on the improvement of the content
- I can focus on the improvement of the performance
- I can focus on the improvement of the SEO
- I can focus on the future Easter Egg

#### Cons - layout

- The layout is not perfect
- I have no inspiration to improve the layout/content
- I'm thinking about a new kind of layout
  - Include some basic information about me
  - Include some animation for the Timeline for example
  - Include an "easter egg" that will redirect to a 3D version of the portfolio
  - Include a true blog section
  - Include a true project section

### Portfolio base content

- Home
  - Presentation
    - Me ?
    - History ?
  - CV
  - Project
    - Tech that i've used and use
    - Project that i've done using those tech (Linked to blog and Doc)
    - Project that i'm working on (Linked to blog and Doc)
    - Future project (Linked to the WIP page blog and Doc)
  - Contact and Social
- Blog
  - Discovery
  - Tutorial
  - Project (linked to the project section)
  - Editorial/Event
- Doc
  - Link to other doc (like the one for each project) with disclaimer
  - Doc page that in reality is a blog post
- Easter Egg (either [ThreeJS](https://docs.tresjs.org/guide/getting-started.html) or [Bevy](https://github.com/bevyengine/bevy) or vanillaVue JS or Gif)
  - 3D version of the portfolio (Little game ?)
  - Pong game
  - Konami code
  - 418 page, i'm a teapot
  - LEEROYYY JENKINS
  - Rick Roll
  - The cake is a lie
  - The matrix (code, bullet time, rain)
  - Star Wars opening with the text "A long time ago in a galaxy far, far away...."
  - Star Wars BB-8/RD-D2 (little droid that will follow the mouse or run through the base of the page)
  - Iron Man (Jarvis ?)
  - The Avengers (Thanos snap ?)
  - Thor (Mjolnir ?)
  - The Hulk (Always angry ?)
  - H2G2 (Don't panic take a towel, 42, "the answer to the ultimate question of life, the universe, and everything" , the wale and the petunia)
  - Jojo (To be continued, the pose, No No No, Yes Yes Yes)
  - Pokemon (Pikachu, Charmander, Bulbasaur, Squirtle, One day i'll be the best)
  - Zelda ("It's dangerous to go alone, take this", the triforce, the master sword, the ocarina, im not zelda but a link)
  - Gundam

Doc and Blog article should be in the markdown format

## Participating

### Install Hook

```bash
cog install-hook --all
```

### Generate code coverage

```bash
cargo llvm-cov --workspace --open
```

### Tool needed

- [Rust](https://www.rust-lang.org/tools/install) => Backend runtime
- [NodeJS](https://nodejs.org/en/download/) => Frontend runtime
- [Yarn](https://yarnpkg.com/getting-started/install) => Package manager
- [GitLeaks](https://github.com/gitleaks/gitleaks) => Git security
- [Rust - Clippy](https://rust-lang.github.io/rust-clippy/master/index.html) => Code security/quality
- [Rust - Rustfmt](https://rust-lang.github.io/rustfmt/) => Code formatting
- [Rust - cargo-llvm](https://github.com/taiki-e/cargo-llvm-cov) => Code coverage
- [Rust - bump](https://crates.io/crates/cargo-bump) => Bump version
- [Git](https://git-scm.com/downloads) => Version control (obviously)
- [Cocogitto](https://docs.cocogitto.io/) => Git hooks and conventional commit

### Create new rust lib

```bash
nx generate @monodon/rust:library <lib-name>
```

## Nx - Front

### Start the application

Run `npx nx serve front` to start the development server. Happy coding!

### Build for production

Run `npx nx build front` to build the application. The build artifacts are stored in the output directory (e.g. `dist/` or `build/`), ready to be deployed.

### Running tasks

To execute tasks with Nx use the following syntax:

```bash
npx nx <target> <project> <...options>
```

You can also run multiple targets:

```bash
npx nx run-many -t <target1> <target2>
```

..or add `-p` to filter specific projects

```bash
npx nx run-many -t <target1> <target2> -p <proj1> <proj2>
```

Targets can be defined in the `package.json` or `projects.json`. Learn more [in the docs](https://nx.dev/features/run-tasks).

### Set up CI

Nx comes with local caching already built-in (check your `nx.json`). On CI you might want to go a step further.

- [Set up remote caching](https://nx.dev/features/share-your-cache)
- [Set up task distribution across multiple machines](https://nx.dev/nx-cloud/features/distribute-task-execution)
- [Learn more how to setup CI](https://nx.dev/recipes/ci)

### Explore the project graph

Run `npx nx graph` to show the graph of the workspace.
It will show tasks that you can run with Nx.

- [Learn more about Exploring the Project Graph](https://nx.dev/core-features/explore-graph)
