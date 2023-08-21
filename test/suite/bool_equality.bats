# bats file_tags=tag:bool
@test "bool/equality.lox" {
  run target/debug/lox test/cases/bool/equality.lox

  [ "${lines[0]}" = "true" ]
  [ "${lines[1]}" = "false" ]
  [ "${lines[2]}" = "false" ]
  [ "${lines[3]}" = "true" ]
  [ "${lines[4]}" = "false" ]
  [ "${lines[5]}" = "false" ]
  [ "${lines[6]}" = "false" ]
  [ "${lines[7]}" = "false" ]
  [ "${lines[8]}" = "false" ]
  [ "${lines[9]}" = "false" ]
  [ "${lines[10]}" = "true" ]
  [ "${lines[11]}" = "true" ]
  [ "${lines[12]}" = "false" ]
  [ "${lines[13]}" = "true" ]
  [ "${lines[14]}" = "true" ]
  [ "${lines[15]}" = "true" ]
  [ "${lines[16]}" = "true" ]
  [ "${lines[17]}" = "true" ]
}
