# bats file_tags=tag:return
@test "return/after_while.lox" {
  run target/debug/lox test/cases/return/after_while.lox

  [ "${lines[0]}" = "ok" ]
}
