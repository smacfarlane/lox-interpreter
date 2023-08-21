# bats file_tags=tag:variable
@test "variable/redeclare_global.lox" {
  run target/debug/lox test/cases/variable/redeclare_global.lox

  [ "${lines[0]}" = "nil" ]
}
