# bats file_tags=tag:while
@test "while/return_closure.lox" {
  run target/debug/lox test/cases/while/return_closure.lox

  [ "${lines[0]}" = "i" ]
}
