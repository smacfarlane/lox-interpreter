# bats file_tags=tag:if
skip
@test "if/else.lox" {
  run target/debug/lox test/cases/if/else.lox

  [ "${lines[0]}" = "good" ]
  [ "${lines[1]}" = "good" ]
  [ "${lines[2]}" = "block" ]
}
