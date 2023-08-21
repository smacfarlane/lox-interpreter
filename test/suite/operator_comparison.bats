# bats file_tags=tag:operator
@test "operator/comparison.lox" {
  run target/debug/lox test/cases/operator/comparison.lox

  [ "${lines[0]}" = "true" ]
  [ "${lines[1]}" = "false" ]
  [ "${lines[2]}" = "false" ]
  [ "${lines[3]}" = "true" ]
  [ "${lines[4]}" = "true" ]
  [ "${lines[5]}" = "false" ]
  [ "${lines[6]}" = "false" ]
  [ "${lines[7]}" = "false" ]
  [ "${lines[8]}" = "true" ]
  [ "${lines[9]}" = "false" ]
  [ "${lines[10]}" = "true" ]
  [ "${lines[11]}" = "true" ]
  [ "${lines[12]}" = "false" ]
  [ "${lines[13]}" = "false" ]
  [ "${lines[14]}" = "false" ]
  [ "${lines[15]}" = "false" ]
  [ "${lines[16]}" = "true" ]
  [ "${lines[17]}" = "true" ]
  [ "${lines[18]}" = "true" ]
  [ "${lines[19]}" = "true" ]
}
