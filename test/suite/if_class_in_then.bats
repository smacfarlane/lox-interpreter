# bats file_tags=tag:if
skip
@test "if/class_in_then.lox" {
  run target/debug/lox test/cases/if/class_in_then.lox

}
