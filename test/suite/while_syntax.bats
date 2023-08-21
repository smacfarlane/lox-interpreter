# bats file_tags=tag:while
@test "while/syntax.lox" {
  run target/debug/lox test/cases/while/syntax.lox

  [ "${lines[0]}" = "1" ]
  [ "${lines[1]}" = "2" ]
  [ "${lines[2]}" = "3" ]
  [ "${lines[3]}" = "0" ]
  [ "${lines[4]}" = "1" ]
  [ "${lines[5]}" = "2" ]
}
