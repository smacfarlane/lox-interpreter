# bats file_tags=tag:logical_operator
skip
@test "logical_operator/and.lox" {
  run target/debug/lox test/cases/logical_operator/and.lox

  [ "${lines[0]}" = "false" ]
  [ "${lines[1]}" = "1" ]
  [ "${lines[2]}" = "false" ]
  [ "${lines[3]}" = "true" ]
  [ "${lines[4]}" = "3" ]
  [ "${lines[5]}" = "true" ]
  [ "${lines[6]}" = "false" ]
}
