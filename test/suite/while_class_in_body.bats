# bats file_tags=tag:while
skip
@test "while/class_in_body.lox" {
  run target/debug/lox test/cases/while/class_in_body.lox

}
