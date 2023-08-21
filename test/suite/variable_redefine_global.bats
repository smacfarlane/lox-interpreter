# bats file_tags=tag:variable
@test "variable/redefine_global.lox" {
  run target/debug/lox test/cases/variable/redefine_global.lox

  [ "${lines[0]}" = "2" ]
}
