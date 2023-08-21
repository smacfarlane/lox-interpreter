# bats file_tags=tag:class
skip
@test "class/local_inherit_other.lox" {
  run target/debug/lox test/cases/class/local_inherit_other.lox

  [ "${lines[0]}" = "B" ]
}
