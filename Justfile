set shell := ["bash", "-cu"]

default:
    @just --list

docker-image version:
    VERSION="{{version}}" ./.github/scripts/build_docker.sh

docker-image-push version:
    PUSH=true VERSION="{{version}}" ./.github/scripts/build_docker.sh

docker-image-push-to image version:
    PUSH=true IMAGE_NAME="{{image}}" VERSION="{{version}}" ./.github/scripts/build_docker.sh
