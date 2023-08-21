# bats file_tags=tag:method
skip
@test "method/too_many_arguments.lox" {
  run target/debug/lox test/cases/method/too_many_arguments.lox

}
