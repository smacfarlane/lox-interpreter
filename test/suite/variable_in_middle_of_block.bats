# bats file_tags=tag:variable
@test "variable/in_middle_of_block.lox" {
  run target/debug/lox test/cases/variable/in_middle_of_block.lox

  [ "${lines[0]}" = "a" ]
  [ "${lines[1]}" = "a b" ]
  [ "${lines[2]}" = "a c" ]
  [ "${lines[3]}" = "a b d" ]
}
