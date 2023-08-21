# bats file_tags=tag:variable
@test "variable/undefined_global.lox" {
  run target/debug/lox test/cases/variable/undefined_global.lox

}
