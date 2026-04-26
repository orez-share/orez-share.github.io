# Builder for keeping crossword lists in sync

## Usage

- Add new crosswords to the bottom of `crosswords.yaml`
- Run `cargo run` in this dir: this will render the templates from `templates` into the `gh-pages` dir.
- `gh-pages` is a git worktree. `cd ../gh-pages`, then add the `.ipuz` download file to `peapods/puz`
- Commit, and push
