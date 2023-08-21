# bats file_tags=tag:for
skip
@test "for/statement_initializer.lox" {
  run target/debug/lox test/cases/for/statement_initializer.lox

}
