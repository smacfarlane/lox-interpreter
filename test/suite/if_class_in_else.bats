# bats file_tags=tag:if
skip
@test "if/class_in_else.lox" {
  run target/debug/lox test/cases/if/class_in_else.lox

}
