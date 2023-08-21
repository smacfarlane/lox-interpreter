# bats file_tags=tag:method
skip
@test "method/missing_arguments.lox" {
  run target/debug/lox test/cases/method/missing_arguments.lox

}
