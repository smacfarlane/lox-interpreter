# bats file_tags=tag:return
skip
@test "return/in_method.lox" {
  run target/debug/lox test/cases/return/in_method.lox

  [ "${lines[0]}" = "ok" ]
}
