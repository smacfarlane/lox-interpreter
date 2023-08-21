# bats file_tags=tag:while
skip
@test "while/closure_in_body.lox" {
  run target/debug/lox test/cases/while/closure_in_body.lox

  [ "${lines[0]}" = "1" ]
  [ "${lines[1]}" = "2" ]
  [ "${lines[2]}" = "3" ]
}
