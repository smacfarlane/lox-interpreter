# bats file_tags=tag:super
skip
@test "super/super_without_name.lox" {
  run target/debug/lox test/cases/super/super_without_name.lox

}
