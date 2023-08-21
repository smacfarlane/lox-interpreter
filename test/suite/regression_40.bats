# bats file_tags=tag:regression
@test "regression/40.lox" {
  run target/debug/lox test/cases/regression/40.lox

  [ "${lines[0]}" = "false" ]
}
