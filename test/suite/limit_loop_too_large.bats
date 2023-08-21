# bats file_tags=tag:limit
skip
@test "limit/loop_too_large.lox" {
  run target/debug/lox test/cases/limit/loop_too_large.lox

}
