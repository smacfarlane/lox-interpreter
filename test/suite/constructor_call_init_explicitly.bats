# bats file_tags=tag:constructor
skip
@test "constructor/call_init_explicitly.lox" {
  run target/debug/lox test/cases/constructor/call_init_explicitly.lox

  [ "${lines[0]}" = "Foo.init(one)" ]
  [ "${lines[1]}" = "Foo.init(two)" ]
  [ "${lines[2]}" = "Foo instance" ]
  [ "${lines[3]}" = "init" ]
}
