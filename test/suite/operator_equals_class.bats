# bats file_tags=tag:operator
skip
@test "operator/equals_class.lox" {
  run target/debug/lox test/cases/operator/equals_class.lox

  [ "${lines[0]}" = "true" ]
  [ "${lines[1]}" = "false" ]
  [ "${lines[2]}" = "false" ]
  [ "${lines[3]}" = "true" ]
  [ "${lines[4]}" = "false" ]
  [ "${lines[5]}" = "false" ]
  [ "${lines[6]}" = "false" ]
  [ "${lines[7]}" = "false" ]
}
