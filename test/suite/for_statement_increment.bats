# bats file_tags=tag:for
skip
@test "for/statement_increment.lox" {
  run target/debug/lox test/cases/for/statement_increment.lox

}
