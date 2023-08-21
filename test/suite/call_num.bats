# bats file_tags=tag:call
skip
@test "call/num.lox" {
  run target/debug/lox test/cases/call/num.lox

}
