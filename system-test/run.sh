#!/bin/bash
set -eu

cd $(dirname "$0")

CACHE=/tmp/oj-cache

oj --version
cargo build --release

for file in src/bin/*.rs; do
    bin=$(echo "${file}" | sed 's/src\/bin\/\(.*\)\.rs/\1/')

    # // url: http://example.com
    url="$(sed -e 's/^ *\/\/ *url\: *\(https\?:\/\/.*\) */\1/ ; t ; d' "$file")"
    if [[ -z "${url}" ]]; then
        echo "${file} doesn't specify url"
        exit 1
    fi

    dir=${CACHE}/$(echo -n "${url}" | md5sum | sed 's/ .*//')
    mkdir -p "${dir}"

    if [[ ! -e "${dir}/test" ]]; then
        sleep 1
        oj download --system -s "${url}" -d "${dir}/test"
    fi

    oj test --mle 256 --tle 8 -c "./target/release/${bin}" -d "${dir}/test"
done
