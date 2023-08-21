# bats file_tags=tag:operator
skip
@test "operator/equals.lox" {
  run target/debug/lox test/cases/operator/equals.lox

  [ "${lines[0]}" = "true" ]
  [ "${lines[1]}" = "true" ]
  [ "${lines[2]}" = "false" ]
  [ "${lines[3]}" = "true" ]
  [ "${lines[4]}" = "false" ]
  [ "${lines[5]}" = "true" ]
  [ "${lines[6]}" = "false" ]
  [ "${lines[7]}" = "false" ]
  [ "${lines[8]}" = "false" ]
  [ "${lines[9]}" = "false" ]
}
