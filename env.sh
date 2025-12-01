export AOC_ROOT=$(dirname -- "$( readlink -f -- "$0"; )")
export AOC_KEY=$(cat $AOC_ROOT/api_key)
export AOC_USER_AGENT=$(cat $AOC_ROOT/user_agent)
export AOC_INPUT_DIR=${AOC_ROOT}/input