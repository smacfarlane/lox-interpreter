# bats file_tags=tag:limit
@test "limit/too_many_upvalues.lox" {
  run target/debug/lox test/cases/limit/too_many_upvalues.lox

}
