# bats file_tags=tag:constructor
skip
@test "constructor/arguments.lox" {
  run target/debug/lox test/cases/constructor/arguments.lox

  [ "${lines[0]}" = "init" ]
  [ "${lines[1]}" = "1" ]
  [ "${lines[2]}" = "2" ]
}
