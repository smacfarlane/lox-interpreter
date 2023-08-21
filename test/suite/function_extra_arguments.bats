# bats file_tags=tag:function
skip
@test "function/extra_arguments.lox" {
  run target/debug/lox test/cases/function/extra_arguments.lox

}
