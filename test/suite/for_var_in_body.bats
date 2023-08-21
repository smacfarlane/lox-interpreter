# bats file_tags=tag:for
skip
@test "for/var_in_body.lox" {
  run target/debug/lox test/cases/for/var_in_body.lox

}
