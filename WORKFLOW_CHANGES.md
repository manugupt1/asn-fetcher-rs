# Workflow Changes for Issue #10

This document describes the changes needed to ensure crates.io publishing happens only after all CI workflows pass on the main branch.

## Problem

Currently, the publish workflow triggers immediately when a version tag is pushed, without waiting for CI checks to complete on the main branch. This could potentially publish broken code to crates.io.

## Solution

Change the publish workflow from a tag-push trigger to a workflow_run trigger that runs after the CI workflow completes successfully.

## How the New Workflow Works

1. Developer runs `./scripts/release.sh [patch|minor|major]`
2. Script creates a commit and tag, then pushes both to the main branch
3. **CI workflow runs first** (triggered by push to main)
4. **After CI completes successfully**, the publish workflow triggers
5. Publish workflow checks if the commit has a release tag
6. If a tag exists and CI passed, the workflow publishes to crates.io

## Changes Required

### 1. Update `.github/workflows/publish.yml`

Replace the entire file with the content shown in the issue comment, or apply these key changes:

**Change the trigger:**
```yaml
# OLD:
on:
  push:
    tags:
      - 'v*.*.*'

# NEW:
on:
  workflow_run:
    workflows: ["CI"]
    types:
      - completed
    branches:
      - main
```

**Add a check-tag job:**
```yaml
jobs:
  check-tag:
    name: Check for Release Tag
    runs-on: ubuntu-latest
    if: ${{ github.event.workflow_run.conclusion == 'success' }}
    outputs:
      has_tag: ${{ steps.check.outputs.has_tag }}
      tag_name: ${{ steps.check.outputs.tag_name }}
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Check if commit has release tag
        id: check
        run: |
          COMMIT_SHA="${{ github.event.workflow_run.head_sha }}"
          echo "Checking commit: $COMMIT_SHA"

          TAG=$(git tag --points-at "$COMMIT_SHA" | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$' || echo "")

          if [ -n "$TAG" ]; then
            echo "has_tag=true" >> $GITHUB_OUTPUT
            echo "tag_name=$TAG" >> $GITHUB_OUTPUT
            echo "Found release tag: $TAG"
          else
            echo "has_tag=false" >> $GITHUB_OUTPUT
            echo "No release tag found on this commit"
          fi
```

**Update existing jobs to depend on check-tag:**
- Add `needs: check-tag` to quality-checks job
- Add `if: ${{ needs.check-tag.outputs.has_tag == 'true' }}` to quality-checks job
- Update publish job to `needs: [check-tag, quality-checks]`
- Update checkout steps to use `ref: ${{ needs.check-tag.outputs.tag_name }}`

### 2. Update `CLAUDE.md` (Documentation)

Update the release section to reflect the new workflow sequence:

```markdown
The release script will:
1. Run quality checks (tests, clippy, format)
2. Update version in Cargo.toml
3. Create git commit and tag
4. Push to GitHub (triggers CI workflow on main)
5. After CI passes, publish workflow runs and publishes to crates.io

**Important**: The publish workflow only runs after all CI checks pass on the main branch. This ensures that releases are only published when the code is verified to be stable.
```

### 3. Update `scripts/release.sh` (Documentation)

Update the success message to reflect the new workflow:

```bash
info "Next steps:"
info "  1. Monitor GitHub Actions: https://github.com/manugupt1/asn-fetcher-rs/actions"
info "  2. The workflow sequence will be:"
info "     - CI workflow runs on main branch"
info "     - After CI passes, publish workflow runs"
info "     - Quality checks are executed"
info "     - Package is published to crates.io"
info "     - GitHub Release is created"
```

Also update the top comment:

```bash
# This script will:
# 1. Run quality checks (tests, clippy, format)
# 2. Bump the version in Cargo.toml
# 3. Create a git commit and tag
# 4. Push to GitHub (which triggers CI, then publish workflow after CI passes)
```

## Implementation Steps

Since the changes are prepared but couldn't be pushed due to GitHub App permissions:

1. **View the committed changes:**
   ```bash
   git fetch origin claude/issue-10-20260114-0238
   git checkout claude/issue-10-20260114-0238
   git show HEAD
   ```

2. **Create a new branch and apply changes:**
   ```bash
   git checkout main
   git pull
   git checkout -b fix/publish-after-ci
   git cherry-pick claude/issue-10-20260114-0238
   ```

3. **Push and create PR:**
   ```bash
   git push origin fix/publish-after-ci
   # Then create a PR from fix/publish-after-ci to main
   ```

Alternatively, you can manually copy the updated `publish.yml` content from the issue comment and apply it directly.

## Benefits

- Guarantees all CI checks pass before publishing to crates.io
- Prevents broken code from being published
- Maintains the same release script workflow for developers
- No changes needed to the release process itself

## Testing

After implementing the changes, test by:

1. Create a test release on a feature branch
2. Observe that CI runs first
3. Verify publish workflow only triggers after CI succeeds
4. Confirm that if CI fails, publish doesn't happen
