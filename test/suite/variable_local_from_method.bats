# bats file_tags=tag:variable
skip
@test "variable/local_from_method.lox" {
  run target/debug/lox test/cases/variable/local_from_method.lox

  [ "${lines[0]}" = "variable" ]
}
