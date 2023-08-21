# bats file_tags=tag:super
skip
@test "super/closure.lox" {
  run target/debug/lox test/cases/super/closure.lox

  [ "${lines[0]}" = "Base" ]
}
