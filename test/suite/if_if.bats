# bats file_tags=tag:if
skip
@test "if/if.lox" {
  run target/debug/lox test/cases/if/if.lox

  [ "${lines[0]}" = "good" ]
  [ "${lines[1]}" = "block" ]
  [ "${lines[2]}" = "true" ]
}
