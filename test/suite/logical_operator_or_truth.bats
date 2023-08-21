# bats file_tags=tag:logical_operator
@test "logical_operator/or_truth.lox" {
  run target/debug/lox test/cases/logical_operator/or_truth.lox

  [ "${lines[0]}" = "ok" ]
  [ "${lines[1]}" = "ok" ]
  [ "${lines[2]}" = "true" ]
  [ "${lines[3]}" = "0" ]
  [ "${lines[4]}" = "s" ]
}
