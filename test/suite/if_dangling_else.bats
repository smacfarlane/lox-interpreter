# bats file_tags=tag:if
@test "if/dangling_else.lox" {
  run target/debug/lox test/cases/if/dangling_else.lox

  [ "${lines[0]}" = "good" ]
}
