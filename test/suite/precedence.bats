# bats file_tags=tag:basic
@test "precedence.lox" {
  run target/debug/lox test/cases/precedence.lox

  [ "${lines[0]}" = "14" ]
  [ "${lines[1]}" = "8" ]
  [ "${lines[2]}" = "4" ]
  [ "${lines[3]}" = "0" ]
  [ "${lines[4]}" = "true" ]
  [ "${lines[5]}" = "true" ]
  [ "${lines[6]}" = "true" ]
  [ "${lines[7]}" = "true" ]
  [ "${lines[8]}" = "0" ]
  [ "${lines[9]}" = "0" ]
  [ "${lines[10]}" = "0" ]
  [ "${lines[11]}" = "0" ]
  [ "${lines[12]}" = "4" ]
}
