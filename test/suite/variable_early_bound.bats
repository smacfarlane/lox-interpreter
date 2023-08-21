# bats file_tags=tag:variable
@test "variable/early_bound.lox" {
  run target/debug/lox test/cases/variable/early_bound.lox

  [ "${lines[0]}" = "outer" ]
  [ "${lines[1]}" = "outer" ]
}
