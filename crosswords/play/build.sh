set -euxo pipefail
shopt -s dotglob
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "${SCRIPT_DIR}"
PROJECT_DIR=$( git rev-parse --show-toplevel )
cd "${PROJECT_DIR}"

# Build the app
APP_DIR="../variant-crossword-player"
pushd "${APP_DIR}"
XWORD_BASE_PATH="/crosswords/play" npm run build
popd

# If the worktree doesn't exist for some reason, make it
if [ ! -d "gh-pages" ]; then
  git worktree add gh-pages gh-pages
fi

# Remove the old build from our site, copy over the new build
PAGE_PATH="${PROJECT_DIR}/gh-pages/crosswords/play/"
rm -rf "$PAGE_PATH"
mkdir -p "$PAGE_PATH"
cp -r "${APP_DIR}/build"/* "$PAGE_PATH"

# Also remove the test xwords
pushd "${PAGE_PATH}"
rm -rf "/puz"

# Commit, but don't push
git add .
date=`date '+%F %H:%M:%S'`
git commit -m "Publish /crosswords/play $date" || $(exit 0)

set +x
echo -e "\n\x1B[32m/crosswords/play built! Don't forget to \`git push\` the /gh-pages directory once you're satisfied with your changes\x1B[0m"
