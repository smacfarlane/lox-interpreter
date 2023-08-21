# bats file_tags=tag:variable
@test "variable/use_local_in_initializer.lox" {
  run target/debug/lox test/cases/variable/use_local_in_initializer.lox

}
