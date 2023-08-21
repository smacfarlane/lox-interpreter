# bats file_tags=tag:limit
@test "limit/stack_overflow.lox" {
  run target/debug/lox test/cases/limit/stack_overflow.lox

}
