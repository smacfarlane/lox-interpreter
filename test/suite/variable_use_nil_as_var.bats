# bats file_tags=tag:variable
skip
@test "variable/use_nil_as_var.lox" {
  run target/debug/lox test/cases/variable/use_nil_as_var.lox

}
