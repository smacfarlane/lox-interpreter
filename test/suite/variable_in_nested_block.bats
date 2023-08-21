# bats file_tags=tag:variable
@test "variable/in_nested_block.lox" {
  run target/debug/lox test/cases/variable/in_nested_block.lox

  [ "${lines[0]}" = "outer" ]
}
