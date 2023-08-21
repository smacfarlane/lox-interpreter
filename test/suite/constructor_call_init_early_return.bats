# bats file_tags=tag:constructor
skip
@test "constructor/call_init_early_return.lox" {
  run target/debug/lox test/cases/constructor/call_init_early_return.lox

  [ "${lines[0]}" = "init" ]
  [ "${lines[1]}" = "init" ]
  [ "${lines[2]}" = "Foo instance" ]
}
