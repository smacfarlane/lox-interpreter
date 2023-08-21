# bats file_tags=tag:while
skip
@test "while/fun_in_body.lox" {
  run target/debug/lox test/cases/while/fun_in_body.lox

}
