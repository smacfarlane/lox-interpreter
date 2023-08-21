# bats file_tags=tag:while
skip
@test "while/var_in_body.lox" {
  run target/debug/lox test/cases/while/var_in_body.lox

}
