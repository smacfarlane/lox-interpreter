# bats file_tags=tag:method
skip
@test "method/too_many_parameters.lox" {
  run target/debug/lox test/cases/method/too_many_parameters.lox

}
