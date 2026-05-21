#!/usr/bin/env bash
# Verifies that values previously baked in at build time (level-filter,
# driver-version, clique-id, cluster-uuid) are now read at runtime via the
# /proc/self/maps-based discover_config_path() helper, and that the
# .so-relative location takes precedence over the /etc/dyll/options/ fallback.
#
# Run inside the libnvidia-ml-so docker image:
#   docker run --rm -v "$(pwd)/tests:/tests" libnvidia-ml-so:latest /tests/discover-config.sh

set -euo pipefail

# Layout the .so as <root>/usr/lib64/libnvidia-ml.so.1 so the
# `lib_dir.parent().parent()` walk in discover_config_path resolves to <root>.
ROOT=$(mktemp -d)
trap 'rm -rf "$ROOT"' EXIT

mkdir -p "$ROOT/usr/lib64" "$ROOT/dyll/options"
cp /workdir/libnvidia-ml.so.1 "$ROOT/usr/lib64/libnvidia-ml.so.1"

# Discovered (.so-relative) options -- these are the ones we expect to win.
echo 'DEBUG'                                  > "$ROOT/dyll/options/level-filter"
echo '999.999.99'                             > "$ROOT/dyll/options/driver-version"
echo '7777'                                   > "$ROOT/dyll/options/clique-id"
echo 'deadbeef-1234-5678-9abc-def012345678'   > "$ROOT/dyll/options/cluster-uuid"

# Fallback options -- different values so any failure to discover surfaces as
# a wrong value (or a suppressed log) rather than a silent pass.
mkdir -p /etc/dyll/options
echo 'ERROR'                                  > /etc/dyll/options/level-filter
echo '000.000.00'                             > /etc/dyll/options/driver-version
echo '1'                                      > /etc/dyll/options/clique-id
echo '00000000-0000-0000-0000-000000000000'   > /etc/dyll/options/cluster-uuid

OUTPUT=$(LD_LIBRARY_PATH="$ROOT/usr/lib64" /workdir/nvidia-smi -q 2>&1 || true)

fail() {
    echo "FAIL: $1" >&2
    echo "----- output -----" >&2
    echo "$OUTPUT" >&2
    exit 1
}

# level-filter: only the DEBUG-level "--- MOCK INIT ---" log appears if the
# discovered file beat the ERROR-level fallback.
grep -q -- '--- MOCK INIT ---' <<<"$OUTPUT" \
    || fail "level-filter: discovered DEBUG was not applied (init log missing)"

# driver-version: nvidia-smi -q prints "Driver Version : <version>".
grep -Eq 'Driver Version[[:space:]]*:[[:space:]]*999\.999\.99' <<<"$OUTPUT" \
    || fail "driver-version: nvidia-smi did not report the discovered value"

# clique-id: nvidia-smi -q prints "ClusterUUID/CliqueId" lines for fabric info.
grep -Eq 'CliqueId[[:space:]]*:[[:space:]]*7777' <<<"$OUTPUT" \
    || fail "clique-id: nvidia-smi did not report the discovered value"

# cluster-uuid: nvidia-smi -q prints "ClusterUUID : <uuid>". Match the
# canonical hyphenated form case-insensitively.
grep -Eqi 'ClusterUUID[[:space:]]*:[[:space:]]*deadbeef-1234-5678-9abc-def012345678' <<<"$OUTPUT" \
    || fail "cluster-uuid: nvidia-smi did not report the discovered value"

echo "PASS: level-filter, driver-version, clique-id, and cluster-uuid all resolved via /proc/self/maps"
