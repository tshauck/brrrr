# Contributing to brrrr

- [Development Workflow](#development-workflow)
  - [Intermediate Updates](#intermediate-updates)
  - [Releasing](#releasing)

## Development Workflow

This section describes how to do intermediate feature-to-feature development, and how to make a release once a set of changes are ready.

### Intermediate Updates

Git commits messages follow Conventional Commits. In particular, the type of change in the commit should match the nature of the code changes. This is important because it is later used in the release process to calculate the part of the version update.

### Releasing

Once a set of commits are ready to be released, run:

```console
# omit --dry-run when doing this for real
$ cz bump --dry-run
```

This will bump the appropriate throughout the code, and make a commit and tag corresponding to the change.

Finally, push `main` and the newly-created tag to GitHub.

```console
# update tag w/ the appropriate the version
$ git push --atomic origin main refs/tags/v0.1.0
```
