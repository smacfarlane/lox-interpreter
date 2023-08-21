# bats file_tags=tag:variable
skip
@test "variable/use_false_as_var.lox" {
  run target/debug/lox test/cases/variable/use_false_as_var.lox

}
