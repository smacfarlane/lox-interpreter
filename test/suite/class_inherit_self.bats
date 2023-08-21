# bats file_tags=tag:class
skip
@test "class/inherit_self.lox" {
  run target/debug/lox test/cases/class/inherit_self.lox

}
