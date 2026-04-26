# Builder for keeping crossword lists in sync

Builder script to apply the crosswords list from [`crosswords.yaml`](../crosswords.yaml) to the templates for the various crossword types, to generate their pages.

## Usage

- Add new crosswords to the bottom of `crosswords.yaml`
- Run `cargo run` in this dir: this will render the templates from [`templates`](../templates) into the [`gh-pages`](/gh-pages) dir.
- `gh-pages` is a git worktree. `cd ../../gh-pages`, then add the `.ipuz` download file to `crosswords/puz`
- Commit, and push
