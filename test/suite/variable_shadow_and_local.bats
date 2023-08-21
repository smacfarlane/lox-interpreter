# bats file_tags=tag:variable
@test "variable/shadow_and_local.lox" {
  run target/debug/lox test/cases/variable/shadow_and_local.lox

  [ "${lines[0]}" = "outer" ]
  [ "${lines[1]}" = "inner" ]
}
