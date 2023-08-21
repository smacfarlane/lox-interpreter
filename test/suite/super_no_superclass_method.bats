# bats file_tags=tag:super
skip
@test "super/no_superclass_method.lox" {
  run target/debug/lox test/cases/super/no_superclass_method.lox

}
