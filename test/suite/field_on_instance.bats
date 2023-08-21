# bats file_tags=tag:field
skip
@test "field/on_instance.lox" {
  run target/debug/lox test/cases/field/on_instance.lox

  [ "${lines[0]}" = "bar value" ]
  [ "${lines[1]}" = "baz value" ]
  [ "${lines[2]}" = "bar value" ]
  [ "${lines[3]}" = "baz value" ]
}
