# bats file_tags=tag:assignment
@test "assignment/global.lox" {
  run target/debug/lox test/cases/assignment/global.lox

  [ "${lines[0]}" = "before" ]
  [ "${lines[1]}" = "after" ]
  [ "${lines[2]}" = "arg" ]
  [ "${lines[3]}" = "arg" ]
}
