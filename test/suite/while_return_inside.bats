# bats file_tags=tag:while
@test "while/return_inside.lox" {
  run target/debug/lox test/cases/while/return_inside.lox

  [ "${lines[0]}" = "i" ]
}
