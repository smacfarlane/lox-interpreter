# bats file_tags=tag:return
@test "return/in_function.lox" {
  run target/debug/lox test/cases/return/in_function.lox

  [ "${lines[0]}" = "ok" ]
}
