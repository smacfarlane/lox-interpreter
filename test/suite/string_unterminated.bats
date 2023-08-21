# bats file_tags=tag:string
@test "string/unterminated.lox" {
  run target/debug/lox test/cases/string/unterminated.lox

}
