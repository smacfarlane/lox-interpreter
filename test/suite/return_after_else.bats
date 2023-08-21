# bats file_tags=tag:return
@test "return/after_else.lox" {
  run target/debug/lox test/cases/return/after_else.lox

  [ "${lines[0]}" = "ok" ]
}
