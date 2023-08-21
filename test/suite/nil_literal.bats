# bats file_tags=tag:nil
@test "nil/literal.lox" {
  run target/debug/lox test/cases/nil/literal.lox

  [ "${lines[0]}" = "nil" ]
}
