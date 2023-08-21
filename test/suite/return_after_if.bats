# bats file_tags=tag:return
@test "return/after_if.lox" {
  run target/debug/lox test/cases/return/after_if.lox

  [ "${lines[0]}" = "ok" ]
}
