# bats file_tags=tag:inheritance
skip
@test "inheritance/constructor.lox" {
  run target/debug/lox test/cases/inheritance/constructor.lox

  [ "${lines[0]}" = "value" ]
}
