# bats file_tags=tag:operator
@test "operator/not_equals.lox" {
  run target/debug/lox test/cases/operator/not_equals.lox

  [ "${lines[0]}" = "false" ]
  [ "${lines[1]}" = "false" ]
  [ "${lines[2]}" = "true" ]
  [ "${lines[3]}" = "false" ]
  [ "${lines[4]}" = "true" ]
  [ "${lines[5]}" = "false" ]
  [ "${lines[6]}" = "true" ]
  [ "${lines[7]}" = "true" ]
  [ "${lines[8]}" = "true" ]
  [ "${lines[9]}" = "true" ]
}
