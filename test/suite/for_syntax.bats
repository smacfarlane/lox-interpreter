# bats file_tags=tag:for
@test "for/syntax.lox" {
  run target/debug/lox test/cases/for/syntax.lox

  [ "${lines[0]}" = "1" ]
  [ "${lines[1]}" = "2" ]
  [ "${lines[2]}" = "3" ]
  [ "${lines[3]}" = "0" ]
  [ "${lines[4]}" = "1" ]
  [ "${lines[5]}" = "2" ]
  [ "${lines[6]}" = "done" ]
  [ "${lines[7]}" = "0" ]
  [ "${lines[8]}" = "1" ]
  [ "${lines[9]}" = "0" ]
  [ "${lines[10]}" = "1" ]
  [ "${lines[11]}" = "2" ]
  [ "${lines[12]}" = "0" ]
  [ "${lines[13]}" = "1" ]
}
