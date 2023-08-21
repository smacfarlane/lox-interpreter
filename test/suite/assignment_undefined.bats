# bats file_tags=tag:assignment
skip
@test "assignment/undefined.lox" {
  run target/debug/lox test/cases/assignment/undefined.lox

}
