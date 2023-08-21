# bats file_tags=tag:variable
@test "variable/duplicate_local.lox" {
  run target/debug/lox test/cases/variable/duplicate_local.lox

}
