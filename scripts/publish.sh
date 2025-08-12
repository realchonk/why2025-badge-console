#!/bin/sh

BADGEHUB='https://badge.why2025.org'

die() {
	printf 'error: %s\n' "$*" >&2
	exit 1
}

assert() {
	"$@" || die "assertion failed: $*"
}

meta='target/metadata.json'

[ -e .env ] && . ./.env

name=$(jq -r '.unique_identifier' "$meta")
version=$(jq -r '.version' "$meta")
binary=$(jq -r '.binary_path' "$meta")
[ -z "$BADGEHUB_API_TOKEN" ] && die "please set \$BADGEHUB_API_TOKEN"
assert [ "$name" ]
assert [ "$version" ]
assert [ -e "$binary" ]

# upload version.txt
post() {
	curl	-X POST							\
		-H "badgehub-api-token: ${BADGEHUB_API_TOKEN}"		\
		-F "file=@$2"						\
		"${BADGEHUB}/api/v3/projects/${name}/draft/files/$1"	\
	|| die "failed to upload $1"
}

post "$binary" "./$binary"
post version.txt ./target/version.txt

curl	-X PATCH					\
	-H "badgehub-api-token: ${BADGEHUB_API_TOKEN}"	\
	-H "Accept: application/json"			\
	-H "Content-Type: application/json"		\
	"${BADGEHUB}/api/v3/projects/${name}/publish"	\
	|| die "failed to publish project"

echo "${name} v${version} published."
