# bats file_tags=tag:variable
@test "variable/shadow_local.lox" {
  run target/debug/lox test/cases/variable/shadow_local.lox

  [ "${lines[0]}" = "shadow" ]
  [ "${lines[1]}" = "local" ]
}
