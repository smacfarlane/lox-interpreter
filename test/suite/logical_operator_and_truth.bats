# bats file_tags=tag:logical_operator
@test "logical_operator/and_truth.lox" {
  run target/debug/lox test/cases/logical_operator/and_truth.lox

  [ "${lines[0]}" = "false" ]
  [ "${lines[1]}" = "nil" ]
  [ "${lines[2]}" = "ok" ]
  [ "${lines[3]}" = "ok" ]
  [ "${lines[4]}" = "ok" ]
}
