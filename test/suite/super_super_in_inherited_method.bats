# bats file_tags=tag:super
skip
@test "super/super_in_inherited_method.lox" {
  run target/debug/lox test/cases/super/super_in_inherited_method.lox

  [ "${lines[0]}" = "A" ]
}
