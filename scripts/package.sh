#!/bin/sh

meta='target/metadata.json'
name=$(jq -r '.unique_identifier' "$meta")
binary=$(jq -r '.binary_path' "$meta")

pkgdir='pkg'
appsdir="${pkgdir}/flash_storage/skel/BADGEVMS/APPS"

rm -rf "${pkgdir}"
mkdir -p "${appsdir}/${name}"
cp -f 'target/metadata.json' "${appsdir}/${name}.json"
cp -f "${binary}" "${appsdir}/${name}/${binary}"
(cd "${pkgdir}" && tar -czf - *) > "${name}.pkg.tgz"
rm -rf "${pkgdir}"
