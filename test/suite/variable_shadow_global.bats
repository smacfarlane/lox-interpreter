# bats file_tags=tag:variable
@test "variable/shadow_global.lox" {
  run target/debug/lox test/cases/variable/shadow_global.lox

  [ "${lines[0]}" = "shadow" ]
  [ "${lines[1]}" = "global" ]
}
