# bats file_tags=tag:logical_operator
skip
@test "logical_operator/or.lox" {
  run target/debug/lox test/cases/logical_operator/or.lox

  [ "${lines[0]}" = "1" ]
  [ "${lines[1]}" = "1" ]
  [ "${lines[2]}" = "true" ]
  [ "${lines[3]}" = "false" ]
  [ "${lines[4]}" = "false" ]
  [ "${lines[5]}" = "false" ]
  [ "${lines[6]}" = "true" ]
}
