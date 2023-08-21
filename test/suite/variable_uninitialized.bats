# bats file_tags=tag:variable
@test "variable/uninitialized.lox" {
  run target/debug/lox test/cases/variable/uninitialized.lox

  [ "${lines[0]}" = "nil" ]
}
