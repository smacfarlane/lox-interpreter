# bats file_tags=tag:class
skip
@test "class/local_reference_self.lox" {
  run target/debug/lox test/cases/class/local_reference_self.lox

  [ "${lines[0]}" = "Foo" ]
}
