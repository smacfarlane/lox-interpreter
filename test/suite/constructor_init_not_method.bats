# bats file_tags=tag:constructor
skip
@test "constructor/init_not_method.lox" {
  run target/debug/lox test/cases/constructor/init_not_method.lox

  [ "${lines[0]}" = "not initializer" ]
}
