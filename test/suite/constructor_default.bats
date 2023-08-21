# bats file_tags=tag:constructor
skip
@test "constructor/default.lox" {
  run target/debug/lox test/cases/constructor/default.lox

  [ "${lines[0]}" = "Foo instance" ]
}
