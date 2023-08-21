# bats file_tags=tag:if
skip
@test "if/var_in_else.lox" {
  run target/debug/lox test/cases/if/var_in_else.lox

}
