# bats file_tags=tag:assignment
@test "assignment/syntax.lox" {
  run target/debug/lox test/cases/assignment/syntax.lox

  [ "${lines[0]}" = "var" ]
  [ "${lines[1]}" = "var" ]
}
