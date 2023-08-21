# bats file_tags=tag:variable
@test "variable/undefined_local.lox" {
  run target/debug/lox test/cases/variable/undefined_local.lox

}
