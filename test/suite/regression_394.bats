# bats file_tags=tag:regression
skip
@test "regression/394.lox" {
  run target/debug/lox test/cases/regression/394.lox

  [ "${lines[0]}" = "B" ]
}
