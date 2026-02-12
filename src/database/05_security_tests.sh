#!/bin/bash
# Security tests for webapp_user privileges
# This script verifies that webapp_user has appropriate restrictions
# Exit code 0 = all tests passed, non-zero = one or more tests failed

set -e  # Exit on first failure

# Database credentials
DB_USER="webapp_user"
DB_PASS="webapp_dev_password"
DB_NAME="webapp_db"
DB_HOST="localhost"
DB_PORT="3306"

# Test counter
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Function to run a test that should fail
test_should_fail() {
    local test_name="$1"
    local sql_command="$2"
    local expected_error="$3"

    TESTS_RUN=$((TESTS_RUN + 1))
    echo -n "Test $TESTS_RUN: $test_name... "

    # Run command and capture output
    output=$(mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASS" -e "$sql_command" 2>&1 || true)

    # Check if error occurred and contains expected message
    if echo "$output" | grep -q "$expected_error"; then
        echo -e "${GREEN}PASSED${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${RED}FAILED${NC}"
        echo "  Expected error containing: $expected_error"
        echo "  Got: $output"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# Function to run a test that should succeed
test_should_succeed() {
    local test_name="$1"
    local sql_command="$2"

    TESTS_RUN=$((TESTS_RUN + 1))
    echo -n "Test $TESTS_RUN: $test_name... "

    # Run command and capture output
    if mysql -h "$DB_HOST" -P "$DB_PORT" -u "$DB_USER" -p"$DB_PASS" -e "$sql_command" 2>&1 > /dev/null; then
        echo -e "${GREEN}PASSED${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${RED}FAILED${NC}"
        echo "  Expected command to succeed but it failed"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

echo "========================================="
echo "webapp_user Security Tests"
echo "========================================="
echo ""

# Test 1: webapp_user cannot CREATE TABLE
test_should_fail \
    "webapp_user cannot CREATE TABLE" \
    "CREATE TABLE $DB_NAME.test_security_table (id INT);" \
    "CREATE command denied"

# Test 2: webapp_user cannot query mysql.user system table
test_should_fail \
    "webapp_user cannot query mysql.user" \
    "SELECT User, Host FROM mysql.user;" \
    "SELECT command denied"

# Test 3: webapp_user cannot CREATE USER
test_should_fail \
    "webapp_user cannot CREATE USER" \
    "CREATE USER 'test_security_user'@'%' IDENTIFIED BY 'testpass';" \
    "CREATE USER"

# Test 4: webapp_user cannot DROP TABLE (even on webapp_db)
test_should_fail \
    "webapp_user cannot DROP TABLE" \
    "DROP TABLE IF EXISTS $DB_NAME.users;" \
    "DROP command denied"

# Test 5: webapp_user can SELECT from webapp_db.users
test_should_succeed \
    "webapp_user can SELECT from webapp_db.users" \
    "SELECT COUNT(*) FROM $DB_NAME.users;"

# Test 6: webapp_user can INSERT into webapp_db.users (duplicate username rejection test)
test_should_fail \
    "webapp_user gets duplicate key error for existing username" \
    "INSERT INTO $DB_NAME.users (username, password, created_at) VALUES ('testuser', 'hash123', NOW());" \
    "Duplicate entry"

echo ""
echo "========================================="
echo "Summary: $TESTS_PASSED/$TESTS_RUN tests passed"
echo "========================================="

# Exit with failure if any test failed
if [ $TESTS_FAILED -gt 0 ]; then
    echo -e "${RED}FAILED: $TESTS_FAILED test(s) failed${NC}"
    exit 1
else
    echo -e "${GREEN}SUCCESS: All security tests passed${NC}"
    exit 0
fi
