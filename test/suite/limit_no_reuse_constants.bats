# bats file_tags=tag:limit
@test "limit/no_reuse_constants.lox" {
  run target/debug/lox test/cases/limit/no_reuse_constants.lox

}
