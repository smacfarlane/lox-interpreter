# bats file_tags=tag:call
skip
@test "call/object.lox" {
  run target/debug/lox test/cases/call/object.lox

}
