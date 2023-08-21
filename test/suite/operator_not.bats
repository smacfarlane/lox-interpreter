# bats file_tags=tag:operator
skip
@test "operator/not.lox" {
  run target/debug/lox test/cases/operator/not.lox

  [ "${lines[0]}" = "false" ]
  [ "${lines[1]}" = "true" ]
  [ "${lines[2]}" = "true" ]
  [ "${lines[3]}" = "false" ]
  [ "${lines[4]}" = "false" ]
  [ "${lines[5]}" = "true" ]
  [ "${lines[6]}" = "false" ]
  [ "${lines[7]}" = "false" ]
}
