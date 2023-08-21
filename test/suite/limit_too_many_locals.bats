# bats file_tags=tag:limit
@test "limit/too_many_locals.lox" {
  run target/debug/lox test/cases/limit/too_many_locals.lox

}
