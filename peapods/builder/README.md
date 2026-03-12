# Builder for the `peapods` dir

- Add new peapods to the bottom of `peapods.yaml`
- Run `cargo run` in this dir: this will render the templates from `templates` into the `gh-pages` dir.
- `gh-pages` is a git worktree. `cd ../gh-pages`, then add the `.ipuz` download file to `peapods/puz`
- Commit, and push
