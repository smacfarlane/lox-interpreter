# bats file_tags=tag:variable
skip
@test "variable/use_this_as_var.lox" {
  run target/debug/lox test/cases/variable/use_this_as_var.lox

}
