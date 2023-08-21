# bats file_tags=tag:variable
@test "variable/use_global_in_initializer.lox" {
  run target/debug/lox test/cases/variable/use_global_in_initializer.lox

  [ "${lines[0]}" = "value" ]
}
