# bats file_tags=tag:constructor
skip
@test "constructor/return_in_nested_function.lox" {
  run target/debug/lox test/cases/constructor/return_in_nested_function.lox

  [ "${lines[0]}" = "bar" ]
  [ "${lines[1]}" = "Foo instance" ]
}
