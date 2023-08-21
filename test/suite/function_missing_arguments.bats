# bats file_tags=tag:function
skip
@test "function/missing_arguments.lox" {
  run target/debug/lox test/cases/function/missing_arguments.lox

}
