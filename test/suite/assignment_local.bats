# bats file_tags=tag:assignment
skip
@test "assignment/local.lox" {
  run target/debug/lox test/cases/assignment/local.lox

  [ "${lines[0]}" = "before" ]
  [ "${lines[1]}" = "after" ]
  [ "${lines[2]}" = "arg" ]
  [ "${lines[3]}" = "arg" ]
}
