# bats file_tags=tag:super
skip
@test "super/super_in_top_level_function.lox" {
  run target/debug/lox test/cases/super/super_in_top_level_function.lox

}
