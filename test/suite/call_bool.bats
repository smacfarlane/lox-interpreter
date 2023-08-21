# bats file_tags=tag:call
skip
@test "call/bool.lox" {
  run target/debug/lox test/cases/call/bool.lox

}
