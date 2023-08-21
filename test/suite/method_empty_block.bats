# bats file_tags=tag:method
skip
@test "method/empty_block.lox" {
  run target/debug/lox test/cases/method/empty_block.lox

  [ "${lines[0]}" = "nil" ]
}
