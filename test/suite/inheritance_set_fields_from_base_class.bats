# bats file_tags=tag:inheritance
skip
@test "inheritance/set_fields_from_base_class.lox" {
  run target/debug/lox test/cases/inheritance/set_fields_from_base_class.lox

  [ "${lines[0]}" = "foo 1" ]
  [ "${lines[1]}" = "foo 2" ]
  [ "${lines[2]}" = "bar 1" ]
  [ "${lines[3]}" = "bar 2" ]
  [ "${lines[4]}" = "bar 1" ]
  [ "${lines[5]}" = "bar 2" ]
}
