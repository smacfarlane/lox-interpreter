# bats file_tags=tag:function
skip
@test "function/missing_comma_in_parameters.lox" {
  run target/debug/lox test/cases/function/missing_comma_in_parameters.lox

}
