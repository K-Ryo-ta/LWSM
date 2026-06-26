#!/usr/bin/env bash
set -euo pipefail

PLATFORMS="${PLATFORMS:-linux/amd64,linux/arm64}"
PUSH="${PUSH:-false}"

derive_image_name() {
  if [[ -n "${IMAGE_NAME:-}" ]]; then
    echo "${IMAGE_NAME}"
    return 0
  fi

  if [[ -n "${GITHUB_REPOSITORY:-}" ]]; then
    echo "ghcr.io/${GITHUB_REPOSITORY,,}"
    return 0
  fi

  local remote_url
  remote_url="$(git config --get remote.origin.url || true)"
  if [[ "${remote_url}" =~ github\.com[:/]([^/]+)/([^/.]+)(\.git)?$ ]]; then
    local owner="${BASH_REMATCH[1]}"
    local repo="${BASH_REMATCH[2]}"
    echo "ghcr.io/${owner,,}/${repo,,}"
    return 0
  fi

  return 1
}

IMAGE_NAME="$(derive_image_name)" || {
  echo "IMAGE_NAME を推定できません。IMAGE_NAME=ghcr.io/<owner>/<repo> を指定してください。" >&2
  exit 1
}

if [[ -z "${VERSION:-}" ]]; then
  echo "VERSION を指定してください（例: VERSION=0.1.0）。" >&2
  exit 1
fi

TITLE="${OCI_TITLE:-lwsm}"
DESCRIPTION="${OCI_DESCRIPTION:-Lightweight Word Search Manager CLI}"
LICENSES="${OCI_LICENSES:-MIT}"
AUTHORS="${OCI_AUTHORS:-K-Ryo-ta}"
SOURCE_URL="${OCI_SOURCE:-https://github.com/${GITHUB_REPOSITORY:-K-Ryo-ta/LWSM}}"
REVISION="${OCI_REVISION:-${GITHUB_SHA:-}}"
CREATED="${OCI_CREATED:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}"

push_flag=()
if [[ "${PUSH}" == "true" ]]; then
  push_flag+=(--push)
fi

docker buildx build \
  "${push_flag[@]}" \
  --platform "${PLATFORMS}" \
  -t "${IMAGE_NAME}:${VERSION}" \
  -t "${IMAGE_NAME}:latest" \
  --label "org.opencontainers.image.title=${TITLE}" \
  --label "org.opencontainers.image.description=${DESCRIPTION}" \
  --label "org.opencontainers.image.licenses=${LICENSES}" \
  --label "org.opencontainers.image.authors=${AUTHORS}" \
  --label "org.opencontainers.image.version=${VERSION}" \
  --label "org.opencontainers.image.source=${SOURCE_URL}" \
  --label "org.opencontainers.image.revision=${REVISION}" \
  --label "org.opencontainers.image.created=${CREATED}" \
  .
