# bats file_tags=tag:call
skip
@test "call/string.lox" {
  run target/debug/lox test/cases/call/string.lox

}
