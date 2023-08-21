# bats file_tags=tag:function
skip
@test "function/too_many_parameters.lox" {
  run target/debug/lox test/cases/function/too_many_parameters.lox

}
