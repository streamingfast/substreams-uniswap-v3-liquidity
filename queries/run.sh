#!/usr/bin/env bash

main() {
  while getopts "h" opt; do
    case $opt in
      h) usage && exit 0;;
      \?) usage_error "Invalid option: -$OPTARG";;
    esac
  done
  shift $((OPTIND-1))

  file="$1"; shift

  if [[ $file == "" ]]; then
    usage_error "parameter <file> is required"
  fi

  if [ -z $PG_DSN ]; then
    usage_error "PG_DSN environment variable is required"
  fi

  set -e

  envsubst -v "$(cat $file)" | check_vars

  # psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable
  host=$(printf $PG_DSN | grep -Eo "@[^:]*" | tr -d "@")
  port=$(printf $PG_DSN | grep -Eo ":[0-9]+" | tr -d ":")
  user=$(printf $PG_DSN | grep -Eo "//[^:]*" | tr -d "/")
  passsword=$(printf $PG_DSN | grep -Eo ":[^@]*@" | sed -E 's/^://' | sed -E 's/@$//')
  database=$(printf $PG_DSN | grep -Eo ":[0-9]+/[^\?]+" | sed -E 's|:[0-9]+/||')

  cat $file | envsubst | PG_PASSWORD="$password" psql -h "$host" -p $port -U $user -d $database
}

check_vars() {
  failed="false"
  banner_shown="false"

  while read v; do
    if [[ ! "${!v}" ]]; then
      if [[ "$banner_shown" == "false" ]]; then
        echo "File $file defines some environment variables that were not defined:"
        banner_shown="true"
      fi

      echo '-' "$v"
      failed="true"
    fi
  done

  if [[ $failed == "true" ]]; then
    exit 1
  fi
}

usage_error() {
  message="$1"
  exit_code="$2"

  echo "ERROR: $message"
  echo ""
  usage
  exit ${exit_code:-1}
}

usage() {
  echo "usage: run <file>"
  echo ""
  echo "Run a query file variable substituted against the database."
  echo "expect PG_DSN to be set to the same format used by the"
  echo "substreams-sink-postgres sink."
  echo ""
  echo "Options"
  echo "    -h          Display help about this script"
}

main "$@"