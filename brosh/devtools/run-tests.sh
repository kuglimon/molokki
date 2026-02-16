#!/usr/bin/env bash

# Simple test runner for GNU Bash test suite
# Usage: ./run-tests.sh [path-to-shell] [test-filter]

set -euo pipefail

# Configuration
SHELL_TO_TEST="${1:-../bash}"
FILTER="${2:-}"
TESTS_DIR="$(dirname "$0")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
SKIPPED=0
TOTAL=0

# Track failed tests for summary
FAILED_TESTS=()

# Build helper programs if needed
build_helpers() {
    local helpers=(recho zecho printenv)
    for helper in "${helpers[@]}"; do
        if [[ -f "${TESTS_DIR}/${helper}.c" && ! -x "${TESTS_DIR}/${helper}" ]]; then
            echo "Building ${helper}..."
            cc -o "${TESTS_DIR}/${helper}" "${TESTS_DIR}/${helper}.c" 2>/dev/null || true
        fi
    done
}

# Run a single test
run_test() {
    local test_name="$1"
    local test_file="${TESTS_DIR}/${test_name}.tests"
    local expected_file="${TESTS_DIR}/${test_name}.right"

    if [[ ! -f "$test_file" ]]; then
        return 1
    fi

    if [[ ! -f "$expected_file" ]]; then
        echo -e "  ${YELLOW}○ SKIP${NC} ${test_name} (no .right file)"
        ((SKIPPED++))
        return 0
    fi

    # Set up environment
    local output
    output=$(
        cd "$TESTS_DIR"
        PATH=".:$PATH" \
        THIS_SH="$SHELL_TO_TEST" \
        LC_ALL=C \
        LANG=C \
        TERM=dumb \
        "$SHELL_TO_TEST" "./${test_name}.tests" 2>&1
    ) || true

    # Compare output
    local expected
    expected=$(cat "$expected_file")

    if [[ "$output" == "$expected" ]]; then
        echo -e "  ${GREEN}✓ PASS${NC} ${test_name}"
        ((PASSED++))
    else
        echo -e "  ${RED}✗ FAIL${NC} ${test_name}"
        ((FAILED++))
        FAILED_TESTS+=("$test_name")
    fi
}

# Discover and run tests
main() {
    echo "Shell under test: $SHELL_TO_TEST"
    echo "Tests directory: $TESTS_DIR"
    echo

    # Verify shell exists
    if [[ ! -x "$SHELL_TO_TEST" ]]; then
        echo "Error: Shell not found or not executable: $SHELL_TO_TEST"
        exit 1
    fi

    # Build helpers
    build_helpers

    echo "Running tests..."
    echo

    # Find all .tests files
    for test_file in "${TESTS_DIR}"/*.tests; do
        [[ -f "$test_file" ]] || continue

        local test_name
        test_name=$(basename "$test_file" .tests)

        # Apply filter if specified
        if [[ -n "$FILTER" && ! "$test_name" =~ $FILTER ]]; then
            continue
        fi

        ((TOTAL++))
        run_test "$test_name"
    done

    # Print summary
    echo
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${GREEN}${PASSED}${NC}/${TOTAL} tests passed"

    if ((FAILED > 0)); then
        echo -e "${RED}${FAILED}${NC} tests failed"
    fi

    if ((SKIPPED > 0)); then
        echo -e "${YELLOW}${SKIPPED}${NC} tests skipped"
    fi

    # List failed tests
    if ((FAILED > 0)); then
        echo
        echo "Failed tests:"
        for t in "${FAILED_TESTS[@]}"; do
            echo "  - $t"
        done
    fi

    # Exit with appropriate code
    if ((FAILED > 0)); then
        exit 1
    fi
}

main "$@"
