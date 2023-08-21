# bats file_tags=tag:constructor
skip
@test "constructor/early_return.lox" {
  run target/debug/lox test/cases/constructor/early_return.lox

  [ "${lines[0]}" = "init" ]
  [ "${lines[1]}" = "Foo instance" ]
}
