# bats file_tags=tag:variable
@test "variable/scope_reuse_in_different_blocks.lox" {
  run target/debug/lox test/cases/variable/scope_reuse_in_different_blocks.lox

  [ "${lines[0]}" = "first" ]
  [ "${lines[1]}" = "second" ]
}
