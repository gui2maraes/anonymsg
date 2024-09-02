set -eo pipefail


PROD_DIR="${PROD_DIR:=/anonymsg}"
REPLACE_FILE="${PROD_DIR}/${REPLACE_FILE:=replace.sh}"
if [[ -z "${PROD_HOST}" ]]; then
    echo >&2 "Environment variable 'PROD_HOST' must be defined for SSH operations."
    exit 1
fi

cargo build --release
scp target/release/anonymsg-server "${PROD_HOST}:${PROD_DIR}/"
ssh "${PROD_HOST}" "bash ${REPLACE_FILE}"
