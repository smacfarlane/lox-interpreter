# bats file_tags=tag:assignment
@test "assignment/associativity.lox" {
  run target/debug/lox test/cases/assignment/associativity.lox

  [ "${lines[0]}" = "c" ]
  [ "${lines[1]}" = "c" ]
  [ "${lines[2]}" = "c" ]
}
