#!/usr/bin/env bash
# Publishes a GitHub release for an existing tag, attaching the shared object and the C bridge
# header built from that tag's own source rather than the working tree.
set -euo pipefail

TAG=""
DRY_RUN=0

_usage() {
	cat <<-EOF >&2
	Usage: $(basename "$0") <tag> [--dry-run]
	  <tag>      an existing tag to publish a release for
	  --dry-run  build and report, without touching GitHub
	EOF
	exit 1
}

_fail() {
	echo "github-release: $1" >&2
	exit 1
}

while [[ $# -gt 0 ]]; do
	case "$1" in
		--dry-run) DRY_RUN=1 ;;
		-h|--help) _usage ;;
		-*) echo "github-release: unknown option: $1" >&2; _usage ;;
		*) [[ -z "${TAG}" ]] || _fail "only one tag is accepted, got '${TAG}' and '$1'"; TAG="$1" ;;
	esac
	shift
done

[[ -n "${TAG}" ]] || _usage

for tool in git cmake cargo gh; do
	command -v "${tool}" >/dev/null 2>&1 \
		|| _fail "missing required tool '${tool}' — install it first (e.g. pacman -S git cmake rust github-cli on Arch)"
done

readonly SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

git -C "${REPO_ROOT}" rev-parse -q --verify "refs/tags/${TAG}" >/dev/null \
	|| _fail "tag '${TAG}' does not exist locally"

if ((DRY_RUN == 0)); then
	git -C "${REPO_ROOT}" ls-remote --exit-code --tags origin "refs/tags/${TAG}" >/dev/null 2>&1 \
		|| _fail "tag '${TAG}' is not on the remote — push it before publishing a release for it"
fi

workdir="$(mktemp -d)"
trap 'rm -rf "${workdir}"' EXIT

readonly SOURCE="${workdir}/source"
readonly PREFIX="${workdir}/prefix"
mkdir -p "${SOURCE}"
git -C "${REPO_ROOT}" archive --format=tar "${TAG}" | tar -x -C "${SOURCE}"

cmake -S "${SOURCE}" -B "${SOURCE}/build" \
	-DCMAKE_INSTALL_PREFIX="${PREFIX}" \
	-DCMAKE_BUILD_TYPE=Release >/dev/null
cmake --build "${SOURCE}/build" >/dev/null
cmake --install "${SOURCE}/build" >/dev/null

shared_object="$(find "${PREFIX}/lib" -maxdepth 1 -type f -name 'libomarchy_theme.so.*' -print -quit)"
[[ -n "${shared_object}" ]] || _fail "the build produced no shared object under ${PREFIX}/lib"

# The installed symlink pointing at the shared object is named after the SONAME the loader resolves,
# which is why the name is read from the link rather than from the binary's own header.
soname="$(find "${PREFIX}/lib" -maxdepth 1 -type l -lname "$(basename "${shared_object}")" -printf '%f' -quit)"
[[ -n "${soname}" ]] || _fail "the build produced no SONAME symlink under ${PREFIX}/lib"

header="${PREFIX}/include/omarchy_theme.h"
[[ -f "${header}" ]] || _fail "the build installed no C bridge header at ${header}"

notes="$(git -C "${REPO_ROOT}" tag -l --format='%(contents)' "${TAG}")
SONAME: ${soname}
Architecture: $(uname -m)"

if ((DRY_RUN == 1)); then
	echo "github-release: would publish ${TAG} with:"
	echo "  $(basename "${shared_object}")"
	echo "  $(basename "${header}")"
	echo "--- release notes ---"
	echo "${notes}"
	exit 0
fi

gh release create "${TAG}" \
	--repo "$(git -C "${REPO_ROOT}" remote get-url origin)" \
	--title "${TAG}" \
	--notes "${notes}" \
	"${shared_object}" \
	"${header}"

echo "github-release: published ${TAG}"
