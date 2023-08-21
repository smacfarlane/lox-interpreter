# bats file_tags=tag:assignment
skip
@test "assignment/to_this.lox" {
  run target/debug/lox test/cases/assignment/to_this.lox

}
