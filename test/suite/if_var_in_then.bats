# bats file_tags=tag:if
skip
@test "if/var_in_then.lox" {
  run target/debug/lox test/cases/if/var_in_then.lox

}
