# bats file_tags=tag:super
skip
@test "super/super_at_top_level.lox" {
  run target/debug/lox test/cases/super/super_at_top_level.lox

}
