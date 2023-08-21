# bats file_tags=tag:class
skip
@test "class/empty.lox" {
  run target/debug/lox test/cases/class/empty.lox

  [ "${lines[0]}" = "Foo" ]
}
