# lab-flow

TODO: Explain the project

- [lab-flow](#lab-flow)
  - [1. Feature](#1-feature)
    - [1.1. `git lab feature start [BRANCH_NAME]`](#11-git-lab-feature-start-branch_name)
    - [1.2. `git lab feature finish [BRANCH_NAME]`](#12-git-lab-feature-finish-branch_name)
  - [2. Bugfix](#2-bugfix)
    - [2.1. `git lab bugfix start [BRANCH_NAME]`](#21-git-lab-bugfix-start-branch_name)
    - [2.2. `git lab bugfix start [BRANCH_NAME] --release [RELEASE VERSION]`](#22-git-lab-bugfix-start-branch_name---release-release-version)
    - [2.3. `git lab bugfix finish [BRANCH_NAME]`](#23-git-lab-bugfix-finish-branch_name)

## 1. Feature

TODO: Explain what a feature is

### 1.1. `git lab feature start [BRANCH_NAME]`

TODO: Explain the git lab feature start command

```mermaid
sequenceDiagram
    actor X as Developer
    participant L as lab-flow
    participant G as git
    X ->>+ L: git lab feature start BRANCH_NAME
    alt With remote server
        L ->>+ G: try to update local develop
        G -->>- L: Updated
    else No remote server
        L ->>+ G: try to update local develop
        G -->>- L: ERROR: no remote server!
        L -) X: Show error on stderr
    end
    L ->>+ G: Create the feature/BRANCH_NAME over develop
    G -->>- L: feature/BRANCH_NAME created
    alt With remote server
        L ->>+ G: Try to push feature/BRANCH_NAME to remote
        G -->>- L: Branch pushed
    else No remote server
        L ->>+ G: Try to push feature/BRANCH_NAME to remote
        G -->>- L: ERROR: no remote server!
        L -) X: Show error on stderr
    end
    L ->>+ G: Checkout the feature/BRANCH_NAME
    G -->>- L: feature/BRANCH_NAME checked out
    L -->>- X: Finished
```

### 1.2. `git lab feature finish [BRANCH_NAME]`

TODO: Explain the git lab feature finish command and that commits must have been done previously.

```mermaid
sequenceDiagram
    actor X as Developer
    participant L as lab-flow
    participant G as git
    X ->>+ L: git lab feature finish BRANCH_NAME
    alt With remote server
        L ->>+ G: push feature/BRANCH_NAME to the remote server
        G -->>- L: Branch pushed
    else No remote server
        L ->>+ G: push feature/BRANCH_NAME to the remote server
        G -->>- L: ERROR: no remote server!
        L -) X: Show error on stderr
    end
    alt With remote server
        L ->>+ G: Verify if a remote server exists
        G -->>- L: Remote server exists!
        L ->>+G: Open up the Git Lab page for the merge request
        note left of G: Asking to merge feature/BRANCH_NAME to the develop branch
        L ->>+ G: Remove the local feature/BRANCH_NAME branch
        G -->>- L: Local branch removed
        L ->>+ G: Pull the develop branch
        G -->>- L: Branch pulled
        L ->>+ G: Checkout the develop branch
        G -->>- L: Branch develop checked out 
    else No remote server
        L ->>+ G: Verify if a remote server exists
        G -->>- L: Remote server doesn't exist!
        L ->>+ G: Merge feature/BRANCH_NAME to develop
        G -->>- L: Branch merged in the develop branch
        L ->>+ G: Delete the feature/BRANCH_NAME branch
        G ->>- L: Branch deleted
        L ->>+ G: Checkout the develop branch
        G -->>- L: Branch develop checked out
    end
```

## 2. Bugfix

### 2.1. `git lab bugfix start [BRANCH_NAME]`

If a bugfix was found in the `develop` branch.

It is the same design of a feature branch but it uses the `bugfix/` prefix informed with `git lab init`.

### 2.2. `git lab bugfix start [BRANCH_NAME] --release [RELEASE VERSION]`

Some error found in the latest release. We need to update the informed release version. If the branch `release/[RELEASE_VERSION` don't exist, we give an error. If it does exist, we use it as the source of the new `bugfix/BRANCH_NAME`.

### 2.3. `git lab bugfix finish [BRANCH_NAME]`

If the branch started from a 
