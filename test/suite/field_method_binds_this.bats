# bats file_tags=tag:field
skip
@test "field/method_binds_this.lox" {
  run target/debug/lox test/cases/field/method_binds_this.lox

  [ "${lines[0]}" = "foo1" ]
  [ "${lines[1]}" = "1" ]
}
