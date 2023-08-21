# bats file_tags=tag:for
skip
@test "for/class_in_body.lox" {
  run target/debug/lox test/cases/for/class_in_body.lox

}
